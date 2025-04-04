use crate::{direction::Direction, position::Position};

#[derive(Debug)]
pub struct MarsRover {
    direction: Direction,
    position: Position,
}

impl MarsRover {
    pub fn new(initial_state: &str) -> Result<Self, String> {
        let parts: Vec<_> = initial_state.split(':').collect();
        if parts.len() != 3 {
            return Err("Invalid initial state".to_string());
        }

        let x: i32 = parts[0].parse().map_err(|_| "Invalid x coordinate")?;
        let y: i32 = parts[1].parse().map_err(|_| "Invalid y coordinate")?;

        let direction = match parts[2] {
            "N" => Direction::North,
            "E" => Direction::East,
            "S" => Direction::South,
            "W" => Direction::West,
            _ => return Err("Invalid direction".to_string()),
        };

        Ok(MarsRover {
            direction,
            position: Position::new(x, y),
        })
    }

    pub fn move_forward(&mut self) {
        match self.direction {
            Direction::North => self.position.set_y(self.position.y() + 1),
            Direction::East => self.position.set_x(self.position.x() + 1),
            Direction::South => self.position.set_y(self.position.y() - 1),
            Direction::West => self.position.set_x(self.position.x() - 1),
        }
    }

    pub fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    pub fn command(&mut self, command: char) {
        match command {
            'M' => self.move_forward(),
            'L' => self.turn_left(),
            'R' => self.turn_right(),
            _ => panic!("Invalid command"),
        }
    }

    pub fn execute_commands(&mut self, commands: &str) {
        for command in commands.chars() {
            self.command(command);
        }
    }
}

impl std::fmt::Display for MarsRover {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.position.x(),
            self.position.y(),
            self.direction
        )
    }
}
