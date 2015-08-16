use std::collections::BTreeMap;
use std::fmt::Debug;

use num::Num;

use rustc_serialize::json::Json;

use model::data::error::ModelError;
use super::SpriteDataSettings;

use math::Vec2;


pub fn modify(settings: &mut SpriteDataSettings,
              delta: &BTreeMap<String, Json> ) -> Result<(), ModelError> {

  for (key, val) in delta.iter() {
    match key as &str {
      "sprite" => settings.sprite_name = try!(get_sprite_name(val)),

      "frame" => settings.frame_index = try!(get_frame_index(val)),

      "cut_from" => settings.cut_from =
                try!(get_pair(val, "\"cut_from\"", "[u16, u16]", extract_u16)),

      "cut_offset" => settings.cut_offset =
                 try!(get_pair(val, "\"cut_offset\"", "[i8, i8]", extract_i8)),

      "size" => settings.size =
                    try!(get_pair(val, "\"size\"", "[u16, u16]", extract_u16)),

      "pin_to" => settings.pin_to =
                  try!(get_pair(val, "\"pin_to\"", "[i16, i16]", extract_i16)),

      "pin_offset" => settings.pin_offset =
                 try!(get_pair(val, "\"pin_offset\"", "[i8, i8]", extract_i8)),

      other => return Err(ModelError::InvalidKey
                                            { key: other.to_string(),
                                              context: "\"defaults\" object" })
    }
  }
  Ok(())
}


fn type_error<T>(obj: &'static str,
                 expected: &'static str ) -> Result<T, ModelError> {
  Err(ModelError::TypeError { obj: obj, expected: expected })
}


fn get_sprite_name(value: &Json) -> Result<String, ModelError> {
  match value {
    &Json::String(ref s) => Ok(s.clone()),
    _ => type_error("\"sprite\"", "string")
  }
}


fn get_frame_index(value: &Json) -> Result<usize, ModelError> {
  let type_err = type_error("\"frame\"", "u64");
  match value {
    &Json::I64(n) => {
      if n < 0 {
        return type_err;
      }
      Ok(n as usize)
    },
    &Json::U64(n) => Ok(n as usize),
    _ => return type_err
  }
}


fn get_pair<T, F>(value: &Json,
                  key: &'static str,
                  expected: &'static str,
                  extract: F ) -> Result<Vec2<T>, ModelError>
  where T: Copy + Debug + Num,
        F: Fn(&'static str, &'static str, &Json) -> Result<T, ModelError> {

  match value {
    &Json::Array(ref arr) => {
      if arr.len() != 2 {
        return type_error(key, expected);
      }

      let a = try!(extract(key, expected, &arr[0]));
      let b = try!(extract(key, expected, &arr[1]));
      Ok(Vec2(a, b))
    },
    _ => type_error(key, expected),
  }
}


// TODO: generalize these `extract...` methods (use num crate)

/// This is only to be used with `get_pair`
fn extract_i8(key: &'static str,
              expected: &'static str,
              obj: &Json ) -> Result<i8, ModelError> {

  match obj {
    // TODO check bounds
    &Json::I64(n) => Ok(n as i8),
    &Json::U64(n) => Ok(n as i8),
    _ => return type_error(key, expected)
  }
}


/// This is only to be used with `get_pair`
fn extract_i16(key: &'static str,
               expected: &'static str,
               obj: &Json ) -> Result<i16, ModelError> {

  match obj {
    // TODO check bounds
    &Json::I64(n) => Ok(n as i16),
    &Json::U64(n) => Ok(n as i16),
    _ => return type_error(key, expected)
  }
}


/// This is only to be used with `get_pair`
fn extract_u16(key: &'static str,
               expected: &'static str,
               obj: &Json ) -> Result<u16, ModelError> {

  match obj {
    &Json::I64(n) => {
      if n < 0 {
        return type_error(key, expected);
      }
      // TODO check `n` < 2**16
      Ok(n as u16)
    },
    // TODO check `n` < 2**16
    &Json::U64(n) => Ok(n as u16),
    _ => type_error(key, expected)
  }
}
