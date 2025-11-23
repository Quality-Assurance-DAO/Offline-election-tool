# API Server and Algorithm Usage Guide

This guide provides comprehensive documentation for the REST API server, including:
- Starting the API server
- Using all three election algorithms (sequential-phragmen, parallel-phragmen, multi-phase)
- Constructing synthetic voter and candidate JSON data
- Edge case examples and validation rules
- Security considerations

**Last Updated**: 2025-01-27  
**Status**: Complete and up-to-date

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

## Constructing Synthetic Data

This section provides detailed guidance on constructing synthetic voter and candidate JSON data for various scenarios, including edge cases.

### Basic Structure

Synthetic data uses the `"type": "synthetic"` data source with the following structure:

```json
{
  "algorithm": "sequential-phragmen",
  "active_set_size": 2,
  "data_source": {
    "type": "synthetic",
    "candidates": [
      {
        "account_id": "string",
        "stake": "string"
      }
    ],
    "nominators": [
      {
        "account_id": "string",
        "stake": "string",
        "targets": ["string"]
      }
    ]
  }
}
```

### Field Requirements

**Candidates:**
- `account_id` (required): Account identifier as string. Can be any string format (SS58, hex, or custom). Does not need to exist on-chain.
- `stake` (required): Stake amount as **string** (to handle large numbers). Must be a valid non-negative integer.

**Nominators:**
- `account_id` (required): Account identifier as string. Must be unique among all nominators.
- `stake` (required): Stake amount as **string**. Must be a valid non-negative integer. Can be zero.
- `targets` (required): Array of candidate account IDs to vote for. Can be empty array `[]`. All target IDs must exist in the candidates list.

### Validation Rules

1. **Unique Account IDs**: All candidate and nominator account IDs must be unique within their respective arrays.
2. **Valid Targets**: All nominator `targets` must reference existing candidate `account_id` values.
3. **Stake Format**: Stake values must be valid non-negative integers (as strings).
4. **Minimum Requirements**: 
   - At least one candidate is required
   - At least one nominator is required (but can have zero stake)
   - `active_set_size` must not exceed the number of candidates

### Standard Examples

#### Example 1: Basic Election

```json
{
  "algorithm": "sequential-phragmen",
  "active_set_size": 2,
  "data_source": {
    "type": "synthetic",
    "candidates": [
      {"account_id": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "stake": "1000000000000"},
      {"account_id": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", "stake": "2000000000000"},
      {"account_id": "5FLSigC9HGRKVhB9F7BqHjXJxZJxZJxZJxZJxZJxZJxZJxZ", "stake": "1500000000000"}
    ],
    "nominators": [
      {
        "account_id": "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
        "stake": "500000000000",
        "targets": [
          "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
          "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
        ]
      },
      {
        "account_id": "5DbKjhNLpqX3HYq2b3tS1J3Z6sF7X8Y9Z0A1B2C3D4E5F6G7H8",
        "stake": "300000000000",
        "targets": ["5FLSigC9HGRKVhB9F7BqHjXJxZJxZJxZJxZJxZJxZJxZJxZ"]
      }
    ]
  }
}
```

#### Example 2: Using Hex Account IDs

```json
{
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
        "targets": [
          "0x1111111111111111111111111111111111111111111111111111111111111111",
          "0x2222222222222222222222222222222222222222222222222222222222222222"
        ]
      }
    ]
  }
}
```

### Edge Case Examples

#### Edge Case 1: Zero Stake Nominators

Nominators can have zero stake. This is valid but they won't contribute to any candidate's total stake.

```json
{
  "algorithm": "sequential-phragmen",
  "active_set_size": 2,
  "data_source": {
    "type": "synthetic",
    "candidates": [
      {"account_id": "candidate1", "stake": "1000000"},
      {"account_id": "candidate2", "stake": "2000000"}
    ],
    "nominators": [
      {
        "account_id": "nominator1",
        "stake": "0",
        "targets": ["candidate1"]
      },
      {
        "account_id": "nominator2",
        "stake": "500000",
        "targets": ["candidate2"]
      }
    ]
  }
}
```

**Note**: Zero stake candidates are **not allowed** - at least one candidate must have non-zero stake for the election to succeed.

#### Edge Case 2: Empty Voting Targets

Nominators can have empty `targets` arrays (not voting for anyone). The election will still succeed, with selection based on candidates' own stake.

```json
{
  "algorithm": "sequential-phragmen",
  "active_set_size": 2,
  "data_source": {
    "type": "synthetic",
    "candidates": [
      {"account_id": "candidate1", "stake": "1000000"},
      {"account_id": "candidate2", "stake": "2000000"},
      {"account_id": "candidate3", "stake": "1500000"}
    ],
    "nominators": [
      {
        "account_id": "nominator1",
        "stake": "500000",
        "targets": []
      },
      {
        "account_id": "nominator2",
        "stake": "300000",
        "targets": []
      }
    ]
  }
}
```

