# Lion Docs

## Installation:
### Install using cargo:
Prerequisites: Cargo and Rust

```bash
  cargo install Lion-cli
```

## Usage:


### File creation:
```bash

  lion-cli new main.rs
  # Creates a file called main.rs
  lion-cli new main.py
  # Creates a file called main.py
  lion-cli new main.cpp
  # Creates a file called main.cpp
  lion-cli new main.c
  # Creates a file called main.c
  lion-cli new main.go
  # Creates a file called main.go
  lion-cli new main.java
  # Creates a file called main.java
```


### Dependency adding:

```bash

  lion-cli dep main.rs serde
  lion-cli dep main.py serde
  # Adds serde as a dependency (only supported for rust and python)

  lion-cli new main.rs serde
  lion-cli new main.py serde
  # Adds serde as a dependency and creates the respective file

```
