extern crate rhino;

use anyhow::Result;
use serial_test::serial;
use std::time::Duration;

use rhino::{Command, Motor};

#[test]
#[serial]
fn test_write() -> Result<()> {
    let mut motor = Motor::from_port(
        serialport::new("/dev/ttyUSB0", 9600)
            .timeout(Duration::from_millis(100))
            .open()?,
    );
    assert!(motor.write(Command::EncoderPosition, 123456789).is_ok());
    // assert_eq!(motor.?)
    assert!(motor.write(Command::EncoderPosition, -123456789).is_ok());
    Ok(())
}

#[test]
#[serial]
fn test_read() -> Result<()> {
    let mut motor = Motor::from_port(
        serialport::new("/dev/ttyUSB0", 9600)
            .timeout(Duration::from_millis(100))
            .open()?,
    );

    motor.write(Command::EncoderPosition, 987654321)?;
    assert_eq!(
        motor
            .read(Command::EncoderPosition)
            .expect("Failed to read encoder position"),
        987654321
    );
    Ok(())
}
