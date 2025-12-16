use std::cell::Cell;

pub struct RatAnimation {
    frames: Vec<String>,
    current_frame: Cell<usize>,
}

impl RatAnimation {
    pub fn new() -> Self {
        let frames = vec![
            r#"
    /\_/\
   ( o.o )
    > ^ <
   /|   |\
  (_|   |_)
"#
            .to_string(),
            r#"
    /\_/\
   ( ^.^ )
    > ~ <
   /|   |\
  (_|   |_)
"#
            .to_string(),
            r#"
    /\_/\
   ( o.o )
    > ^ <
    |   |
   / \ / \
"#
            .to_string(),
            r#"
    /\_/\
   ( ^.^ )
    > v <
    |   |
   / \ / \
"#
            .to_string(),
            r#"
    /\_/\
   ( o.o )
    > ^ <
  /|     |\
 (_|     |_)
"#
            .to_string(),
            r#"
    /\_/\
   ( ^.^ )
    > w <
  /|     |\
 (_|     |_)
"#
            .to_string(),
        ];

        Self {
            frames,
            current_frame: Cell::new(0),
        }
    }

    pub fn get_current_frame(&self) -> &str {
        &self.frames[self.current_frame.get()]
    }

    pub fn next_frame(&self) {
        let next = (self.current_frame.get() + 1) % self.frames.len();
        self.current_frame.set(next);
    }

    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }
}
