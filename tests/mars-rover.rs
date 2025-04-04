use ::mars_rover::MarsRover;
use test_case::test_case;

#[test]
fn test_mars_rover_creation_success() {
    // This will specifically test the Ok branch in the new() function
    let result = MarsRover::new("10:20:N");
    assert!(result.is_ok());

    let rover = result.unwrap();
    assert_eq!(rover.to_string(), "10:20:N");
}

#[test]
fn test_mars_rover_all_directions() {
    // Test creation with each possible direction
    for direction in ["N", "E", "S", "W"] {
        let initial_state = format!("5:5:{}", direction);
        let result = MarsRover::new(&initial_state);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), initial_state);
    }
}

#[test_case("0:0:N", "0:0:N"; "Returns the initial state")]
#[test_case("1:-1:W", "1:-1:W"; "Returns a second different initial state")]
#[test_case("5:5:S", "5:5:S"; "Returns a third different initial state")]
fn mars_rover_state(initial_state: &str, expected_state: &str) {
    let rover = MarsRover::new(initial_state);
    let rover = rover.unwrap();
    assert_eq!(
        rover.to_string(),
        expected_state,
        "State of the rover did not match initial state"
    );
}

#[test_case("0:0:N", "0:1:N"; "Rover moves north")]
#[test_case("0:0:W", "-1:0:W"; "Rover moves west")]
#[test_case("0:0:S", "0:-1:S"; "Rover moves south")]
#[test_case("0:0:E", "1:0:E"; "Rover moves east")]
fn mars_rover_move(initial_state: &str, expected_state: &str) {
    let mut rover = MarsRover::new(initial_state).unwrap();
    rover.move_forward();
    assert_eq!(
        rover.to_string(),
        expected_state,
        "State of the rover did not match after moving forward"
    );
}

#[test_case("0:0:N", "0:0:W"; "Rover turns left from north to west")]
#[test_case("0:0:W", "0:0:S"; "Rover turns left from west to south")]
#[test_case("0:0:S", "0:0:E"; "Rover turns left from south to east")]
#[test_case("0:0:E", "0:0:N"; "Rover turns left from east to north")]
fn mars_rover_turn_left(initial_state: &str, expected_state: &str) {
    let mut rover = MarsRover::new(initial_state).unwrap();
    rover.turn_left();
    assert_eq!(
        rover.to_string(),
        expected_state,
        "State of the rover did not match after turning left"
    );
}

#[test_case("0:0:N", "0:0:E"; "Rover turns right from north to east")]
#[test_case("0:0:E", "0:0:S"; "Rover turns right from east to south")]
#[test_case("0:0:S", "0:0:W"; "Rover turns right from south to west")]
#[test_case("0:0:W", "0:0:N"; "Rover turns right from west to north")]
fn mars_rover_turn_right(initial_state: &str, expected_state: &str) {
    let mut rover = MarsRover::new(initial_state).unwrap();
    rover.turn_right();
    assert_eq!(
        rover.to_string(),
        expected_state,
        "State of the rover did not match after turning right"
    );
}

#[test_case("0:0:N", "0:1:W"; "Rover moves forward and turns left")]
#[test_case("0:0:W", "-1:0:S"; "Rover moves forward and turns left from west to south")]
#[test_case("0:0:S", "0:-1:E"; "Rover moves forward and turns left from south to east")]
#[test_case("0:0:E", "1:0:N"; "Rover moves forward and turns left from east to north")]

fn mars_rover_move_and_turns_left(initial_state: &str, expected_state: &str) {
    let mut rover = MarsRover::new(initial_state).unwrap();
    rover.move_forward();
    rover.turn_left();
    assert_eq!(
        rover.to_string(),
        expected_state,
        "State of the rover did not match after moving forward and turning left"
    );
}

#[test_case("0:0:N", "0:1:E"; "Rover moves forward and turns right")]
#[test_case("0:0:E", "1:0:S"; "Rover moves forward and turns right from east to south")]
#[test_case("0:0:S", "0:-1:W"; "Rover moves forward and turns right from south to west")]
#[test_case("0:0:W", "-1:0:N"; "Rover moves forward and turns right from west to north")]
fn mars_rover_move_and_turns_right(initial_state: &str, expected_state: &str) {
    let mut rover = MarsRover::new(initial_state).unwrap();
    rover.move_forward();
    rover.turn_right();
    assert_eq!(
        rover.to_string(),
        expected_state,
        "State of the rover did not match after moving forward and turning right"
    );
}

#[test_case("0:0:N", "MMM", "0:3:N"; "Rover moves forward 3 times")]
#[test_case("0:0:N", "MMRMM", "2:2:E"; "Rover moves forward 2 times and turns right and moves forward 2 times")]

