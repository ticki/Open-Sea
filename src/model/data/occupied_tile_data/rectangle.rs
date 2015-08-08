use std::collections::BTreeMap;

use rustc_serialize::json::Json;

use core::Vec2;

use model::data::error::{LoadModelError, ModelError};


pub fn parse(block: &Json,
             ret: &mut Vec<Vec2<i8>> ) -> Result<(), LoadModelError> {

  match block {
    &Json::Object(ref obj) => {
      try!(parse_2(obj, ret));
      Ok(())
    },

    _ => try!(Err(ModelError::TypeError
                              { obj: "\"rectangle\" key in \"occupied_tiles\"",
                                expected: "object" }))
  }
}


fn parse_2(obj: &BTreeMap<String, Json>,
           ret: &mut Vec<Vec2<i8>> ) -> Result<(), LoadModelError> {

  if obj.len() != 2 {
    return try!(Err(ModelError::WrongNumKeys
                   { expected: 2,
                     context: "\"rectangle\" object in \"occupied_tiles\"" }));
  }

  let type_err = |key| Err(ModelError::TypeError { obj: key,
                                                   expected: "[i8, i8]" });

  let mut start = None;
  let mut size = None;
  for (key, val) in obj.iter() {
    match key as &str {
      "start" => {
        let extract_result = extract_i8_pair(val);
        if let Err(()) = extract_result {
          return try!(type_err(
               "\"start\" key in \"rectangle\" object in \"occupied_tiles\""));
        }
        start = Some(extract_result.unwrap());
      },

      "size" => {
        let extract_result = extract_i8_pair(val);
        if let Err(()) = extract_result {
          return try!(type_err(
                "\"size\" key in \"rectangle\" object in \"occupied_tiles\""));
        }
        size = Some(extract_result.unwrap());
      },

      other => try!(Err(ModelError::InvalidKey
                    { key: other.to_string(),
                      context: "\"rectangle\" object in \"occupied_tiles\"" }))
    };
  }
  Ok(())
}


fn extract_i8_pair(value: &Json) -> Result<Vec2<i8>, ()> {
  match value {
    &Json::Array(ref arr) => {
      if arr.len() != 2 {
        return Err(());
      }
      Ok(Vec2( try!(extract_i8(&arr[0])), try!(extract_i8(&arr[1])) ))
    },
    _ => Err(())
  }
}


fn extract_i8(value: &Json) -> Result<i8, ()> {
  match value {
    // TODO refactor all these methods into Json extract module or something
    // TODO bounds checking
    &Json::I64(n) => Ok(n as i8),
    &Json::U64(n) => Ok(n as i8),
    _ => Err(())
  }
}
