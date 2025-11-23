# Contract: Link Update Process

**Feature**: Repository Audit and Reorganization  
**Date**: 2025-01-27  
**Type**: Process Contract

## Overview

This contract defines the process for updating internal links during documentation reorganization. All internal links must be updated to point to new file locations before old files are removed.

## Link Types

### Internal File Links

**Pattern**: `[text](path/to/file.md)` or `[text](path/to/file.md#section)`

**Examples**:
- `[API Usage](API_USAGE.md)`
- `[Testing Guide](TESTING.md#quick-start)`
- `[Performance Benchmarks](PERFORMANCE_BENCHMARKS.md)`

**Update Rules**:
- Update path to new location in `docs/` structure
- Preserve section anchors if target section exists
- Update link text if file renamed significantly

### Section Links (Same File)

**Pattern**: `[text](#section-anchor)`

**Examples**:
- `[Quick Start](#quick-start)`
- `[API Endpoints](#api-endpoints)`

**Update Rules**:
- No change needed (stays within same file)
- Verify anchor still exists after content consolidation

### External Links

**Pattern**: `[text](https://example.com)` or `[text](http://example.com)`

**Examples**:
- `[Polkadot Documentation](https://wiki.polkadot.network)`
- `[Substrate Documentation](https://docs.substrate.io)`

**Update Rules**:
- No change needed (external links unchanged)
- Verify links still work
- Update if external resource moved

### Code References

**Pattern**: `[text](src/path/to/file.rs)` or `src/path/to/file.rs`

**Examples**:
- `src/algorithms/sequential_phragmen.rs`
- `[Algorithm Trait](src/algorithms/trait_def.rs)`

**Update Rules**:
- No change needed (code paths unchanged)
- Verify file still exists

## Link Update Process

### Phase 1: Audit

1. **Identify All Links**: Search all markdown files for link patterns
2. **Categorize Links**: Classify as internal, external, section, or code reference
3. **Create Link Inventory**: List all links with source file and target

### Phase 2: Mapping

1. **Create Path Mapping**: Map old paths to new paths
2. **Verify Targets**: Ensure all target files exist in new structure
3. **Handle Consolidations**: Map multiple source files to single target

### Phase 3: Update

1. **Update Internal Links**: Replace old paths with new paths
2. **Update Section Anchors**: Verify and update section anchors if needed
3. **Update Link Text**: Update text if file renamed significantly

### Phase 4: Verification

1. **Check All Links**: Verify all links point to valid targets
2. **Test Navigation**: Follow links to ensure they work
3. **Fix Broken Links**: Correct any broken links found

### Phase 5: Removal

1. **Remove Old Files**: Only after all links updated
2. **Final Verification**: Verify no broken links remain

## Link Mapping Table

| Old Link Target | New Link Target | Notes |
|----------------|-----------------|-------|
| `API_USAGE.md` | `docs/api/rest-api.md` or `docs/api/programmatic-api.md` | Split into two files |
| `ALGORITHM_EXTENSIBILITY.md` | `docs/guides/algorithms.md` | Renamed |
| `PERFORMANCE_BENCHMARKS.md` | `docs/guides/performance.md` | Renamed |
| `RPC_ARCHIVE_NODES.md` | `docs/guides/rpc-usage.md` | Consolidated |
| `RPC_TESTING.md` | `docs/guides/rpc-usage.md` | Consolidated |
| `TESTING.md` | `docs/testing/overview.md` | Consolidated |
| `TEST_RUNNING_INSTRUCTIONS.md` | `docs/testing/overview.md` | Consolidated |
| `RFP_COMPLIANCE_ASSESSMENT.md` | `docs/reference/rfp-compliance.md` | Renamed |

## Special Cases

### Split Files

**Case**: `API_USAGE.md` split into `rest-api.md` and `programmatic-api.md`

**Handling**:
- Links to REST API content → `docs/api/rest-api.md`
- Links to programmatic API content → `docs/api/programmatic-api.md`
- General API links → Update link text to specify which API

### Consolidated Files

**Case**: Multiple files consolidated into one (e.g., `TESTING.md` + `TEST_RUNNING_INSTRUCTIONS.md` → `overview.md`)

**Handling**:
- Links to either source file → `docs/testing/overview.md`
- Section anchors may need updating if sections merged
- Preserve section anchors that still exist

### Section Anchors

**Case**: Section anchors may change after consolidation

**Handling**:
- Verify anchor format: lowercase, hyphens for spaces
- Update anchors if section titles changed
- Test anchors work after update

## Validation Rules

1. **No Broken Links**: All internal links must point to existing files
2. **Correct Paths**: All paths must use correct relative paths from source file
3. **Preserved Anchors**: Section anchors must work if target section exists
4. **Updated Text**: Link text should reflect new file names if significantly changed

## Success Criteria

- ✅ All internal links updated to new paths
- ✅ No broken links remain
- ✅ Section anchors work correctly
- ✅ External links verified (still work)
- ✅ Code references unchanged (still valid)

## Tools and Commands

### Finding Links

```bash
# Find all markdown links
grep -r "\[.*\](.*\.md)" --include="*.md" .

# Find section links
grep -r "\[.*\](#.*)" --include="*.md" .
```

### Verifying Links

```bash
# Check if target files exist
find docs/ -name "*.md" -type f

# Verify relative paths from source file
# (manual check required)
```

## Example Updates

### Before
```markdown
See [API_USAGE.md](API_USAGE.md) for API documentation.
See [Testing Guide](TESTING.md#quick-start) for test instructions.
```

### After
```markdown
See [REST API documentation](docs/api/rest-api.md) and [Programmatic API](docs/api/programmatic-api.md) for API usage.
See [Testing Guide](docs/testing/overview.md#quick-start) for test instructions.
```

