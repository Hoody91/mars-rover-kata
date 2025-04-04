use ::mars_rover::MarsRover;
use test_case::test_case;

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
fn mars_rover_mixed_rotations() {
    let mut rover = MarsRover::new("0:0:N").unwrap();
    rover.execute_commands("RLRLRLRL").unwrap();
    assert_eq!(
        rover.to_string(),
        "0:0:N",
        "Rover direction should be unchanged after alternating turns"
    );
}
