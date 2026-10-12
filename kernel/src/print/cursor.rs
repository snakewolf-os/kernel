#[derive(Clone, Copy)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Cursor {
    pub const fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
    }

    pub fn set(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}
