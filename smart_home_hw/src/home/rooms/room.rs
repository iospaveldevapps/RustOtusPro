use super::devices::device::Device;

#[derive(Debug)]
pub struct Room {
    pub name: String,
    pub devices: Vec<Device>,
}

impl Room {
    pub fn new(name: String, devices: Vec<Device>) -> Self {
        Room { name, devices }
    }
}

impl Clone for Room {
    fn clone(&self) -> Self {
        Room {
            name: self.name.clone(),
            devices: self.devices.clone(),
        }
    }
}
