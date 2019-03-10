mod joystick;
use joystick::{Action, Joystick, LinuxJoystick};

use std::env;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let device_path = env::args()
        .skip(1)
        .next()
        .unwrap_or("/dev/input/js0".to_string());
    println!("Opening {}", device_path);

    let mut joystick = LinuxJoystick::open(device_path)?;
    loop {
        let event = joystick.read()?;
        println!("{:?}", event);
        if event.action == Action::Released {
            Command::new("systemctl")
                .arg("suspend")
                .spawn()
                .expect("failed to execute process");
        }
    }
}
