# Lion is an open-source CLI tool that makes developers lives easier
## Lion is a project ready for use but still under development
It helps by creating the file with some preset code and add external dependencies for you (implemented for rust and python)

## Installation

### Using Cargo (for Rust developers)
```bash
cargo install Lion-cli
```

### Easy Install (macOS and Linux)
```bash
curl -fsSL https://raw.githubusercontent.com/TeenCoder159/lion/main/install.sh | bash
```

### Easy Install (Windows)
PowerShell (Run as Administrator):
```powershell
Invoke-Expression (Invoke-WebRequest -Uri https://raw.githubusercontent.com/TeenCoder159/lion/main/install.ps1 -UseBasicParsing).Content
```

See the
[Docs](DOCS.md)
on how to use it and which languages are supported for which command:

- [x] Creates the file
- [x] Adds preset code
- [x] Allow dependency feature to be called individually
- [x] Add a compiling + running feature
- [x] Project creation with a preset file structure


- [ ] Linking of external libraries
- [ ] Add a lion settings file to get info about project
- [ ] Additional features coming soon...

## Current list of supported languages:
  - Rust
  - Python
  - C++
  - C
  - Go
  - Java
  - TypeScript
