use super::devices::device::SmartDeviceClone;

pub struct Room {
    pub name: String,
    pub devices: Vec<Box<dyn SmartDeviceClone>>,
}

impl Room {
    pub fn new(name: String, devices: Vec<Box<dyn SmartDeviceClone>>) -> Self {
        Room { name, devices }
    }
}

impl Clone for Room {
    fn clone(&self) -> Self {
        Room {
            name: self.name.clone(),
            devices: self.devices.iter().map(|d| d.clone_box()).collect(),
        }
    }
}
