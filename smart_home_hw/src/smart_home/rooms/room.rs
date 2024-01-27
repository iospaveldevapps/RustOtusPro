use super::devices::device::SmartDevice;

pub struct Room {
    pub name: String,
    pub devices: Vec<Box<dyn SmartDevice>>,
}

impl Room {
    pub fn new(name: String, devices: Vec<Box<dyn SmartDevice>>) -> Self {
        Room { name, devices }
    }
}
