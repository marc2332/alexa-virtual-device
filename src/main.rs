use rustmo_server::virtual_device::{VirtualDevice, VirtualDeviceError, VirtualDeviceState};
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::thread;
use rustmo_server::RustmoServer;
use std::process::{
    Command,
    Child,
};

struct MatrixDevice {
    process: Option<Child>,
    state: VirtualDeviceState
}

impl MatrixDevice {
    fn new() -> Self {
        MatrixDevice{
            process: None,
            state: VirtualDeviceState::Off
        }
    }
}

impl VirtualDevice for MatrixDevice {
    fn turn_on(&mut self) -> Result<VirtualDeviceState, VirtualDeviceError> {
        let cwd = std::env::current_dir().unwrap().display().to_string();

        let iamnothacker = format!("{}/iamnothacker.exe", cwd);

        let mut proc = Command::new("cmd.exe")
            .arg(&iamnothacker)
            .arg("35").spawn().unwrap();

        self.process = Some(proc);
        self.state = VirtualDeviceState::On;
        Ok(self.state)
    }

    fn turn_off(&mut self) -> Result<VirtualDeviceState, VirtualDeviceError> {
        if self.process.is_some() {
            self.process.as_mut().unwrap().kill();
            self.process = None;
        }
        self.state = VirtualDeviceState::Off;
        Ok(self.state)
    }

    fn check_is_on(&mut self) -> Result<VirtualDeviceState, VirtualDeviceError> {
        Ok(self.state)
    }
}


fn main() -> std::io::Result<()> {

    let mut server = RustmoServer::new(Ipv4Addr::from_str("192.168.1.157").unwrap());

    server.add_device("caracola", 1100, MatrixDevice::new()).unwrap();

    thread::park();
    Ok(())
}
