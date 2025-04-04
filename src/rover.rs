use crate::{direction::Direction, position::Position};

#[derive(Debug, Default, Clone)]
pub struct MarsRover {
    direction: Direction,
    position: Position,
}

impl MarsRover {
    pub fn new(initial_state: &str) -> Result<Self, String> {
        let parts: Vec<_> = initial_state.trim().split(':').collect();
        if parts.len() != 3 {
            return Err("Invalid initial state".to_string());
        }

        let x: i32 = parts[0]
            .parse()
            .map_err(|_| format!("Invalid x coordinate '{}'", parts[0]))?;
        let y: i32 = parts[1]
            .parse()
            .map_err(|_| format!("Invalid y coordinate '{}'", parts[1]))?;

        let direction = Direction::from_str(parts[2])
            .ok_or_else(|| format!("Invalid direction '{}'", parts[2]))?;

        Ok(MarsRover {
            direction,
            position: Position::new(x, y),
        })
    }

    pub fn move_forward(&mut self) {
        let (dx, dy) = self.direction.get_delta();
        self.position.set_x(self.position.x() + dx);
        self.position.set_y(self.position.y() + dy);
    }

    pub fn turn_left(&mut self) {
        self.direction = self.direction.turn_left();
    }

    pub fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    pub fn command(&mut self, command: char) -> Result<(), String> {
        match command {
            'M' => {
                self.move_forward();
                Ok(())
            }
            'L' => {
                self.turn_left();
                Ok(())
            }
            'R' => {
                self.turn_right();
                Ok(())
            }
            _ => Err(format!("Invalid command: {}", command)),
        }
    }

    pub fn execute_commands(&mut self, commands: &str) -> Result<(), String> {
        for command in commands.chars() {
            self.command(command)?;
        }
        Ok(())
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
