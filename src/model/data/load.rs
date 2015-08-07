use std::collections::BTreeMap;

use rustc_serialize::json::Json;

use core::{Vec2, util};

use super::{ModelData, Sprite};
use super::error::{LoadModelError, ModelError};
use super::sprite_data;


impl ModelData {
  pub fn load(path: &str) -> Result<ModelData, LoadModelError> {
    let obj = try!(ModelData::load_json(path));

    match obj.get("name") {
      Some(name) => println!("Loading model {}...", name),
      None =>
        return try!(Err(ModelError::MissingKey { key: "name",
                                                 context: "model data" }))
    };

    let sprite_data;
    match obj.get("sprite_data") {
      Some(&Json::Array(ref raw_sprite_data)) =>
        sprite_data = try!(sprite_data::parse(raw_sprite_data)),

      Some(_) => return Err(LoadModelError::ModelError(
                            ModelError::TypeError { obj: "key \"sprite_data\"",
                                                    expected: "array" })),

      None => return try!(Err(ModelError::MissingKey { key: "sprite_data",
                                                       context: "model data" }))
    };

    let occupied_tile_data;
    match obj.get("occupied_tiles") {
      Some(&Json::Array(ref raw_occupied_tile_data)) =>
        occupied_tile_data =
          try!( ModelData::parse_occupied_tile_data(raw_occupied_tile_data) ),

      Some(_) => return Err(LoadModelError::ModelError(
                         ModelError::TypeError { obj: "key \"occupied_tiles\"",
                                                 expected: "array" })),

      None =>
        return try!(Err(ModelError::MissingKey { key: "occupied_tiles",
                                                 context: "model data" }))
    };

    Ok(ModelData { sprite_data: sprite_data,
                   occupied_tiles: occupied_tile_data })
  }

  fn load_json(path: &str) -> Result<BTreeMap<String, Json>, LoadModelError> {
    let file_contents = try!(util::read_utf8_file(path));
    let json_obj = try!(Json::from_str(&file_contents));
    match json_obj {
      Json::Object(data) => Ok(data),
      _ => try!(Err( ModelError::TopLevelNotObject ))
    }
  }

  fn parse_occupied_tile_data(data: &Vec<Json>)
                                     -> Result<Vec<Vec2<u8>>, LoadModelError> {
    unimplemented!()
  }
}
