//! Traits, structs, and other objects for the game

mod angle;
mod config;
mod game_view;
pub mod view;
mod map;
pub mod util;
mod vec2;
pub mod cam;
pub mod object;

pub use self::object::{Animate, Entity, Move, Sprite, Position, Dir, Id};
pub use self::angle::Angle;
pub use self::config::Config;
pub use self::game_view::GameView;
pub use self::view::View;
pub use self::map::{Prop, Map, Tile, TileMap};
pub use self::vec2::Vec2;
pub use opengl_graphics::*;


