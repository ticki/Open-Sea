use std::cmp::max;
use std::collections::BTreeMap;

use model::data::{Frame, Sprite};
use model::data::error::ModelError;


#[derive(Clone, Debug)]
pub struct SpriteBuilder {
    pub resource: Option<usize>,
    pub frames: BTreeMap<usize, Frame>,
}


impl SpriteBuilder {
    pub fn new() -> SpriteBuilder {
        SpriteBuilder { resource: None, frames: BTreeMap::new() }
    }

    pub fn build(mut self, name: &String) -> Result<Sprite, ModelError> {
        let resource;
        if let Some(r) = self.resource {
            resource = r;
        }
        else {
            // TODO get default tileset from resource manager
            resource = 0;
        }

        if self.frames.len() == 0 {
            panic!("No frames given for sprite.")
        }
        let expected_max = self.frames.len();

        let mut actual_max = 0;
        for key in self.frames.keys() {
            actual_max = max(*key, actual_max);
        }
        if actual_max != expected_max {
            return Err(ModelError::InvalidFrames { sprite_name: name.clone(),
            length: expected_max,
            max_index: actual_max });
        }

        let mut frames = Vec::with_capacity(expected_max);
        let mut i = frames.len();
        while i < expected_max {
            frames.push(self.frames.remove(&i).unwrap());
            i = frames.len();
        }

        Ok(Sprite::new(resource, frames))
    }
}
