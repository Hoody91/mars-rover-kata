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

#[test]
fn mars_rover_trims_whitespace() {
    let rover = MarsRover::new(" 1:2:N ").unwrap();
    assert_eq!(rover.to_string(), "1:2:N");
}