fn mars_rover_runs_commands(initial_state: &str, commands: &str, expected_state: &str) {
    let mut rover = MarsRover::new(initial_state).unwrap();
    rover.execute_commands(commands).unwrap();
    assert_eq!(
        rover.to_string(),
        expected_state,
        "State of the rover did not match after executing commands"
    );
}

#[test]
fn mars_rover_invalid_initial_state_format() {
    let result = MarsRover::new("0:0");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid initial state");
}

#[test]
fn mars_rover_invalid_x_coordinate() {
    let result = MarsRover::new("abc:0:N");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid x coordinate"));
}

#[test]
fn mars_rover_invalid_y_coordinate() {
    let result = MarsRover::new("0:abc:N");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid y coordinate"));
}

#[test]
fn mars_rover_invalid_direction() {
    let result = MarsRover::new("0:0:X");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid direction"));
}

#[test]
fn mars_rover_trims_whitespace() {
    let rover = MarsRover::new(" 1:2:N ").unwrap();
    assert_eq!(rover.to_string(), "1:2:N");
}

#[test_case('M', true; "Valid move command")]
#[test_case('L', true; "Valid left turn command")]
#[test_case('R', true; "Valid right turn command")]
#[test_case('X', false; "Invalid command")]
fn mars_rover_command_validation(command: char, should_succeed: bool) {
    let mut rover = MarsRover::new("0:0:N").unwrap();
    let result = rover.command(command);
    assert_eq!(result.is_ok(), should_succeed);
}

#[test_case("0:0:N", "MRMLLM", "0:1:W"; "Complex sequence 1")]
#[test_case("5:5:E", "MMLMRMMRRMML", "7:6:S"; "Complex sequence 2")]
#[test_case("3:3:S", "MMMLMMMRMMMML", "6:-4:E"; "Complex sequence 3")]
fn mars_rover_complex_commands(initial_state: &str, commands: &str, expected_state: &str) {
    let mut rover = MarsRover::new(initial_state).unwrap();
    rover.execute_commands(commands).unwrap();
    assert_eq!(
        rover.to_string(),
        expected_state,
        "State of the rover did not match after executing commands"
    );
}

#[test]
fn mars_rover_invalid_command() {
    let mut rover = MarsRover::new("0:0:N").unwrap();
    let result = rover.execute_commands("MMXMM");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid command"));
}
#[test]
fn mars_rover_full_rotation_left() {
    let mut rover = MarsRover::new("0:0:N").unwrap();
    rover.execute_commands("LLLL").unwrap();
    assert_eq!(
        rover.to_string(),
        "0:0:N",
        "Rover should be back at starting direction after 4 left turns"
    );
}

#[test]
fn mars_rover_full_rotation_right() {
    let mut rover = MarsRover::new("0:0:N").unwrap();
    rover.execute_commands("RRRR").unwrap();
    assert_eq!(
        rover.to_string(),
        "0:0:N",
        "Rover should be back at starting direction after 4 right turns"
    );
}

#[test]
fn mars_rover_square_pattern() {
    let mut rover = MarsRover::new("0:0:N").unwrap();
    rover.execute_commands("MMRMMLMMRMMLMMRMM").unwrap(); // Move in a 2x2 square
    assert_eq!(
        rover.to_string(),
        "6:6:E",
        "Rover should complete a square and return to starting position"
    );
}

#[test]
fn mars_rover_return_to_start() {
    let mut rover = MarsRover::new("0:0:N").unwrap();
    rover.execute_commands("MMMMLLMMMM").unwrap();
    assert_eq!(
        rover.to_string(),
        "0:0:S",
        "Rover should return to starting position but face opposite direction"
    );
}

#[test]
fn mars_rover_mixed_rotations() {
    let mut rover = MarsRover::new("0:0:N").unwrap();
    rover.execute_commands("RLRLRLRL").unwrap();
    assert_eq!(
        rover.to_string(),
        "0:0:N",
        "Rover direction should be unchanged after alternating turns"
    );
}

#[test_case("1000000:1000000:N", "MMMM", "1000000:1000004:N"; "Large positive coordinates")]
#[test_case("-1000000:-1000000:S", "MMMM", "-1000000:-1000004:S"; "Large negative coordinates")]
fn mars_rover_large_coordinates(initial_state: &str, commands: &str, expected_state: &str) {
    let mut rover = MarsRover::new(initial_state).unwrap();
    rover.execute_commands(commands).unwrap();
    assert_eq!(
        rover.to_string(),
        expected_state,
        "State of the rover did not match with large coordinates"
    );
}
