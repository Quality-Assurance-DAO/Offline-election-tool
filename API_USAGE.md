# API Server and Algorithm Usage Guide

This guide shows you how to start the REST API server and use all three election algorithms.

## Starting the API Server

### Basic Usage

```bash
# Start server on default port 3000
offline-election server

# Start server on custom port
offline-election server --port 8080
```

The server will start and display:
```
🚀 API server listening on http://0.0.0.0:3000
   POST   /elections/run
   GET    /elections/:id/results
   GET    /elections/:id/diagnostics
   GET    /health
```

### Health Check

Test if the server is running:
```bash
curl http://localhost:3000/health
```

Expected response: `OK`

---

## Using All Three Algorithms

### Via CLI

#### 1. Sequential Phragmen
```bash
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 100 \
  --rpc-url https://rpc.polkadot.io
```

#### 2. Parallel Phragmen
```bash
offline-election run \
  --algorithm parallel-phragmen \
  --active-set-size 100 \
  --rpc-url https://rpc.polkadot.io
```

#### 3. Multi-phase
```bash
offline-election run \
  --algorithm multi-phase \
  --active-set-size 100 \
  --rpc-url https://rpc.polkadot.io
```

#### Using Synthetic Data (All Algorithms)
```bash
# Sequential Phragmen with synthetic data
offline-election run \
  --algorithm sequential-phragmen \
  --active-set-size 2 \
  --synthetic

# Parallel Phragmen with synthetic data
offline-election run \
  --algorithm parallel-phragmen \
  --active-set-size 2 \
  --synthetic

# Multi-phase with synthetic data
offline-election run \
  --algorithm multi-phase \
  --active-set-size 2 \
  --synthetic
```

---

### Via REST API

#### 1. Sequential Phragmen with RPC Data

```bash
curl -X POST http://localhost:3000/elections/run \
  -H "Content-Type: application/json" \
  -d '{
    "algorithm": "sequential-phragmen",
    "active_set_size": 100,
    "data_source": {
      "type": "rpc",
      "url": "https://rpc.polkadot.io"
    }
  }'
```

#### 2. Parallel Phragmen with RPC Data

```bash
curl -X POST http://localhost:3000/elections/run \
  -H "Content-Type: application/json" \
  -d '{
    "algorithm": "parallel-phragmen",
    "active_set_size": 100,
    "data_source": {
      "type": "rpc",
      "url": "https://rpc.polkadot.io"
    }
  }'
```

#### 3. Multi-phase with RPC Data

```bash
curl -X POST http://localhost:3000/elections/run \
  -H "Content-Type: application/json" \
  -d '{
    "algorithm": "multi-phase",
    "active_set_size": 100,
    "data_source": {
      "type": "rpc",
      "url": "https://rpc.polkadot.io"
    }
  }'
```

#### 4. Using Synthetic Data (All Algorithms)

**Sequential Phragmen:**
```bash
curl -X POST http://localhost:3000/elections/run \
  -H "Content-Type: application/json" \
  -d '{
    "algorithm": "sequential-phragmen",
    "active_set_size": 2,
    "data_source": {
      "type": "synthetic",
      "candidates": [
        {"account_id": "0x1111111111111111111111111111111111111111111111111111111111111111", "stake": "1000000"},
        {"account_id": "0x2222222222222222222222222222222222222222222222222222222222222222", "stake": "2000000"},
        {"account_id": "0x3333333333333333333333333333333333333333333333333333333333333333", "stake": "1500000"}
      ],
      "nominators": [
        {
          "account_id": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
          "stake": "500000",
          "targets": [
            "0x1111111111111111111111111111111111111111111111111111111111111111",
            "0x2222222222222222222222222222222222222222222222222222222222222222"
          ]
        },
        {
          "account_id": "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
          "stake": "300000",
          "targets": ["0x3333333333333333333333333333333333333333333333333333333333333333"]
        }
      ]
    }
  }'
```

**Parallel Phragmen:**
```bash
curl -X POST http://localhost:3000/elections/run \
  -H "Content-Type: application/json" \
  -d '{
    "algorithm": "parallel-phragmen",
    "active_set_size": 2,
    "data_source": {
      "type": "synthetic",
      "candidates": [
        {"account_id": "0x1111111111111111111111111111111111111111111111111111111111111111", "stake": "1000000"},
        {"account_id": "0x2222222222222222222222222222222222222222222222222222222222222222", "stake": "2000000"}
      ],
      "nominators": [
        {
          "account_id": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
          "stake": "500000",
          "targets": ["0x1111111111111111111111111111111111111111111111111111111111111111"]
        }
      ]
    }
  }'
```

