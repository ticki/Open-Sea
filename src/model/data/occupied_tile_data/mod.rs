use std::collections::BTreeMap;

use rustc_serialize::json::Json;

use super::error::{LoadModelError, ModelError};
use core::Vec2;

mod individual;
mod rectangle;

/// Parse data about occupied tiles
pub fn parse(data: &Vec<Json>) -> Result<Vec<Vec2<i8>>, LoadModelError> {
  let mut ret = Vec::new();
  try!(parse_2(data, &mut ret));
  Ok(ret)
}


fn parse_2(data: &Vec<Json>,
           ret: &mut Vec<Vec2<i8>> ) -> Result<(), LoadModelError> {

  for obj in data {
    try!(parse_obj(obj, ret));
  }
  Ok(())
}


fn parse_obj(obj: &Json,
             ret: &mut Vec<Vec2<i8>> ) -> Result<(), LoadModelError> {

  match obj {
    &Json::Object(ref obj) => {
      try!(parse_obj_2(obj, ret));
    },
    _ => try!(Err(ModelError::TypeError { obj: "\"occupied_tiles\"",
                                          expected: "object" }))
  }
  Ok(())
}


fn parse_obj_2(obj: &BTreeMap<String, Json>,
               ret: &mut Vec<Vec2<i8>> ) -> Result<(), LoadModelError> {

  if obj.len() > 1 {
    return try!(Err(ModelError::WrongNumKeys
                       { expected: 1,
                         context: "\"occupied_tiles\" list element object" }));
  }
  for (key, val) in obj.iter() {
    match key as &str {
      "individual" => try!(individual::parse(val, ret)),
      "rectangle" => try!(rectangle::parse(val, ret)),
      other => try!(Err(
        ModelError::InvalidKey { key: other.to_string(),
                                 context:
                                   "\"occupied_tiles\" list element object" }))
    }
  }
  Ok(())
}
