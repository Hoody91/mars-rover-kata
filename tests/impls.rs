use mars_rover::MarsRover;

#[test]
fn mars_rover_default() {
    let rover = MarsRover::default();
    assert_eq!(rover.to_string(), "0:0:N");
}

#[test]
fn mars_rover_clone() {
    let mut original = MarsRover::new("0:0:N").unwrap();
    original.execute_commands("MM").unwrap();

    let mut clone = original.clone();
    original.execute_commands("RR").unwrap();
    clone.execute_commands("LL").unwrap();

    assert_eq!(original.to_string(), "0:2:S");
    assert_eq!(clone.to_string(), "0:2:S");
}
