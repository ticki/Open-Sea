use core::Entity;
use math::Vec2;

/// The camera
pub struct Cam<'a> {
    /// The entity in focus
    in_focus: &'a Entity,
}

// TODO: [x] Make the camera locked to the player? (Or is this a bad idea?)

impl<'a> Cam<'a> {
    /// Creates a new Cam
    fn new(focus: &'a Entity) -> Cam {
        Cam {
            in_focus: focus,
        }
    }
    /// Get position
    fn get_pos(&self) -> Vec2<f64> {
        self.in_focus.get_cur_pos() 
    }
}
