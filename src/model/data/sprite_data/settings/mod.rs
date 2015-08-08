use model::data::Sprite;

mod modify;
pub use self::modify::modify;


/// This struct simplifies the implementation of `parse`
#[derive(Clone, Debug)]
pub struct SpriteDataSettings {
  pub sprite_name: String,
  pub frame_index: usize,
  pub cut_from: (u16, u16),
  pub cut_offset: (i8, i8),
  pub size: (u16, u16),
  pub pin_to: (i16, i16),
  pub pin_offset: (i8, i8),
}


impl SpriteDataSettings {
  pub fn defaults() -> SpriteDataSettings {
    SpriteDataSettings {
      sprite_name: "default".to_string(),
      frame_index: 0,
      cut_from: (0, 0),
      cut_offset: (0, 0),
      size: (1, 1),
      pin_to: (0, 0),
      pin_offset: (0, 0),
    }
  }
}
