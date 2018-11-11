use std::fs::File;
use std::io::{Error, ErrorKind, Read};

use joystick::{Action, Joystick, JoystickEvent};

#[derive(Debug)]
struct Event {
    time: u32,
    value: i16,
    _type: u8,
    number: u8,
}

impl Event {
    fn is_init(&self) -> bool {
        self._type == 0x81 // JS_EVENT_BUTTON | JS_EVENT_INIT
    }
}

pub struct LinuxJoystick {
    buffer: Vec<u8>,
    fd: File,
}

impl LinuxJoystick {
    fn read_internal(&mut self) -> std::io::Result<Event> {
        self.fd.read_exact(&mut self.buffer)?;
        Ok(unsafe { std::ptr::read(self.buffer.as_ptr() as *const _) })
    }
}

impl Joystick for LinuxJoystick {
    fn open(device_path: String) -> std::io::Result<Self> {
        let mut joystick = LinuxJoystick {
            buffer: vec![0; std::mem::size_of::<Event>()],
            fd: File::open(device_path)?,
        };
        if joystick.read_internal()?.is_init() {
            Ok(joystick)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "read did not return the synthetic JS_EVENT_INIT event, check the device path you provided."))
        }
    }

    fn read(&mut self) -> std::io::Result<JoystickEvent> {
        let event = self.read_internal()?;
        Ok(event.into())
    }
}

impl From<Event> for JoystickEvent {
    fn from(event: Event) -> Self {
        JoystickEvent {
            action: if event.value == 1 {
                Action::Pressed
            } else {
                Action::Released
            },
            button: event.number,
        }
    }
}
