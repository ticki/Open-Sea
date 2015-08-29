use std::collections::BTreeMap;

use rustc_serialize::json::Json;

use super::error::ModelError;
use math::Vec2;

mod individual;
mod rectangle;


/// Parse data about occupied tiles
pub fn parse(data: &Vec<Json>) -> Result<Vec<Vec2<i8>>, ModelError> {
    let mut ret = Vec::new();
    try!(parse_2(data, &mut ret));
    Ok(ret)
}


fn parse_2(data: &Vec<Json>,
           ret: &mut Vec<Vec2<i8>> ) -> Result<(), ModelError> {

    for obj in data {
        try!(parse_obj(obj, ret));
    }
    Ok(())
}


fn parse_obj(obj: &Json,
             ret: &mut Vec<Vec2<i8>> ) -> Result<(), ModelError> {

    match obj {
        &Json::Object(ref obj) => parse_obj_2(obj, ret),
        _ => Err(ModelError::TypeError { obj: "\"occupied_tiles\"",
            expected: "object" })
    }
}


fn parse_obj_2(obj: &BTreeMap<String, Json>,
               ret: &mut Vec<Vec2<i8>> ) -> Result<(), ModelError> {

    if obj.len() > 1 {
        return Err(ModelError::WrongNumKeys
                   { expected: 1,
                   context: "\"occupied_tiles\" list element object" });
    }
    for (key, val) in obj.iter() {
        match key as &str {
            "individual" => try!(individual::parse(val, ret)),
            "rectangle" => try!(rectangle::parse(val, ret)),
            other => return Err(ModelError::InvalidKey
                                { key: other.to_string(),
                                context: "\"occupied_tiles\" list element object" })
        }
    }
    Ok(())
}
