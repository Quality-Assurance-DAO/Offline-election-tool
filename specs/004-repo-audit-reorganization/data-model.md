# Data Model: Repository Audit and Reorganization

**Feature**: Repository Audit and Reorganization  
**Date**: 2025-01-27  
**Phase**: Phase 1 - Design & Contracts

## Overview

This document defines the data models for the repository audit and reorganization feature. The entities represent documentation files, technical terms, documentation sections, and related concepts that need to be managed during the reorganization process.

## Entities

### DocumentationFile

Represents a markdown or text file containing project documentation.

**Attributes**:
- `file_path: String` - Current file path relative to repository root
- `target_path: Option<String>` - New file path after reorganization (None if file to be removed)
- `content: String` - File content (markdown text)
- `target_audience: AudienceType` - Primary audience (Newcomer, Contributor, Maintainer, Reference)
- `last_updated: Option<Date>` - Last modification date (if available from git history)
- `file_type: FileType` - Type of documentation file
- `relationships: Vec<FileRelationship>` - Relationships to other files (links, references)
- `duplicate_status: DuplicateStatus` - Whether file is duplicate, redundant, or unique
- `outdated_status: OutdatedStatus` - Whether file is outdated
- `unique_content: Option<String>` - Unique content to preserve if file is redundant

**Validation Rules**:
- `file_path` must be a valid relative path
- `target_path` must be None or a valid relative path within `docs/` or root
- `content` must not be empty if `target_path` is Some
- `duplicate_status` and `outdated_status` must be determined before reorganization

**State Transitions**:
- `Identified` → `Audited` (after content analysis)
- `Audited` → `Consolidated` (if duplicate/redundant, after content merge)
- `Audited` → `Moved` (if unique, after path update)
- `Consolidated` → `Removed` (after content preserved in target)
- `Moved` → `Updated` (after links updated)

**Relationships**:
- Many-to-many with `DocumentationFile` (via links)
- One-to-many with `TechnicalTerm` (terms used in file)
- One-to-many with `DocumentationSection` (sections within file)

### TechnicalTerm

Represents a domain-specific word or phrase that requires definition.

**Attributes**:
- `term_name: String` - The technical term (e.g., "Sequential Phragmen")
- `definition: String` - Definition of the term
- `first_occurrence_file: String` - File path where term first appears
- `first_occurrence_section: Option<String>` - Section within file where term first appears
- `cross_references: Vec<String>` - List of file paths that reference this term
- `term_type: TermType` - Category of term (Algorithm, Polkadot, Technical, Encoding)

**Validation Rules**:
- `term_name` must not be empty
- `definition` must not be empty
- `first_occurrence_file` must be a valid file path
- All files in `cross_references` must exist

**State Transitions**:
- `Identified` → `Defined` (after definition added)
- `Defined` → `Linked` (after cross-references updated)

**Relationships**:
- Many-to-many with `DocumentationFile` (files that use the term)

### DocumentationSection

Represents a logical grouping of related documentation content within a file.

**Attributes**:
- `section_title: String` - Title of the section
- `content: String` - Section content (markdown text)
- `parent_file: String` - File path containing this section
- `parent_section: Option<String>` - Parent section title (for nested sections)
- `child_sections: Vec<String>` - Child section titles (for nested sections)
- `cross_references: Vec<SectionReference>` - References to other sections
- `section_type: SectionType` - Type of section (Overview, Guide, Reference, Example)

**Validation Rules**:
- `section_title` must not be empty
- `content` must not be empty
- `parent_file` must be a valid file path
- `cross_references` must point to valid sections

**State Transitions**:
- `Created` → `Linked` (after cross-references added)
- `Linked` → `Moved` (if parent file moved, section moves with it)

**Relationships**:
- Many-to-one with `DocumentationFile` (belongs to file)
- Many-to-many with `DocumentationSection` (via cross-references)

### TestResultExample

Represents a documented example showing expected test output.

**Attributes**:
- `test_name: String` - Name of the test
- `expected_output_format: OutputFormat` - Format of output (JSON, HumanReadable)
- `example_output: String` - Example output text
- `interpretation_guide: String` - Guide explaining what the output means
- `success_criteria: Vec<String>` - List of criteria indicating success
- `parent_document: String` - Documentation file containing this example

**Validation Rules**:
- `test_name` must not be empty
- `example_output` must not be empty
- `interpretation_guide` must not be empty
- `success_criteria` must not be empty
- `parent_document` must be a valid file path

**State Transitions**:
- `Created` → `Documented` (after added to documentation)

**Relationships**:
- Many-to-one with `DocumentationFile` (belongs to documentation file)

### PolkadotEcosystemContext

Represents information explaining how this tool relates to Polkadot.

**Attributes**:
- `ecosystem_overview: String` - Overview of Polkadot ecosystem
- `tool_role: String` - Description of tool's role in ecosystem
- `dependencies: Vec<Dependency>` - List of Polkadot/Substrate dependencies
- `interaction_points: Vec<InteractionPoint>` - Points of interaction with Polkadot
- `parent_document: String` - Documentation file containing this context

