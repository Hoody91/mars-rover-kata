use crate::{direction::Direction, position::Position};

#[derive(Debug, Default, Clone)]
/// Represents the Mars Rover.
/// The rover can move forward, turn left, or turn right.
/// ## Examples
/// ```
/// use mars_rover::MarsRover;
///
/// let mut rover = MarsRover::new("3:4:N").unwrap();
///
/// assert_eq!(rover.to_string(), "3:4:N");
///
/// rover.move_forward();
/// assert_eq!(rover.to_string(), "3:5:N");
///
/// rover.turn_left();
/// assert_eq!(rover.to_string(), "3:5:W");
///
/// rover.move_forward();
/// assert_eq!(rover.to_string(), "2:5:W");
///
/// rover.turn_right();
/// assert_eq!(rover.to_string(), "2:5:N");
///
/// rover.execute_commands("MMRMM");
/// assert_eq!(rover.to_string(), "4:7:E");
///
/// rover.command('L').unwrap();
/// assert_eq!(rover.to_string(), "4:7:N");
///
/// rover.command('M').unwrap();
/// assert_eq!(rover.to_string(), "4:8:N");
///
/// rover.command('R').unwrap();
/// assert_eq!(rover.to_string(), "4:8:E");
/// ```
pub struct MarsRover {
    direction: Direction,
    position: Position,
}

impl MarsRover {
    /// Creates a new `MarsRover` with the given initial state.
    /// The initial state should be in the format "x:y:direction", where:
    /// - x is the x-coordinate (integer)
    /// - y is the y-coordinate (integer)
    /// - direction is one of "N", "E", "S", "W" (case insensitive)
    ///
    /// Returns a `Result` containing the `MarsRover` or an error message.
    ///
    /// ## Errors
    /// Returns an error if the initial state is not in the correct format or if the coordinates are invalid.
    ///
    /// ## Examples
    /// ```
    /// use mars_rover::MarsRover;
    ///
    /// let rover = MarsRover::new("3:4:N").unwrap();
    ///
    /// assert_eq!(rover.to_string(), "3:4:N");
    /// ```
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

        let direction = Direction::try_from(parts[2])?;

        Ok(MarsRover {
            direction,
            position: Position::new(x, y),
        })
    }

    /// Moves the rover one step in the current direction.
    /// The rover's position is updated accordingly.
    /// The direction is determined by the `Direction` enum.
    /// The rover moves in the following way:
    /// - North (N): y + 1
    /// - East (E): x + 1
    /// - South (S): y - 1
    /// - West (W): x - 1
    /// ## Examples
    /// ```
    /// use mars_rover::MarsRover;
    ///
    /// let mut rover = MarsRover::new("0:0:N").unwrap();
    ///
    /// rover.move_forward();
    ///
    /// assert_eq!(rover.to_string(), "0:1:N");
    /// ```
    pub fn move_forward(&mut self) {
        let (dx, dy) = self.direction.get_delta();
        self.position.x += dx;
        self.position.y += dy;
    }
    /// Turns the rover left (90 degrees counter-clockwise).
    /// The direction is updated according to the `Direction` enum.
    /// The rover turns in the following way:
    /// - North (N) -> West (W)
    /// - East (E) -> North (N)
    /// - South (S) -> East (E)
    /// - West (W) -> South (S)
    /// ## Examples
    /// ```
    /// use mars_rover::MarsRover;
    ///
    /// let mut rover = MarsRover::new("0:0:N").unwrap();
    ///
    /// rover.turn_left();
    ///
    /// assert_eq!(rover.to_string(), "0:0:W");
    /// ```
    pub fn turn_left(&mut self) {
        self.direction = self.direction.turn_left();
    }

    /// Turns the rover right (90 degrees clockwise).
    /// The direction is updated according to the `Direction` enum.
    /// The rover turns in the following way:
    /// - North (N) -> East (E)
    /// - East (E) -> South (S)
    /// - South (S) -> West (W)
    /// - West (W) -> North (N)
    /// ## Examples
    /// ```
    /// use mars_rover::MarsRover;
    ///
    /// let mut rover = MarsRover::new("0:0:N").unwrap();
    ///
    /// rover.turn_right();
    ///
    /// assert_eq!(rover.to_string(), "0:0:E");
    /// ```
    pub fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    /// Executes a single command for the rover.
    /// The command can be:
    /// - 'M' for move forward
    /// - 'L' for turn left
    /// - 'R' for turn right
    /// ## Examples
    /// ```
    /// use mars_rover::MarsRover;
    ///
    /// let mut rover = MarsRover::new("0:0:N").unwrap();
    ///
    /// rover.command('M').unwrap();
    /// assert_eq!(rover.to_string(), "0:1:N");
    ///
    /// rover.command('L').unwrap();
    /// assert_eq!(rover.to_string(), "0:1:W");
    ///
    /// rover.command('M').unwrap();
    /// assert_eq!(rover.to_string(), "-1:1:W");
    /// ```
    /// ## Errors
    /// Returns an error if the command is invalid.
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

    /// Executes a series of commands for the rover.
    /// The commands can be:
    /// - 'M' for move forward
    /// - 'L' for turn left
    /// - 'R' for turn right
    /// ## Examples
    /// ```
    /// use mars_rover::MarsRover;
    ///
    /// let mut rover = MarsRover::new("0:0:N").unwrap();
    ///
    /// rover.execute_commands("MMRMM").unwrap();
    ///
    /// assert_eq!(rover.to_string(), "2:2:E");
    /// ```
    /// ## Errors
    /// Returns an error if any command in the series is invalid.
    /// The rover will stop executing commands at the first invalid command.
    /// The rover's state will be updated up to the point of the invalid command.
    /// If the command is invalid, the rover's state will not be updated.
    /// The rover will not execute any further commands after the invalid command.
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
            self.position.x, self.position.y, self.direction
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mars_rover_creation() {
        let input = "3:4:N";
        let rover = MarsRover::new(input).expect("MarsRover should be created successfully");

        assert_eq!(rover.position.x, 3);
        assert_eq!(rover.position.y, 4);
        assert_eq!(rover.direction, Direction::North);
    }
}
