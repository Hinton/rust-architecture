# Rust Architecture

> A CLI tool for generating and maintaining architecture documentation in monorepos.

## Overview

Rust Architecture automatically extracts structured metadata from your project's markdown files and compiles them into comprehensive architecture documentation. Designed for monorepos with multiple packages, crates and modules. It helps keep your architecture documentation up-to-date and consistent.

## Features

- 📝 **Automatic Documentation Generation** - Extracts metadata from markdown with support for front matter
- 🏗️ **Category-based Organization** - Groups components by category
- 🔍 **Glob Pattern Support** - Use patterns like `**/README.md` to locate files
- 📦 **Monorepo-friendly** - Designed specifically for multi-crate/module/package projects

## Installation

```bash
cargo build --release
```

## Usage

### Generate Architecture Documentation

```bash
# Basic usage
./target/release/rust-architecture generate "**/README.md" ARCHITECTURE.md

# View help
./target/release/rust-architecture generate --help
```

### Command Arguments

- **Pattern**: Glob pattern to match markdown files (e.g., `**/README.md`)
- **Output**: Path for the generated architecture document (e.g., `ARCHITECTURE.md`)

## How It Works

1. **Scan**: Finds all markdown files matching your pattern
2. **Extract**: Reads front matter metadata (`description` and `category`), falling back to the first paragraph if `description` is missing
3. **Organize**: Groups components by category
4. **Generate**: Creates a structured architecture document

## Example

### Input Files

**`./crates/core/README.md`**

```markdown
---
description: "Core utilities for the project"
category: "Utilities"
---

# Core Module

...
```

**`./crates/cli/README.md`**

```markdown
---
category: "Interfaces"
---

# CLI Module

Command-line interface for the project

...
```

### Generated Output

**`ARCHITECTURE.md`**

```markdown
# Architecture Documentation

## Utilities

- `crates/core/README.md`: Core utilities for the project

## Interfaces

- `crates/cli/README.md`: Command-line interface for the project
```

## Front Matter Format

Your markdown files should include YAML front matter with these fields:

```yaml
---
description: "Brief description of this component"
category: "Category name (e.g., Utilities, Services, Interfaces)"
---
```

### Description Fallback

The `description` field is optional. If omitted, the tool will automatically extract the first paragraph after the title heading from your markdown content:

```markdown
---
category: "Utilities"
---

# Core Module

This paragraph becomes the description automatically.

More content here...
```

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run in development
cargo run -- generate "**/README.md" ARCHITECTURE.md
```
