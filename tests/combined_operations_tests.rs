use ::mars_rover::MarsRover;
use test_case::test_case;

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
