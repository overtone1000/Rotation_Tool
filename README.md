# Rotation_Tool
Simple tool for using previous exam data to estimate proposed rotation volumes

# Development Environment
- Install rust https://rustup.rs/
- Install rust-analyzer extension for VS Code
- For debugging, install CodeLLDB
- Updates can be achieved. Check rust compiler version with `rustc --version` and update with `rustup update`
- Install CSV Editor extension for VS Code (makes dealing with categories easier)

# To Do
- `CoverageTree` handling of "All" creates many branches and is ineffecient.
  - Only creating branches if there is existing work would help, but would not cover real work poorly represented in the source data (Wet Reads are the best example)
  - May be better to have a key enum that includes "All" (or just specifically handle the "All" case differently) but this would require special functions for findings gaps and overlaps.
