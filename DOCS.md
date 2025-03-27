# Lion Docs

## Installation:
### Install using cargo:
Prerequisites: Cargo and Rust

```bash
  cargo install Lion-cli
```
#### Note the capital L in lion-cli

## Usage:

### With a lion.toml configuration File

If the file path in the Lion.toml file created is the file you want to run, then specifying the file repeatedly is not required:

```bash
Lion-cli run
```

Generating a lion.toml is as simple as doing:
```bash
Lion-cli init file/path
```


example Lion.toml:
```toml
[Project]
file_path = "src/main.rs"
file_ext = "rs"
```



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
  # Adds serde as a dependency using cargo

  lion-cli dep main.go https://github.com/user/serde.git
  lion-cli dep main.py https://github.com/user/serde.git # MUST BE A GIT URL
  # Adds serde as a dependency (only supported for rust and python)

  lion-cli dep main.cpp serde
  lion-cli dep main.c serde
  # Adds #include "serde/serde.h" to the top of main.cpp
  # Temporarily unavailable

  lion-cli new main.rs serde
  lion-cli new main.py https://github.com/user/serde.gits
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

### Making projects:
```bash

  lion-cli proj main.rs proj-name
  # Creates a directory called proj-name and creates a main.rs file in an src folder in proj-name
```

  C++ code requires g++
  C code requires gcc
  Python code requires python3
  Rust code requires rustc
  Go code requires go
  Java code requires JDK
