use std::error::Error;

use rustc_serialize::{Decoder, json};

use super::util;


#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
  title: String,
  window_size: Vec<u32>
}


impl Config {
  pub fn load() -> Result<Config, <json::Decoder as Decoder>::Error> {
    json::decode(&util::read_file("./assets/config.json").unwrap())
  }

  pub fn game_title(&self) -> &String {
    &self.title
  }

  pub fn window_size(&self) -> [u32; 2] {
    [ self.window_size[0], self.window_size[1] ]
  }
}
