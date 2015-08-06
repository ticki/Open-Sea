use std::collections::BTreeMap;

use rustc_serialize::json::Json;

use core::{Vec2, util};

use super::{ModelData, Sprite, SpriteDataDefaults};
use super::error::{LoadModelError, ModelError};


impl ModelData {
  pub fn load(path: &str) -> Result<ModelData, LoadModelError> {
    let obj = try!(ModelData::load_json(path));

    match obj.get("name") {
      Some(name) => println!("Loading model {}...", name),
      None =>
        return Err(LoadModelError::ModelError(ModelError::MissingKey("name")))
    };

    let sprite_data;
    match obj.get("sprite_data") {
      Some(&Json::Array(ref raw_sprite_data)) =>
        sprite_data = ModelData::parse_sprite_data(raw_sprite_data,
                                                   SpriteDataDefaults::new() ),

      Some(_) => return Err(LoadModelError::ModelError(
                               ModelError::TypeError { key: "sprite_data",
                                                       expected: "array" })),

      None => return Err(LoadModelError::ModelError(
                                        ModelError::MissingKey("sprite_data")))
    };

    let occupied_tile_data;
    match obj.get("occupied_tiles") {
      Some(&Json::Array(ref raw_occupied_tile_data)) =>
        occupied_tile_data =
          ModelData::parse_occupied_tile_data(raw_occupied_tile_data),

      Some(_) => return Err(LoadModelError::ModelError(
                                ModelError::TypeError { key: "occupied_tiles",
                                                        expected: "array" })),

      None => return Err(LoadModelError::ModelError(
                                    ModelError::MissingKey("occupied_tiles")))
    };

    Ok(
      ModelData {
        sprite_data: BTreeMap::new(),
        occupied_tiles: Vec::new()
      }
    )
  }

  fn load_json(path: &str) -> Result<BTreeMap<String, Json>, LoadModelError> {
    let file_contents = try!(util::read_utf8_file(path));
    let json_obj = try!(Json::from_str(&file_contents));
    match json_obj {
      Json::Object(data) => Ok(data),
      _ => Err(LoadModelError::ModelError(
                         ModelError::Other("Model file must be JSON object") ))
    }
  }

  fn parse_sprite_data(
    data: &Vec<Json>,
    defaults: SpriteDataDefaults ) -> BTreeMap<String, Sprite> {

    for obj in data {
      match obj {
        &Json::Object(ref map) => {
          if map.contains_key("with") {
            if map.len() > 1 {
              //return Err()
            }
          }
        },
        _ => {} // return Err()
      }
    }
    BTreeMap::new()
  }

  fn parse_occupied_tile_data(data: &Vec<Json>) -> Vec<Vec2<u8>> {
    Vec::new()
  }
}
