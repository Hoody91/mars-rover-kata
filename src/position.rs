#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// Represents the position of the rover on the Mars grid.
pub(crate) struct Position {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Position {
    /// Creates a new `Position` with the given x and y coordinates.
    pub(crate) fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}
