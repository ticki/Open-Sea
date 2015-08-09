//! Traits, structs, and other objects for the game

// TODO: Make more use of modules here:
mod angle;
mod config;
pub mod state;
mod map;
pub mod util;
mod vec2;
pub mod cam;
pub mod object;
mod prop;
mod entity;
mod matter;

pub use self::object::{Move, Sprite, Position, Dir};
pub use self::angle::Angle;
pub use self::config::Config;
pub use self::map::{Map, Tile, TileMap};
pub use self::vec2::Vec2;
pub use self::prop::{Prop};
pub use self::entity::{Entity, Id};
pub use self::matter::{Matter};

