extern crate rhino;

use anyhow::Result;

use rhino::Command;

#[test]
fn test_as_char() {
    assert_eq!(Command::MotorSpeedAndDirection.as_char(), 'S')
}

#[test]
fn test_from_char() {
    assert_eq!(
        Command::from_char('S').unwrap(),
        Command::MotorSpeedAndDirection
    )
}
