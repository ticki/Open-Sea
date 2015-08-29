use std::error::Error;

use rustc_serialize::{Decoder, json};

use super::util;

mod error;
pub use self::error::LoadConfigError;


const CONFIG_PATH: &'static str = "./data/config.json";


/// The configuration of the window
#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
    title: String,
    window_size: Vec<u32>
}


impl Config {
    pub fn load() -> Result<Config, LoadConfigError> {
        let config_contents = try!(util::read_utf8_file(CONFIG_PATH));
        // See self::error source for why try! isn't used here.
        match json::decode(&config_contents) {
            Ok(config) => Ok(config),
            Err(e) => Err(LoadConfigError::JsonError(e)),
        }
    }

    pub fn game_title(&self) -> &String {
        &self.title
    }

    pub fn window_size(&self) -> [u32; 2] {
        [ self.window_size[0], self.window_size[1] ]
    }
}
