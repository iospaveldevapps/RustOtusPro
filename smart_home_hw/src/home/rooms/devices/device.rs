/// Used to group all type devices
pub trait SmartDevice {
    fn name(&self) -> &String;
}

pub trait SmartDeviceClone: SmartDevice {
    fn clone_box(&self) -> Box<dyn SmartDeviceClone>;
}

impl<T> SmartDeviceClone for T
where
    T: 'static + SmartDevice + Clone,
{
    fn clone_box(&self) -> Box<dyn SmartDeviceClone> {
        Box::new(self.clone())
    }
}
