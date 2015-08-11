use std::collections::BTreeMap;

use rustc_serialize::json::Json;

use super::{Frame, Sprite};
use super::error::{LoadModelError, ModelError};

mod settings;
mod sprite_builder;

use self::settings::SpriteDataSettings;
use self::sprite_builder::SpriteBuilder;


/// Process sprite data.
pub fn parse(data: &Vec<Json>)
                              -> Result<BTreeMap<String, Sprite>, ModelError> {

  let mut uncompressed: BTreeMap<String, SpriteBuilder> = BTreeMap::new();
  try!(parse_2(data, &SpriteDataSettings::defaults(), &mut uncompressed));

  let mut ret = BTreeMap::new();
  for (key, val) in uncompressed {
    let sprite = try!(val.build(&key));
    ret.insert(key, sprite);
  }
  Ok(ret)
}


/// Just the recursive version of `parse`.
///
/// This fills in `ret` with the processed data. It returns Ok(()) on success.
fn parse_2(data: &Vec<Json>,
           settings: &SpriteDataSettings,
           ret: &mut BTreeMap<String, SpriteBuilder> )
                                                    -> Result<(), ModelError> {

  for obj in data {
    try!(process(obj, settings, ret));
  };
  Ok(())
}


/// Process one item in "sprite_data"
fn process(json_obj: &Json,
           settings: &SpriteDataSettings,
           ret: &mut BTreeMap<String, SpriteBuilder> )
                                                    -> Result<(), ModelError> {

  match json_obj {
    &Json::Object(ref obj) => process_2(obj, settings, ret),
    _ => Err(ModelError::TypeError { obj: "\"sprite_data\" list element",
                                     expected: "object" })
  }
}


/// Process an object in "sprite_data"
///
/// This function just processes a JSON value after it has been verified to be
/// an object.
fn process_2(obj: &BTreeMap<String, Json>,
             settings: &SpriteDataSettings,
             ret: &mut BTreeMap<String, SpriteBuilder> )
                                                    -> Result<(), ModelError> {

  if obj.contains_key("with") {
    if obj.len() > 1 {
      return Err(ModelError::WrongNumKeys
                                    { expected: 1,
                                      context: "object containing \"with\"" });
    }
    process_with(obj, settings, ret)
  }
  else {
    let mut final_settings = settings.clone();
    try!(settings::modify(&mut final_settings, obj));
    insert_frame(&final_settings, ret)
  }
}


fn process_with(with_obj: &BTreeMap<String, Json>,
                settings: &SpriteDataSettings,
                ret: &mut BTreeMap<String, SpriteBuilder> )
                                                    -> Result<(), ModelError> {

  let mut new_settings = settings.clone();
  let mut data = None;
  for (key, val) in with_obj.iter() {
    match key as &str {
      "defaults" => {
        if let &Json::Object(ref delta) = val {
          try!(settings::modify(&mut new_settings, &delta));
        }
        else {
          return Err(ModelError::TypeError { obj: "\"defaults\"",
                                             expected: "object" });
        };
      },
      "data" => {
        match val {
          &Json::Object(ref obj) => data = Some(obj),
          _ => return Err(ModelError::TypeError { obj: "\"data\"",
                                                  expected: "object" })
        }
      },
      other => return Err(ModelError::InvalidKey
                                                { key: other.to_string(),
                                                  context: "\"with\" object" })
    };
  };
  if let Some(obj) = data {
    return process_2(obj, &new_settings, ret);
  }
  Err(ModelError::MissingKey { key: "data", context: "\"with\" object" })
}


fn insert_frame(settings: &SpriteDataSettings,
                ret: &mut BTreeMap<String, SpriteBuilder> )
                                                    -> Result<(), ModelError> {

  let mut sprite = match ret.get(&settings.sprite_name) {
    Some(sprite) => sprite.clone(),
    None => SpriteBuilder::new(),
  };

  if let Some(_) = sprite.frames.get(&settings.frame_index) {
    return Err(ModelError::FrameRedef
                                   { sprite_name: settings.sprite_name.clone(),
                                     frame_index: settings.frame_index });
  };

  let frame = Frame {
    cut_from: settings.cut_from,
    cut_offset: settings.cut_offset,
    size: settings.size,
    pin_to: settings.pin_to,
    pin_offset: settings.pin_offset,
  };

  sprite.frames.insert(settings.frame_index, frame);

  ret.insert(settings.sprite_name.clone(), sprite);
  Ok(())
}
