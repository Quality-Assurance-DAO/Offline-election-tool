# RPC Snapshots and Historical/Archive Node Support

This document explains how the tool handles historical block queries, what archive nodes are, which RPC endpoints support historical queries, and how to troubleshoot issues with past-era simulations.

## Overview

The tool supports the `--block-number` flag to take snapshots at specific historical blocks. This allows you to:

- Simulate elections from past eras
- Compare election outcomes across different historical periods
- Validate election accuracy against known historical results
- Run regression tests using historical chain snapshots

**Important**: Querying historical blocks requires an **archive node** (also called a full node with archive mode enabled). Regular RPC endpoints typically only maintain recent state and cannot serve historical queries reliably.

## What Are Archive Nodes?

### Archive Nodes vs. Regular Nodes

**Regular RPC Nodes** (also called "pruned" nodes):
- Only maintain recent state (typically last ~256 blocks or a few days)
- Cannot query historical blocks beyond their retention period
- Faster and require less storage
- Most public RPC endpoints fall into this category

**Archive Nodes** (also called "full archive" nodes):
- Maintain complete historical state for all blocks
- Can query any block from genesis to present
- Require significantly more storage (hundreds of GB to TB)
- Slower queries due to larger database size
- Essential for historical block queries

### Why Archive Nodes Are Needed

When you specify `--block-number`, the tool:

1. Calls `chain_getBlockHash` with the block number to get the block hash
2. Uses `state_getStorage` with that block hash to query historical state
3. Retrieves validators, nominators, and stakes as they existed at that block

If the RPC endpoint doesn't have historical state, these queries will fail or return incorrect data.

## Supported RPC Endpoints

### Known Archive Node Providers

The following RPC endpoints are known to support historical queries (archive mode):

#### Polkadot
- ✅ **OnFinality Archive**: `https://polkadot.api.onfinality.io/public` (archive mode)
- ✅ **Dwellir Archive**: `https://polkadot-rpc.dwellir.com` (archive mode)
- ✅ **1RPC Archive**: `https://1rpc.io/dot` (may support archive queries)
- ⚠️ **Public RPC**: `https://rpc.polkadot.io` (limited historical support, typically last ~256 blocks)

#### Kusama
- ✅ **OnFinality Archive**: `https://kusama.api.onfinality.io/public` (archive mode)
- ✅ **Dwellir Archive**: `https://kusama-rpc.dwellir.com` (archive mode)
- ⚠️ **Public RPC**: `https://kusama-rpc.polkadot.io` (limited historical support)

#### Westend
- ✅ **OnFinality Archive**: `https://westend.api.onfinality.io/public` (archive mode)
- ⚠️ **Public RPC**: `https://westend-rpc.polkadot.io` (limited historical support)

### How to Verify Archive Node Support

To test if an RPC endpoint supports historical queries:

```bash
# Test with a block from several months ago
# Replace <BLOCK_NUMBER> with a block number from 3+ months ago
curl -X POST https://your-rpc-endpoint \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "chain_getBlockHash",
    "params": ["0x<BLOCK_NUMBER_HEX>"],
    "id": 1
  }'
```

If the endpoint returns a block hash (not null), it likely supports that historical block.

**Note**: Even if `chain_getBlockHash` succeeds, `state_getStorage` queries may still fail if the endpoint doesn't maintain full historical state. The tool will detect this and provide helpful error messages.

## How the Tool Handles Historical Queries

### RPC Methods Used

The tool uses the following Substrate RPC methods for historical queries:

1. **`chain_getBlockHash(block_number)`**
   - Retrieves the block hash for a given block number
   - Works on both regular and archive nodes (if block is within retention period)

2. **`state_getStorage(key, block_hash)`**
   - Queries storage state at a specific block hash
   - Requires archive node for historical blocks
   - Used to fetch:
     - `Session::Validators()` or `Staking::Validators()`
     - `Staking::Nominators()`
     - `Staking::Ledger()`

3. **`state_getKeys(prefix, block_hash)`** / **`state_getKeysPaged(...)`**
   - Retrieves all storage keys with a given prefix at a specific block
   - Used to enumerate all nominators and ledger entries
   - Requires archive node for historical blocks

### Error Handling

The tool includes comprehensive error handling for historical queries:

#### Block Hash Retrieval Failures

If `chain_getBlockHash` fails or times out:

```
Error: RPC error: Timeout after 30 seconds while getting block hash for block 12345678.
The RPC endpoint may be slow or unresponsive.
Please try:
- Using a different RPC endpoint
- Using --input-file with JSON data instead
- Checking your network connection
```

**What this means**: The endpoint may not support that historical block, or it's too slow/unresponsive.

**Solutions**:
- Try an archive node endpoint (see list above)
- Use a more recent block number
- Use `--input-file` with pre-fetched JSON data

#### Storage Query Failures

If `state_getStorage` returns null or fails:

```
Error: RPC error: Could not fetch validators from chain storage.
Tried Session::Validators() and Staking::Validators() storage keys.
Both returned null, which might indicate:
1. The storage keys are incorrect for this chain
2. The block hash is invalid
3. The RPC endpoint doesn't support these storage queries
Block hash: 0x...
Try using --input-file with JSON data instead.
```

**What this means**: The endpoint doesn't have historical state for that block, or the storage structure differs.

**Solutions**:
- Use an archive node endpoint
- Verify the block number is valid for that chain
- Use `--input-file` with pre-fetched JSON data

