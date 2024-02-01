use super::device_data_types::DeviceDataTypes;
/// Used to group all type devices
use super::device_states::DeviceStates;

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

pub trait Device {
    fn full_info(&self) -> String;
    fn info_about(&self, data: &DeviceDataTypes);
    fn toggle(&mut self, state: DeviceStates);
}
