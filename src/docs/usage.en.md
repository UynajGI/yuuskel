# Standardized Project Structure Guide

This project uses a standardized directory layout to unify the management of **code, configurations, inputs, outputs, and documentation**. It is suitable for a wide range of personal projects, including scripting tools, data analysis, web applications, and experimental prototypes.

## 📂 Directory Structure Overview

- **input/**: Raw input files (read-only, never modified)
- **output/**: Processed results, generated files, and intermediate artifacts
- **assets/**: Static resources (e.g., plots, images, audio, video)
  - `temp/`: Temporary debug assets (safe to delete at any time)
- **src/**: Core source code
- **scripts/**: Executable scripts (each script performs a single task)
- **configs/**: Configuration files (YAML, JSON, TOML, etc.)
- **docs/**: Project documentation, notes, and design materials
- **logs/**: Log files for tracking script executions, errors, and debug information
- **notebooks/**: Interactive exploration environments (e.g., Jupyter, Pluto)

## 🌐 Loading Environment Variables

Key paths are defined in the `.env` file at the project root. We recommend loading them using a dotenv library in your language of choice:

### Python

```python
from dotenv import load_dotenv
import os
load_dotenv()
output_dir = os.environ["OUTPUT_DIR"]
```

### Rust

```rust
use dotenvy::dotenv;
dotenv().ok();
let output_dir = std::env::var("OUTPUT_DIR").unwrap();
```

### Julia / Bash / Others

Refer to the dotenv documentation for your specific language or shell.

## 🔄 Recommended Workflow

1. Place raw files into `input/`
2. Write scripts → put them in `scripts/`
3. Scripts read from `input/` and write to `output/` or `assets/`
4. Always access paths via `.env` — **never hardcode them**
5. Write documentation in `docs/` for easy review and sharing

> ✅ This structure enables **fully reproducible workflows**, boosting both personal productivity and long-term maintainability.
