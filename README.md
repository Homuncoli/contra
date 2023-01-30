# Contra
Contra is a configuration file loader for Rust.

## Features
- [x] Load literals
- [x] Load collection of literals
- [x] Load nested structs
- [x] Load collection of structs
- [ ] \(Optional) Support multiple File Formats

## Usuage
Contra adds the derive macro: *Serialize*   which implements the *serialize* method for the given struct.
Contra adds the derive macro: *Deserialize* which implements the *deserialize* method for the given struct.
These functions are best used via the *Persistant* trait which automatically implemented for all struct that are both Serializable, and Deserializable.
The *Persistant trait* provides the functions *load* and *save*, which selects the appropiate serializer/deserializer based on the *path* given as parameter.

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