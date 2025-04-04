#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) enum Direction {
    #[default]
    North,
    East,
    South,
    West,
}

impl TryFrom<&str> for Direction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "north" | "n" => Some(Direction::North),
            "east" | "e" => Some(Direction::East),
            "south" | "s" => Some(Direction::South),
            "west" | "w" => Some(Direction::West),
            _ => None,
        }
        .ok_or_else(|| format!("Invalid direction '{}'", value))
    }
}

impl Direction {
    /// Returns the delta (x, y) for the direction.
    /// North is (0, 1), East is (1, 0), South is (0, -1), West is (-1, 0).
    /// This is used to calculate the new position of the rover when it moves forward.
    pub(crate) fn get_delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        }
    }

    /// Returns the direction after turning left.
    /// North -> West, East -> North, South -> East, West -> South
    pub(crate) fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    /// Returns the direction after turning right.
    /// North -> East, East -> South, South -> West, West -> North
    pub(crate) fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "N"),
            Direction::East => write!(f, "E"),
            Direction::South => write!(f, "S"),
            Direction::West => write!(f, "W"),
        }
    }
}
