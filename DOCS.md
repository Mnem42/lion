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

  lion-cli dep main.cpp serde
  # Adds #include "serde/serde.h" to the top of main.cpp

  lion-cli new main.rs serde
  lion-cli new main.py serde
  # Adds serde as a dependency and creates the respective file

  # Requires Cargo for dependency linking in rust
  # Note: this doesn't link it (for python and cpp), it still requires you to create the CMake file

```

### Running code:

```bash

  lion-cli run main.cpp
  lion-cli run main.c
  lion-cli run main.py
  lion-cli run main.rs
  lion-cli run main.go
  lion-cli run main.java
  # Compiles and runs the code (requires respective compilers to be installed and setup)

```
