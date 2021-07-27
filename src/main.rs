use rustmo_server::virtual_device::{VirtualDevice, VirtualDeviceError, VirtualDeviceState};
use rustmo_server::RustmoServer;
use std::env;
use std::net::Ipv4Addr;
use std::process::{Child, Command};
use std::str::FromStr;
use std::thread;

struct Device {
    process: Option<Child>,
    state: VirtualDeviceState,
}

impl Device {
    fn new() -> Self {
        Device {
            process: None,
            state: VirtualDeviceState::Off,
        }
    }
}

impl VirtualDevice for Device {
    fn turn_on(&mut self) -> Result<VirtualDeviceState, VirtualDeviceError> {
        let cwd = std::env::current_dir().unwrap().display().to_string();

        let iamnothacker = format!("{}/iamnothacker.exe", cwd);

        let proc = Command::new("cmd.exe")
            .arg(&iamnothacker)
            .arg("35")
            .spawn()
            .unwrap();

        self.process = Some(proc);
        self.state = VirtualDeviceState::On;
        Ok(self.state)
    }

    fn turn_off(&mut self) -> Result<VirtualDeviceState, VirtualDeviceError> {
        if self.process.is_some() {
            self.process.as_mut().unwrap().kill().unwrap();
            self.process = None;
        }
        self.state = VirtualDeviceState::Off;
        Ok(self.state)
    }

    fn check_is_on(&mut self) -> Result<VirtualDeviceState, VirtualDeviceError> {
        Ok(self.state)
    }
}

// Listening port
static PORT: u16 = 1100;

fn main() {
    let mut args = env::args();
    args.next();

    if let Some(ip) = args.next() {
        if let Some(name) = args.next() {
            let mut server = RustmoServer::new(Ipv4Addr::from_str(&ip).unwrap());

            server.add_device(&name, PORT, Device::new()).unwrap();

            thread::park();
        } else {
            println!("You must a device name");
        }
    } else {
        println!("You must specify an IP");
    }
}
