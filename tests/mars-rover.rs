use ::mars_rover::MarsRover;
use test_case::test_case;

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
    rover.execute_commands(commands);
    assert_eq!(
        rover.to_string(),
        expected_state,
        "State of the rover did not match after executing commands"
    );
}
