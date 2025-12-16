use crate::animation_loader::AnimationConfig;
use std::cell::Cell;

pub trait Animation {
    fn get_current_frame(&self) -> &str;
    fn next_frame(&self);
    fn frame_count(&self) -> usize;
    fn name(&self) -> &str;
}

pub struct CustomAnimation {
    name: String,
    frames: Vec<String>,
    current_frame: Cell<usize>,
}

impl CustomAnimation {
    pub fn new(config: AnimationConfig) -> Self {
        Self {
            name: config.name,
            frames: config.frames,
            current_frame: Cell::new(0),
        }
    }

    pub fn from_frames(name: String, frames: Vec<String>) -> Self {
        Self {
            name,
            frames,
            current_frame: Cell::new(0),
        }
    }
}

impl Animation for CustomAnimation {
    fn get_current_frame(&self) -> &str {
        &self.frames[self.current_frame.get()]
    }

    fn next_frame(&self) {
        let next = (self.current_frame.get() + 1) % self.frames.len();
        self.current_frame.set(next);
    }

    fn frame_count(&self) -> usize {
        self.frames.len()
    }

    fn name(&self) -> &str {
        &self.name
    }
}
