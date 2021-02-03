use anyhow::Result;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Command {
    MotorSpeedAndDirection,
    MotorMaxSpeed,
    SpeedDamping,
    I2CAddress,
    LoadDefaultValues,
    EncoderPosition,
    GotoPosition,
    RelativeGotoPosition,
    SpeedFeedbackGain,
    PGain,
    IGain,
    AutoCalibrateSpeedFeedbackGain,
    // Unknown
}

impl Command {
    pub fn as_char(&self) -> char {
        match *self {
            Command::MotorSpeedAndDirection => 'S',
            Command::MotorMaxSpeed => 'M',
            Command::SpeedDamping => 'D',
            Command::I2CAddress => 'E',
            Command::LoadDefaultValues => 'Y',
            Command::EncoderPosition => 'P',
            Command::GotoPosition => 'G',
            Command::RelativeGotoPosition => 'R',
            Command::SpeedFeedbackGain => 'A',
            Command::PGain => 'B',
            Command::IGain => 'C',
            Command::AutoCalibrateSpeedFeedbackGain => 'X',
        }
    }

    pub fn from_char(c: char) -> Result<Command> {
        Ok(match c {
            'S' => Self::MotorSpeedAndDirection,
            'M' => Self::MotorMaxSpeed,
            'D' => Self::SpeedDamping,
            'E' => Self::I2CAddress,
            'Y' => Self::LoadDefaultValues,
            'P' => Self::EncoderPosition,
            'G' => Self::GotoPosition,
            'R' => Self::RelativeGotoPosition,
            'A' => Self::SpeedFeedbackGain,
            'B' => Self::PGain,
            'C' => Self::IGain,
            'X' => Self::AutoCalibrateSpeedFeedbackGain,
            _ => bail!("char '{}' is not a command", c),
        })
    }
}
