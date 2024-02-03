use super::devices::device::Device;

pub trait SmartRoomService {
    fn all_devices(&self) -> &Vec<Device>;
}
