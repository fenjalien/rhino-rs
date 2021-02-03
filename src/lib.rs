mod command;
mod motor;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate anyhow;

pub use command::Command;
pub use motor::Motor;

/*
trait ServoMotorConfig{}

trait Command{}
trait Response{}

trait ServoMotor {
    fn from_config(config: dyn ServoMotorConfig) -> Self;
    fn transmit(self, command: Command);
    fn receive(self) -> Response;
}

mod Attribute {
    trait Position {
        fn get_position(self) -> i32;
        fn set_position(self, position: i32);
        fn goto_position(self, position: i32);
        fn goto_relative_position(self, relative_position: i32);
    }
    trait Speed {
        fn set_speed(self, speed: i32);
        fn get_speed(self) -> i32;
        fn set_max_speed(self, max_speed: i32);
        fn get_max_speed(self) -> i32
    }
}
*/
