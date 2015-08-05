use std::error::Error;
use std::fs::File;
use std::io::Read;

use rustc_serialize::{Decoder, json};


#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
  title: String,
  window_size: Vec<u32>
}


impl Config {
  pub fn load() -> Result<Config, <json::Decoder as Decoder>::Error> {
    let mut f = File::open("./assets/config.json").unwrap();
    let mut buffer = Vec::new();
    let _ = f.read_to_end(&mut buffer).unwrap();
    let s = String::from_utf8(buffer).unwrap();
    json::decode(&s)
  }

  pub fn game_title(&self) -> &String {
    &self.title
  }

  pub fn window_size(&self) -> [u32; 2] {
    [ self.window_size[0], self.window_size[1] ]
  }
}
