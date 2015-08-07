use std::collections::BTreeMap;

use rustc_serialize::json::Json;

use super::Sprite;
use super::error::{LoadModelError, ModelError};


/// The `sprite_data` keys
const KEYS: &'static [&'static str] = &[
  "cut_from",
  "cut_offset",
  "file",
  "frame",
  "pin_offset",
  "pin_to",
  "size",
  "sprite",
];


/// This struct simplifies the implementation of `parse`
#[derive(Clone, Debug)]
struct SpriteDataDefaults {
  pub sprite: String,
  pub frame: u64,
  pub cut_from: (u16, u16),
  pub cut_offset: (i8, i8),
  pub size: (u16, u16),
  pub pin_to: (i32, i32),
  pub pin_offset: (i8, i8),
}


impl SpriteDataDefaults {
  pub fn new() -> SpriteDataDefaults {
    SpriteDataDefaults {
      sprite: "default".to_string(),
      frame: 0,
      cut_from: (0, 0),
      cut_offset: (0, 0),
      size: (1, 1),
      pin_to: (0, 0),
      pin_offset: (0, 0),
    }
  }
}


/// Process sprite data.
pub fn parse(data: &Vec<Json>)
                          -> Result<BTreeMap<String, Sprite>, LoadModelError> {

  let mut ret = BTreeMap::new();
  try!(parse_2(data, &SpriteDataDefaults::new(), &mut ret));
  Ok(ret)
}


/// Just the recursive version of `parse`.
///
/// This fills in `ret` with the processed data. It returns Ok(()) on success.
fn parse_2(data: &Vec<Json>,
           defaults: &SpriteDataDefaults,
           ret: &mut BTreeMap<String, Sprite> ) -> Result<(), LoadModelError> {

  for obj in data {
    match obj {
      &Json::Object(ref with_obj) => {
        if with_obj.contains_key("with") {
          if with_obj.len() > 1 {
            return try!(Err(ModelError::MultiKeyWith));
          }
          try!(process_with(with_obj, defaults, ret));
        }
      },
      _ => return try!(Err(
                   ModelError::TypeError { obj: "\"sprite_data\" list element",
                                           expected: "object" }))
    }
  }
  unimplemented!();
}


fn process_with(with_obj: &BTreeMap<String, Json>,
                defaults: &SpriteDataDefaults,
                ret: &mut BTreeMap<String, Sprite> )
                                                -> Result<(), LoadModelError> {

  let new_defaults = defaults.clone();
  for (key, val) in with_obj.iter() {
    match key as &str {
      "defaults" => {},
      "data" => {},
      other => try!(Err(ModelError::InvalidKey { key: other.to_string(),
                                                 context: "\"with\" object" }))
    };
  };
  Ok(())
}