**Validation Rules**:
- `ecosystem_overview` must not be empty
- `tool_role` must not be empty
- `dependencies` must include at least: sp-npos-elections, frame-election-provider-support
- `interaction_points` must include at least: RPC endpoints, on-chain elections

**State Transitions**:
- `Created` → `Documented` (after added to documentation)

**Relationships**:
- Many-to-one with `DocumentationFile` (belongs to documentation file)

### LinkReference

Represents a link between documentation files or sections.

**Attributes**:
- `source_file: String` - File containing the link
- `source_section: Option<String>` - Section containing the link
- `link_text: String` - Display text of the link
- `target_file: String` - Target file path (relative or absolute)
- `target_section: Option<String>` - Target section anchor
- `link_type: LinkType` - Type of link (Internal, External, Section)
- `status: LinkStatus` - Status of link (Valid, Broken, NeedsUpdate)

**Validation Rules**:
- `source_file` must be a valid file path
- `target_file` must be a valid path (relative or absolute URL)
- `link_type` must match the format of `target_file`
- `status` must be determined during audit

**State Transitions**:
- `Identified` → `Validated` (after target verified)
- `Validated` → `Updated` (if target path changed)
- `Broken` → `Fixed` (after target corrected)

**Relationships**:
- Many-to-one with `DocumentationFile` (source file)
- Many-to-one with `DocumentationFile` (target file, if internal)

## Enumerations

### AudienceType
- `Newcomer` - New users who need context and quick start
- `Contributor` - Developers who want to contribute code
- `Maintainer` - Project maintainers who need reference material
- `Reference` - Users who need specific technical information

### FileType
- `Readme` - Main entry point documentation
- `Guide` - User guide or tutorial
- `Api` - API documentation
- `Testing` - Testing documentation
- `Reference` - Reference documentation
- `Specification` - Feature specification

### DuplicateStatus
- `Unique` - File has unique content
- `Duplicate` - File is duplicate (>95% similarity with another file)
- `Redundant` - File is redundant (>60% overlap but has unique content)
- `Unknown` - Status not yet determined

### OutdatedStatus
- `Current` - File reflects current functionality
- `Outdated` - File references non-existent features OR (18+ months old AND contradicted)
- `Unknown` - Status not yet determined

### TermType
- `Algorithm` - Election algorithm name
- `Polkadot` - Polkadot ecosystem concept
- `Technical` - General technical term
- `Encoding` - Encoding format (e.g., SS58)

### SectionType
- `Overview` - High-level overview section
- `Guide` - Step-by-step guide section
- `Reference` - Reference material section
- `Example` - Example or tutorial section

### OutputFormat
- `JSON` - JSON format output
- `HumanReadable` - Human-readable text output

### LinkType
- `Internal` - Link to another file in repository
- `External` - Link to external resource (URL)
- `Section` - Link to section within same file

### LinkStatus
- `Valid` - Link target exists and is accessible
- `Broken` - Link target does not exist
- `NeedsUpdate` - Link target path needs updating after reorganization

## Relationships Summary

```
DocumentationFile
├── 1:N → DocumentationSection (file contains sections)
├── M:N → DocumentationFile (via links)
├── M:N → TechnicalTerm (files use terms)
└── 1:N → LinkReference (file contains links)

TechnicalTerm
└── M:N → DocumentationFile (terms used in files)

DocumentationSection
├── N:1 → DocumentationFile (belongs to file)
└── M:N → DocumentationSection (via cross-references)

TestResultExample
└── N:1 → DocumentationFile (belongs to documentation file)

PolkadotEcosystemContext
└── N:1 → DocumentationFile (belongs to documentation file)

LinkReference
├── N:1 → DocumentationFile (source file)
└── N:1 → DocumentationFile (target file, if internal)
```

## Validation Rules Summary

1. **File Path Validation**: All file paths must be valid relative paths within repository
2. **Content Preservation**: No content loss during consolidation (FR-015)
3. **Link Integrity**: All internal links must be updated before file removal (FR-012)
4. **Term Definition**: All technical terms must be defined where first used (FR-005)
5. **Structure Compliance**: Documentation structure must follow hybrid pattern (FR-004)

## State Transition Summary

### DocumentationFile Lifecycle
1. **Identified** → File discovered during audit
2. **Audited** → Content analyzed, duplicate/redundant status determined
3. **Consolidated** → Content merged into target file (if duplicate/redundant)
4. **Moved** → File moved to new location (if unique)
5. **Updated** → Links updated to new location
6. **Removed** → Old file removed (after consolidation)

### LinkReference Lifecycle
1. **Identified** → Link discovered during audit
2. **Validated** → Target verified to exist
3. **Updated** → Link path updated to new location
4. **Fixed** → Broken link corrected

