mod linux;
pub use self::linux::LinuxJoystick;

#[derive(Debug, PartialEq)]
pub enum Action {
    Pressed,
    Released,
}

#[derive(Debug, PartialEq)]
pub struct JoystickEvent {
    pub action: Action,
    pub button: u8,
}

pub trait Joystick {
    fn open(device_path: String) -> std::io::Result<Self>
    where
        Self: std::marker::Sized;

    fn read(&mut self) -> std::io::Result<JoystickEvent>;
}