#### Timeout Errors

Historical queries can be slow. The tool uses timeouts:
- 30 seconds for block hash retrieval
- 30 seconds for validator queries
- 60 seconds for nominator queries (can be large datasets)

If timeouts occur:

```
Error: RPC error: Timeout after 60 seconds while fetching nominators.
Block hash: 0x...
This usually means the RPC endpoint doesn't support storage queries or is very slow.
Proceeding with zero nominators - election will use only validator self-stakes.
```

**What this means**: The query is taking too long, possibly because:
- The endpoint is slow or overloaded
- Historical queries are inherently slower
- The dataset is very large

**Solutions**:
- Try a different archive node endpoint
- Use `--input-file` with pre-fetched JSON data
- The tool will proceed with validators only (no nominator votes)

### Retry Logic

The tool automatically retries transient errors with exponential backoff:

- **Max retries**: 5 attempts
- **Initial delay**: 2 seconds
- **Max delay**: 30 seconds
- **Retryable errors**: HTTP 5xx, timeouts, network errors

If all retries fail, the tool suggests alternative endpoints and provides guidance.

## Best Practices

### 1. Use Archive Node Endpoints for Historical Queries

Always use archive node endpoints when querying historical blocks:

```bash
# ✅ Good: Using archive node
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://polkadot.api.onfinality.io/public \
  --block-number 10000000

# ⚠️ May fail: Using regular RPC (only works for recent blocks)
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://rpc.polkadot.io \
  --block-number 10000000
```

### 2. Pre-fetch and Cache Historical Data

For frequently used historical blocks, pre-fetch the data and save it as JSON:

```bash
# Fetch data once
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://polkadot.api.onfinality.io/public \
  --block-number 10000000 \
  --output-file snapshot_10000000.json

# Use cached data for subsequent runs (much faster)
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --input-file snapshot_10000000.json
```

### 3. Verify Block Numbers Are Valid

Before querying, verify the block number exists:

```bash
# Check latest block number
curl -X POST https://polkadot.api.onfinality.io/public \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}'

# Check if specific block exists
curl -X POST https://polkadot.api.onfinality.io/public \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"chain_getBlockHash","params":["0x989680"],"id":1}'
```

### 4. Handle Rate Limits

Archive node endpoints may have rate limits. If you encounter rate limiting:

- Add delays between requests
- Use multiple endpoints (rotate between them)
- Pre-fetch and cache data
- Contact the endpoint provider for higher limits

### 5. Test with Recent Blocks First

Before querying very old blocks, test with recent historical blocks:

```bash
# Test with block from 1 week ago
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://polkadot.api.onfinality.io/public \
  --block-number $(($(date +%s) / 6 - 10080))  # Approximate block from 1 week ago

# If that works, try older blocks
```

## Troubleshooting

### Problem: "Block hash retrieval failed"

**Possible causes**:
- Block number doesn't exist (too high)
- RPC endpoint doesn't support that historical block
- Network timeout

**Solutions**:
1. Verify block number is valid for that chain
2. Try an archive node endpoint
3. Use a more recent block number
4. Check network connectivity

### Problem: "Storage queries return null"

**Possible causes**:
- RPC endpoint doesn't maintain historical state
- Block hash is invalid
- Storage structure differs for that chain/era

**Solutions**:
1. Use an archive node endpoint
2. Verify block hash is correct
3. Check if chain had different storage structure at that era
4. Use `--input-file` with pre-fetched data

### Problem: "Query times out"

**Possible causes**:
- Endpoint is slow or overloaded
- Historical queries are inherently slower
- Very large dataset (many nominators)

**Solutions**:
1. Try a different archive node endpoint
2. Increase timeout (modify code if needed)
3. Pre-fetch data during off-peak hours
4. Use `--input-file` with cached data

### Problem: "No nominators found"

**Possible causes**:
- Nominator queries timed out (tool proceeds with validators only)
- No nominators existed at that block
- Storage query failed

**Solutions**:
1. Check if this is expected (early chain eras may have had fewer nominators)
2. Verify with a different endpoint
3. Check error messages for storage query failures

## Alternative: Using Pre-fetched JSON Data

If archive node access is unreliable or unavailable, you can:

1. **Fetch data once** using an archive node (or manual tools)
2. **Save as JSON** using the tool's output format
3. **Reuse the JSON** for all subsequent runs

This approach:
- ✅ Eliminates dependency on archive node availability
- ✅ Much faster (no network queries)
- ✅ Reproducible results
- ✅ Works offline

Example workflow:

```bash
# Step 1: Fetch historical data (do this once)
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://polkadot.api.onfinality.io/public \
  --block-number 10000000 \
  --output-file historical_snapshot.json

# Step 2: Use cached data for all subsequent runs
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --input-file historical_snapshot.json \
  --diagnostics
```

## Summary

- ✅ The tool supports `--block-number` for historical block queries
- ⚠️ Historical queries require **archive nodes** (not regular RPC endpoints)
- ✅ Known archive node providers are listed above
- ✅ The tool provides helpful error messages when archive nodes are unavailable
- ✅ Pre-fetching and caching historical data is recommended for reliability
- ✅ Alternative: Use `--input-file` with pre-fetched JSON data

For questions or issues with historical queries, check:
1. Are you using an archive node endpoint?
2. Is the block number valid for that chain?
3. Are there network/timeout issues?
4. Consider pre-fetching and caching data

