[![Crates.io](https://img.shields.io/crates/v/simple-game-utils)](https://crates.io/crates/simple-game-utils "Crates.io version")
[![Documentation](https://img.shields.io/docsrs/simple-game-utils)](https://docs.rs/simple-game-utils "Documentation")

# Game Utils

Simple game utilities 

## Usage

### Cargo

In your Cargo.toml file add

```
simple-game-utils = { version = "0.2.0", features = ["controller", "serde"] }
```

### Code

```
let controller = Controller::new();

controller
```

## Features

### controller

Very basic controller support

- no support for choosing controllers
- no support for detecting disconnections, etc

### serde

Adds serialization for some structs