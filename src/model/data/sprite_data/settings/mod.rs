mod modify;
pub use self::modify::modify;

use math::Vec2;


/// This struct simplifies the implementation of `parse`
#[derive(Clone, Debug)]
pub struct SpriteDataSettings {
    pub sprite_name: String,
    pub frame_index: usize,
    pub cut_from: Vec2<u16>,
    pub cut_offset: Vec2<i8>,
    pub size: Vec2<u16>,
    pub pin_to: Vec2<i16>,
    pub pin_offset: Vec2<i8>,
}


impl SpriteDataSettings {
    pub fn defaults() -> SpriteDataSettings {
        SpriteDataSettings {
            sprite_name: "default".to_string(),
            frame_index: 0,
            cut_from: Vec2(0, 0),
            cut_offset: Vec2(0, 0),
            size: Vec2(1, 1),
            pin_to: Vec2(0, 0),
            pin_offset: Vec2(0, 0),
        }
    }
}
