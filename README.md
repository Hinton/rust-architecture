# Rust Architecture

> A CLI tool for generating and maintaining architecture documentation in monorepos.

## Overview

Rust Architecture automatically extracts structured metadata from your project's markdown files and compiles them into comprehensive architecture documentation. Designed for monorepos with multiple packages, crates and modules. It helps keep your architecture documentation up-to-date and consistent.

## Features

- 📝 **Automatic Documentation Generation** - Extracts metadata from markdown front matter
- 🏗️ **Category-based Organization** - Groups components by category automatically
- 🔍 **Glob Pattern Support** - Use patterns like `**/README.md` to find files
- 🚀 **Fast & Lightweight** - Built with Rust for performance
- 📦 **Monorepo-friendly** - Designed specifically for multi-crate/module projects

## Installation

```bash
cargo build --release
```

## Usage

### Generate Architecture Documentation

```bash
# Basic usage
./target/release/rust-architecture generate **/README.md ARCHITECTURE.md

# View help
./target/release/rust-architecture generate --help
```

### Command Arguments

- **Pattern**: Glob pattern to match markdown files (e.g., `**/README.md`)
- **Output**: Path for the generated architecture document (e.g., `ARCHITECTURE.md`)

## How It Works

1. **Scan**: Finds all markdown files matching your pattern
2. **Extract**: Reads front matter metadata (`description` and `category`)
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
description: "Command-line interface for the project"
category: "Interfaces"
---

# CLI Module
...
```

### Generated Output

**`ARCHITECTURE.md`**
```markdown
# Architecture Documentation

## Utilities
- `crates/core`: **Core utilities for the project** (`crates/core/README.md`)

## Interfaces
- `crates/cli`: **Command-line interface for the project** (`crates/cli/README.md`)
```

## Front Matter Format

Your markdown files should include YAML front matter with these fields:

```yaml
---
description: "Brief description of this component"
category: "Category name (e.g., Utilities, Services, Interfaces)"
---
```

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run in development
cargo run -- generate **/README.md ARCHITECTURE.md
```

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.