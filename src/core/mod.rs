//! Traits, structs, and other objects for the game

// TODO: Make more use of modules here:
mod config;
//pub mod map; // TODO uncomment. squelching errors
//pub mod cam; // TODO uncomment. squelching errors.
mod direction;
//mod entity; // TODO uncomment. squelching errors.
pub mod matter;
//pub mod object; // TODO uncomment. squelching errors.
pub mod util;
// pub mod cache; // TODO uncomment. squelching errors

pub use self::config::Config;
//pub use self::map::{Map, Tile, TileMap}; // TODO uncomment. squelching errors
//pub use self::entity::{Entity, Id, Event}; // TODO uncomment. squelching errors.
pub use self::matter::{Matter};
pub use self::direction::Dir;
