use std::collections::BTreeMap;
// TODO remove:
// use std::path::Path;

use rustc_serialize::json::Json;

use core::{Vec2, util};


struct Frame {
  texture: u16,
  cut_from: (u16, u16),
  size: (u16, u16),
  pin_to: (i32, i32)
}


struct Sprite {
  frames: Vec<Frame>
}


struct ModelData {
  sprite_data: BTreeMap<String, Sprite>,
  occupied_tiles: Vec<Vec2<u8>>
}


impl ModelData {
  pub fn load(path: &str) -> Option<ModelData> {
    let obj;
    match ModelData::load_json(path) {
      Some(_obj) => obj = _obj,
      None => return None
    };

    match obj.get("name") {
      Some(name) => println!("Loading model {}...", name),
      None => return None
    };

    let sprite_data;
    match obj.get("sprite_data") {
      Some(&Json::Array(ref raw_sprite_data)) =>
        sprite_data = ModelData::parse_sprite_data(raw_sprite_data),
      _ => return None
    };

    let occupied_tile_data;
    match obj.get("occupied_tiles") {
      Some(&Json::Array(ref raw_occupied_tile_data)) =>
        occupied_tile_data =
          ModelData::parse_occupied_tile_data(raw_occupied_tile_data),
      _ => return None
    };

    Some(
      ModelData {
        sprite_data: BTreeMap::new(),
        occupied_tiles: Vec::new()
      }
    )
  }

  fn load_json(path: &str) -> Option<BTreeMap<String,Json>> {
    let file_contents;
    match util::read_file(path) {
      Ok(contents) => file_contents = contents,
      _ => return None
    };
    let json_obj;
    match Json::from_str(&file_contents) {
      Ok(json) => json_obj = json,
      _ => return None
    };
    match json_obj {
      Json::Object(data) => Some(data),
      _ => None
    }
  }

  fn parse_sprite_data(data: &Vec<Json>) -> BTreeMap<String, Sprite> {
    BTreeMap::new()
  }

  fn parse_occupied_tile_data(data: &Vec<Json>) -> Vec<Vec2<u8>> {
    Vec::new()
  }
}
