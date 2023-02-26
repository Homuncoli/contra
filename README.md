# Contra

[![Publish](https://github.com/Homuncoli/contra/actions/workflows/publish.yml/badge.svg?branch=master)](https://github.com/Homuncoli/contra/actions/workflows/publish.yml)
[![Version](https://github.com/Homuncoli/contra/actions/workflows/version.yml/badge.svg?branch=master)](https://github.com/Homuncoli/contra/actions/workflows/version.yml)
[![Lint, Build and Test](https://github.com/Homuncoli/contra/actions/workflows/build-test.yml/badge.svg?branch=master)](https://github.com/Homuncoli/contra/actions/workflows/build-test.yml)

Contra is a configuration file loader for Rust.

The serialization/deserialization is heavily "inspired" (if not blatantly copied) from the [serde](https://docs.rs/serde/latest/serde/) crate. Special thanks to 'Josh Mcguigan' and his very helpful article [Understanding-serde](https://www.joshmcguigan.com/blog/understanding-serde/).

## Features
- [x] Load and save literals
  - [x] string literals
  - [x] numeric literals
  - [x] enum literals
- [x] Load and save structs
  - [x] primitive structs
  - [x] nested structs
- [x] Load collections
  - [x] vectors
  - [x] maps
- [ ] Support multiple File Formats
  - [x] JSON
  - [ ] TOML
  - [ ] Cfg

## Usuage
Contra adds the derive macro: *Serialize*   which implements the *serialize* method for the given struct.
Contra adds the derive macro: *Deserialize* which implements the *deserialize* method for the given struct.
These functions are best used via the *Persistent* trait which automatically implemented for all struct that are both Serializable, and Deserializable.
The *Persistent trait* provides the functions *load* and *save*, which selects the appropiate serializer/deserializer based on the *path* given as parameter.

## Example
```rust
#[derive(Serialize, Deserialize, Default)]
pub struct Player {
    name: String,
    health: i32,
    dmg: f32,
    items: Vec<Item>,
}

#[derive(Deserialize, Deserialize)]
pub struct Item {
    name: String,
    slot: u32,
    stats: Vec<f32>,
}

fn main() {
    let saved_player = Player::default();
    player1::save("Player1.json").expect("failed to save player");

    let loaded_player = Player::load("Player1.json").expect("failed to load player");
}
```