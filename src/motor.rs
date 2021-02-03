use anyhow::Result;
use onig::Regex;
use serialport::SerialPort;
use std::io::{BufRead, BufReader, Lines, Write};

use crate::command::Command;

lazy_static! {
    static ref READ_RE: Regex =
        Regex::new(r#"(?<cmd>[SMDEYPGRABCX])(?: (?<value>-?\d+))?"#).unwrap();
}

pub struct Motor {
    port: Box<dyn SerialPort>,
    pub(crate) reader: Lines<BufReader<Box<dyn SerialPort>>>,
}

impl Motor {
    pub fn from_port(port: Box<dyn SerialPort>) -> Self {
        Self {
            reader: BufReader::new(port.try_clone().expect("Failed to clone port")).lines(),
            port,
        }
    }

    fn read_response(&mut self) -> Result<(Command, Option<i32>)> {
        if let Some(response_string) = self.reader.next() {
            if let Some(response_caps) = READ_RE.captures(&response_string?) {
                if let Some(command_char) = response_caps.at(1) {
                    return Ok((
                        Command::from_char(command_char.chars().next().unwrap())?,
                        response_caps
                            .at(2)
                            .map(|value_str| value_str.parse::<i32>().unwrap()),
                    ));
                }
                bail!(
                    "Got incorrect response from motor, got: '{:?}'",
                    response_caps.at(0)
                )
            }
        }
        bail!("Found no response to read")
    }

    pub fn write(&mut self, command: Command, value: i32) -> Result<()> {
        write!(self.port, "{} {}\r\n", command.as_char(), value)?;
        let response = self.read_response()?;
        if response == (command, Some(value)) {
            return Ok(());
        }
        bail!(
            "Got incorrect response from motor. Wrote: '{} {}' Read: '{} {:?}'",
            command.as_char(),
            value,
            response.0.as_char(),
            response.1
        )
    }

    pub fn read(&mut self, command: Command) -> Result<i32> {
        write!(self.port, "{}\r\n", command.as_char())?;
        let (response_command, response_value) = self.read_response()?;
        if response_command == command {
            if let Some(value) = response_value {
                return Ok(value);
            }
        }
        bail!(
            "Got incorrect response from motor. Wrote: '{}' Read: '{} {:?}'",
            command.as_char(),
            response_command.as_char(),
            response_value
        )
    }

    pub fn read_speed(&mut self) -> Result<i32> {
        self.read(Command::MotorSpeedAndDirection)
    }
    pub fn write_speed(&mut self, value: i32) -> Result<()> {
        self.write(Command::MotorSpeedAndDirection, value)
    }

    pub fn read_max_speed(&mut self) -> Result<i32> {
        self.read(Command::MotorMaxSpeed)
    }
    pub fn write_max_speed(&mut self, value: i32) -> Result<()> {
        self.write(Command::MotorMaxSpeed, value)
    }

    pub fn read_speed_damping(&mut self) -> Result<i32> {
        self.read(Command::SpeedDamping)
    }
    pub fn write_speed_damping(&mut self, value: i32) -> Result<()> {
        self.write(Command::SpeedDamping, value)
    }

    pub fn read_i2c_address(&mut self) -> Result<i32> {
        self.read(Command::I2CAddress)
    }
    pub fn write_i2c_address(&mut self, value: i32) -> Result<()> {
        self.write(Command::I2CAddress, value)
    }

    // fn load_default_values(self);

    pub fn read_position(&mut self) -> Result<i32> {
        self.read(Command::EncoderPosition)
    }
    pub fn write_position(&mut self, value: i32) -> Result<()> {
        self.write(Command::EncoderPosition, value)
    }
    pub fn read_goto_position(&mut self) -> Result<i32> {
        self.read(Command::GotoPosition)
    }
    pub fn write_goto_position(&mut self, value: i32) -> Result<()> {
        self.write(Command::GotoPosition, value)
    }
    pub fn write_goto_relative_position(&mut self, value: i32) -> Result<()> {
        self.write(Command::RelativeGotoPosition, value)
    }
    pub fn read_speedfeedback_gain(&mut self) -> Result<i32> {
        self.read(Command::SpeedFeedbackGain)
    }
    pub fn write_speedfeedback_gain(&mut self, value: i32) -> Result<()> {
        self.write(Command::SpeedFeedbackGain, value)
    }
    pub fn read_p_gain(&mut self) -> Result<i32> {
        self.read(Command::PGain)
    }
    pub fn write_p_gain(&mut self, value: i32) -> Result<()> {
        self.write(Command::PGain, value)
    }
    pub fn read_i_gain(&mut self) -> Result<i32> {
        self.read(Command::IGain)
    }
    pub fn write_i_gain(&mut self, value: i32) -> Result<()> {
        self.write(Command::IGain, value)
    }

    // fn auto_calibrate_speedfeedback_gain(self);
}
