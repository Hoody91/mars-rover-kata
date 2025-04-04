#[derive(Debug)]
pub(crate) struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub(crate) fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub(crate) fn x(&self) -> i32 {
        self.x
    }
    pub(crate) fn y(&self) -> i32 {
        self.y
    }

    pub(crate) fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub(crate) fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}