**Multi-phase:**
```bash
curl -X POST http://localhost:3000/elections/run \
  -H "Content-Type: application/json" \
  -d '{
    "algorithm": "multi-phase",
    "active_set_size": 2,
    "data_source": {
      "type": "synthetic",
      "candidates": [
        {"account_id": "0x1111111111111111111111111111111111111111111111111111111111111111", "stake": "1000000"},
        {"account_id": "0x2222222222222222222222222222222222222222222222222222222222222222", "stake": "2000000"}
      ],
      "nominators": [
        {
          "account_id": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
          "stake": "500000",
          "targets": ["0x1111111111111111111111111111111111111111111111111111111111111111"]
        }
      ]
    }
  }'
```

#### 5. Using JSON Data

```bash
curl -X POST http://localhost:3000/elections/run \
  -H "Content-Type: application/json" \
  -d '{
    "algorithm": "sequential-phragmen",
    "active_set_size": 2,
    "data_source": {
      "type": "json",
      "data": {
        "candidates": [
          {"account_id": "0x1111111111111111111111111111111111111111111111111111111111111111", "stake": 1000000},
          {"account_id": "0x2222222222222222222222222222222222222222222222222222222222222222", "stake": 2000000}
        ],
        "nominators": [
          {
            "account_id": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "stake": 500000,
            "targets": ["0x1111111111111111111111111111111111111111111111111111111111111111"]
          }
        ]
      }
    }
  }'
```

---

## Retrieving Results

After running an election, you'll receive a response with an `election_id`. Use it to retrieve results:

```bash
# Replace <election_id> with the ID from the response
curl http://localhost:3000/elections/<election_id>/results
```

Example response:
```json
{
  "election_id": "550e8400-e29b-41d4-a716-446655440000",
  "result": {
    "selected_validators": [...],
    "stake_distribution": [...],
    "total_stake": "800000",
    "algorithm_used": "sequential-phragmen",
    "execution_metadata": {...}
  },
  "execution_time_ms": 123
}
```

## Getting Diagnostics

```bash
curl http://localhost:3000/elections/<election_id>/diagnostics
```

---

## Comparing Algorithms

To compare results from different algorithms, run the same data through each:

```bash
# Run with sequential phragmen
SEQUENTIAL=$(curl -s -X POST http://localhost:3000/elections/run \
  -H "Content-Type: application/json" \
  -d '{"algorithm": "sequential-phragmen", "active_set_size": 2, "data_source": {...}}')

# Run with parallel phragmen
PARALLEL=$(curl -s -X POST http://localhost:3000/elections/run \
  -H "Content-Type: application/json" \
  -d '{"algorithm": "parallel-phragmen", "active_set_size": 2, "data_source": {...}}')

# Run with multi-phase
MULTI=$(curl -s -X POST http://localhost:3000/elections/run \
  -H "Content-Type: application/json" \
  -d '{"algorithm": "multi-phase", "active_set_size": 2, "data_source": {...}}')

# Compare the results
echo "$SEQUENTIAL" | jq '.result.selected_validators[].account_id'
echo "$PARALLEL" | jq '.result.selected_validators[].account_id'
echo "$MULTI" | jq '.result.selected_validators[].account_id'
```

---

## Example: Complete Workflow

```bash
# 1. Start the server (in one terminal)
offline-election server --port 3000

# 2. Run an election (in another terminal)
RESPONSE=$(curl -s -X POST http://localhost:3000/elections/run \
  -H "Content-Type: application/json" \
  -d '{
    "algorithm": "sequential-phragmen",
    "active_set_size": 2,
    "data_source": {
      "type": "synthetic",
      "candidates": [
        {"account_id": "0x1111111111111111111111111111111111111111111111111111111111111111", "stake": "1000000"},
        {"account_id": "0x2222222222222222222222222222222222222222222222222222222222222222", "stake": "2000000"}
      ],
      "nominators": [
        {
          "account_id": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
          "stake": "500000",
          "targets": ["0x1111111111111111111111111111111111111111111111111111111111111111"]
        }
      ]
    }
  }')

# 3. Extract election ID
ELECTION_ID=$(echo "$RESPONSE" | jq -r '.election_id')
echo "Election ID: $ELECTION_ID"

# 4. Get results
curl http://localhost:3000/elections/$ELECTION_ID/results | jq
```

---

## Troubleshooting

### Server won't start
- Check if port is already in use: `lsof -i :3000`
- Try a different port: `offline-election server --port 8080`

### API returns errors
- Check server logs for detailed error messages
- Verify JSON format is correct
- Ensure all required fields are present

### Algorithm comparison
- Use the same `data_source` for all three algorithms
- Use the same `active_set_size` for fair comparison
- Check `execution_time_ms` to compare performance

