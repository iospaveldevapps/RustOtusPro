use super::device::Device;
use super::device_info_types::DeviceInfoTypes;
use super::device_states::DeviceStates;

pub trait DeviceStateManager {
    fn toggle(&mut self, new_state: DeviceStates);
}

pub trait DeviceInfo {
    fn full_info(device: &Device) -> String;
    fn info_about(device: &Device, data: &DeviceInfoTypes);
}

pub trait DeviceFieldData {
    fn name(&self) -> &String;
}