#### Edge Case 3: Single Candidate

Minimum valid election with one candidate and one nominator.

```json
{
  "algorithm": "sequential-phragmen",
  "active_set_size": 1,
  "data_source": {
    "type": "synthetic",
    "candidates": [
      {"account_id": "candidate1", "stake": "1000000"}
    ],
    "nominators": [
      {
        "account_id": "nominator1",
        "stake": "500000",
        "targets": ["candidate1"]
      }
    ]
  }
}
```

#### Edge Case 4: Single Nominator

Election with multiple candidates but only one nominator.

```json
{
  "algorithm": "sequential-phragmen",
  "active_set_size": 2,
  "data_source": {
    "type": "synthetic",
    "candidates": [
      {"account_id": "candidate1", "stake": "1000000"},
      {"account_id": "candidate2", "stake": "2000000"},
      {"account_id": "candidate3", "stake": "1500000"}
    ],
    "nominators": [
      {
        "account_id": "nominator1",
        "stake": "5000000",
        "targets": ["candidate1", "candidate2", "candidate3"]
      }
    ]
  }
}
```

#### Edge Case 5: Maximum Active Set Size

When `active_set_size` equals the number of candidates, all candidates are selected.

```json
{
  "algorithm": "sequential-phragmen",
  "active_set_size": 3,
  "data_source": {
    "type": "synthetic",
    "candidates": [
      {"account_id": "candidate1", "stake": "1000000"},
      {"account_id": "candidate2", "stake": "2000000"},
      {"account_id": "candidate3", "stake": "1500000"}
    ],
    "nominators": [
      {
        "account_id": "nominator1",
        "stake": "500000",
        "targets": ["candidate1", "candidate2", "candidate3"]
      }
    ]
  }
}
```

#### Edge Case 6: All Nominators Vote for All Candidates

Dense voting pattern where every nominator votes for every candidate.

```json
{
  "algorithm": "parallel-phragmen",
  "active_set_size": 2,
  "data_source": {
    "type": "synthetic",
    "candidates": [
      {"account_id": "candidate1", "stake": "1000000"},
      {"account_id": "candidate2", "stake": "2000000"}
    ],
    "nominators": [
      {
        "account_id": "nominator1",
        "stake": "500000",
        "targets": ["candidate1", "candidate2"]
      },
      {
        "account_id": "nominator2",
        "stake": "300000",
        "targets": ["candidate1", "candidate2"]
      },
      {
        "account_id": "nominator3",
        "stake": "200000",
        "targets": ["candidate1", "candidate2"]
      }
    ]
  }
}
```

#### Edge Case 7: Very Large Stake Values

Stake values can be extremely large (up to u128::MAX = 2^128 - 1). Use string format to avoid JSON number precision issues.

```json
{
  "algorithm": "sequential-phragmen",
  "active_set_size": 2,
  "data_source": {
    "type": "synthetic",
    "candidates": [
      {"account_id": "candidate1", "stake": "340282366920938463463374607431768211455"},
      {"account_id": "candidate2", "stake": "170141183460469231731687303715884105727"}
    ],
    "nominators": [
      {
        "account_id": "nominator1",
        "stake": "1000000000000000000000000000",
        "targets": ["candidate1", "candidate2"]
      }
    ]
  }
}
```

**Note**: Very large stake values may result in longer computation times.

#### Edge Case 8: Custom Account ID Formats

Account IDs don't need to be valid SS58 or hex - any string is acceptable for synthetic data.

```json
{
  "algorithm": "sequential-phragmen",
  "active_set_size": 2,
  "data_source": {
    "type": "synthetic",
    "candidates": [
      {"account_id": "validator-alice", "stake": "1000000"},
      {"account_id": "validator-bob", "stake": "2000000"}
    ],
    "nominators": [
      {
        "account_id": "nominator-charlie",
        "stake": "500000",
        "targets": ["validator-alice", "validator-bob"]
      }
    ]
  }
}
```

### Invalid Examples (Will Fail)

#### Invalid 1: Duplicate Account IDs

```json
{
  "candidates": [
    {"account_id": "candidate1", "stake": "1000000"},
    {"account_id": "candidate1", "stake": "2000000"}  // ❌ Duplicate ID
  ]
}
```

**Error**: `Duplicate candidate account ID: candidate1`

#### Invalid 2: Invalid Target Reference

