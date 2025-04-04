use ::mars_rover::MarsRover;
use test_case::test_case;

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
