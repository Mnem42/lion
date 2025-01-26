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

  lion-cli main.rs
  # Creates a file called main.rs
  lion-cli main.py
  # Creates a file called main.py
  lion-cli main.cpp
  # Creates a file called main.cpp
  lion-cli main.c
  # Creates a file called main.c
  lion-cli main.go
  # Creates a file called main.go
  lion-cli main.java
  # Creates a file called main.java
```


### Dependency adding:

```bash

  lion-cli dep serde
  # Adds serde as a dependency (only supported for rust and python so far)

  lion-cli main.rs serde
  lion-cli main.py serde
  # Adds serde as a dependency and creates the respective file

```