```json
{
  "candidates": [
    {"account_id": "candidate1", "stake": "1000000"}
  ],
  "nominators": [
    {
      "account_id": "nominator1",
      "stake": "500000",
      "targets": ["nonexistent-candidate"]  // ❌ Target doesn't exist
    }
  ]
}
```

**Error**: `Voting edge references non-existent candidate: nonexistent-candidate`

#### Invalid 3: Negative Stake

```json
{
  "candidates": [
    {"account_id": "candidate1", "stake": "-1000000"}  // ❌ Negative stake
  ]
}
```

**Error**: `Invalid stake value: invalid digit found in string`

#### Invalid 4: Zero Candidates

```json
{
  "candidates": [],  // ❌ Empty candidates array
  "nominators": [
    {"account_id": "nominator1", "stake": "500000", "targets": []}
  ]
}
```

**Error**: `Election data must contain at least one candidate`

#### Invalid 5: Active Set Size Too Large

```json
{
  "algorithm": "sequential-phragmen",
  "active_set_size": 5,  // ❌ Only 2 candidates available
  "data_source": {
    "type": "synthetic",
    "candidates": [
      {"account_id": "candidate1", "stake": "1000000"},
      {"account_id": "candidate2", "stake": "2000000"}
    ],
    "nominators": [...]
  }
}
```

**Error**: `Requested 5 candidates but only 2 available`

### Best Practices

1. **Use String Format for Stakes**: Always use strings for stake values to avoid JSON number precision issues with large values.
2. **Unique Account IDs**: Ensure all account IDs are unique within candidates and nominators.
3. **Validate Targets**: Double-check that all nominator targets reference existing candidate account IDs.
4. **Test Edge Cases**: Test with zero stakes, empty targets, and single candidates/nominators to understand behavior.
5. **Account ID Format**: While any string works, using consistent formats (SS58, hex, or descriptive names) improves readability.

### Complete Example: Testing Edge Cases

```bash
# Test with zero stake nominator
curl -X POST http://localhost:3000/elections/run \
  -H "Content-Type: application/json" \
  -d '{
    "algorithm": "sequential-phragmen",
    "active_set_size": 2,
    "data_source": {
      "type": "synthetic",
      "candidates": [
        {"account_id": "candidate1", "stake": "1000000"},
        {"account_id": "candidate2", "stake": "2000000"}
      ],
      "nominators": [
        {
          "account_id": "nominator1",
          "stake": "0",
          "targets": ["candidate1"]
        },
        {
          "account_id": "nominator2",
          "stake": "500000",
          "targets": ["candidate2"]
        }
      ]
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

## Security Considerations

### ⚠️ Production Deployment Warning

The REST API server **does not include authentication, rate limiting, or request size limits** by default. These features must be added before deploying to production.

### Current Security Features

**Input Validation**:
- JSON structure validation (automatic via Serde)
- Algorithm type validation
- Account ID format validation (SS58 encoding)
- Stake value validation (numeric, non-negative)
- Data integrity checks (unique IDs, valid references)
- Election configuration validation

**Error Handling**:
- Malformed JSON returns `400 Bad Request` with error details
- Invalid input returns `400 Bad Request` with field-specific errors
- Internal errors return `500 Internal Server Error` without exposing sensitive details

### Required Security Measures for Production

**1. Authentication**

The API currently has no authentication. Implement one of:
- API key authentication via custom middleware
- Bearer token (JWT/OAuth2) authentication
- IP whitelisting (if applicable)
- Reverse proxy authentication (nginx, Traefik)

**2. Rate Limiting**

Protect against abuse:
- Per-IP rate limiting (e.g., 100 requests/minute)
- Per-endpoint limits (stricter on `/elections/run`)
- Per-API-key limits (if using API keys)

**3. Request Size Limits**

Protect against memory exhaustion:
- Set maximum JSON payload size (recommended: 10MB)
- Limit array sizes (candidates/nominators)
- Configure via Axum middleware: `DefaultBodyLimit::max()`

**4. Additional Recommendations**

- **HTTPS/TLS**: Always use HTTPS in production
- **CORS**: Configure CORS headers appropriately
- **SSRF Protection**: Validate RPC URLs (block internal IPs, use allowlists)
- **Resource Limits**: Set memory/CPU limits for the server process
- **Monitoring**: Implement logging and monitoring for suspicious activity
- **Reverse Proxy**: Deploy behind nginx/Traefik with additional security layers

### Example Secure Configuration

```bash
# Deploy behind reverse proxy with:
# - SSL/TLS termination
# - Rate limiting
# - Authentication
# - Request size limits
# - IP filtering

# Or implement middleware in the application:
# - API key validation
# - Rate limiting middleware
# - Request size limits
```

See the [Security and Robustness](../README.md#security-and-robustness) section in the main README for detailed recommendations.

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

