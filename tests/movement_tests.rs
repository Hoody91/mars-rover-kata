use ::mars_rover::MarsRover;
use test_case::test_case;

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
