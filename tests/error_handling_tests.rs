use ::mars_rover::MarsRover;

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
