use rustc_serialize::json::Json;

use core::Vec2;

use model::data::error::ModelError;

/// Parse individual occupied tiles
pub fn parse(block: &Json,
             ret: &mut Vec<Vec2<i8>> ) -> Result<(), ModelError> {

  match block {
    &Json::Array(ref arr) => {
      for obj in arr {
        try!(parse_element(obj,
                           ret,
                           "element of list \"individual\"",
                           "[i8, i8]" ));
      }
    },
    _ => return Err(ModelError::TypeError
                           { obj: "\"individual\" block in \"occupied_tiles\"",
                             expected: "array" })
  }
  Ok(())
}


fn parse_element(elem: &Json,
                 ret: &mut Vec<Vec2<i8>>,
                 type_err_obj: &'static str,
                 type_err_exp: &'static str ) -> Result<(), ModelError> {

  let type_err = ModelError::TypeError { obj: type_err_obj,
                                         expected: type_err_exp };

  match elem {
    &Json::Array(ref tile) => {
      if tile.len() != 2 {
        return Err(type_err);
      }
      let v = Vec2(
          try!(extract_i8(&tile[0], type_err_obj, type_err_exp)),
          try!(extract_i8(&tile[1], type_err_obj, type_err_exp)) );
      ret.push(v);
      Ok(())
    },
    _ => Err(type_err)
  }
}


fn extract_i8(value: &Json,
              type_err_obj: &'static str,
              type_err_exp: &'static str ) -> Result<i8, ModelError> {
  match value {
    &Json::I64(n) => Ok(n as i8),
    &Json::U64(n) => Ok(n as i8),
    _ => Err(ModelError::TypeError { obj: type_err_obj,
                                     expected: type_err_exp })
  }
}
