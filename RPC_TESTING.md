# Testing RPC Endpoint Connection

The tool can now connect to live Substrate RPC endpoints! Here's how to test it.

## Quick Test

### Test Connection to Polkadot

```bash
# Test with latest block
cargo run -- run \
  --algorithm sequential-phragmen \
  --active-set-size 3 \
  --rpc-url https://rpc.polkadot.io

# Test with specific block number
cargo run -- run \
  --algorithm sequential-phragmen \
  --active-set-size 3 \
  --rpc-url https://rpc.polkadot.io \
  --block-number 20000000
```

### Test Connection to Kusama

```bash
cargo run -- run \
  --algorithm sequential-phragmen \
  --active-set-size 3 \
  --rpc-url https://kusama-rpc.polkadot.io
```

### Test Connection to Local Node

```bash
cargo run -- run \
  --algorithm sequential-phragmen \
  --active-set-size 3 \
  --rpc-url http://localhost:9933
```

## Current Status

✅ **Working:**
- RPC endpoint connection
- Block hash retrieval
- Latest block number detection
- Async runtime support

⚠️ **In Progress:**
- Storage key encoding (needs proper TwoX128 hashing)
- SCALE codec decoding for validators and nominators
- Storage queries for Staking::Validators, Staking::Nominators, Staking::Ledger

## What Happens When You Run

1. **Connection**: The tool successfully connects to the RPC endpoint
2. **Block Hash**: Retrieves the block hash for the specified (or latest) block
3. **Storage Query**: Attempts to query storage for validators and nominators
4. **Current Limitation**: Storage decoding is not yet fully implemented

## Expected Output

When you run the command, you'll see:

```
Error: RPC error: Could not fetch validators. This may require chain-specific storage key encoding.
Try using --input-file with JSON data instead, or check if the RPC endpoint supports state_queryStorageAt for Staking::Validators storage key.
Block hash: 0x... (URL: https://rpc.polkadot.io)
```

This confirms:
- ✅ RPC endpoint is reachable
- ✅ Connection is successful
- ✅ Block hash was retrieved
- ⚠️ Storage decoding needs implementation

## Next Steps for Full RPC Support

To complete RPC support, the following needs to be implemented:

1. **Storage Key Encoding**
   - Use `sp_core::twox_128` to hash pallet and storage item names
   - Encode: `twox128("Staking") + twox128("Validators")`
   - Similar for Nominators and Ledger

2. **SCALE Decoding**
   - Decode `Vec<AccountId>` for validators
   - Decode `BoundedVec<AccountId>` for nominator targets
   - Decode `StakingLedger` for nominator stakes

3. **Storage Queries**
   - Query `Staking::Validators()` for validator list
   - Query `Staking::Nominators()` for nominator targets
   - Query `Staking::Ledger()` for nominator stakes
   - Handle pagination for large datasets

## Alternative: Use JSON Input

Until storage decoding is fully implemented, you can:

1. Fetch data manually from RPC using tools like `substrate-api-client`
2. Convert to the JSON format expected by the tool
3. Use `--input-file` to run elections

## Public RPC Endpoints for Testing

- **Polkadot**: `https://rpc.polkadot.io`
- **Kusama**: `https://kusama-rpc.polkadot.io`
- **Westend**: `https://westend-rpc.polkadot.io`
- **Rococo**: `https://rococo-rpc.polkadot.io`

## Verification

To verify the connection is working, check that:
1. The tool connects without network errors
2. A block hash is retrieved (shown in error message)
3. The block number is correct (if specified)

The fact that you get a storage decoding error (not a connection error) confirms the RPC connection is successful!


