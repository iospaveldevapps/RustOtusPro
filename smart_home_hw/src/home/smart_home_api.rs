use super::rooms::room::Room;
use super::smart_home_errors::SmartHomeServiceErrors;
use crate::devices::device::Device;
use crate::reports::report::Report;

pub trait SmartHomeService {
    // device
    fn add_device(
        &mut self,
        room_name: &str,
        device: Device,
    ) -> Result<String, SmartHomeServiceErrors>;
    fn remove_device(
        &mut self,
        room_name: &str,
        device: &Device,
    ) -> Result<String, SmartHomeServiceErrors>;
    fn all_room_devices(&self, room: Room) -> Vec<Device>;

    // room
    fn add_room(&mut self, room: Room);
    fn remove_room(&mut self, room_name: &str) -> Result<String, SmartHomeServiceErrors>;
    fn all_rooms(&self) -> &Vec<Room>;

    // report
    fn report(&self, device_name: Option<String>) -> Result<Report, SmartHomeServiceErrors>;

    // helper
    fn get_mut_room(&mut self, room_name: &str) -> Option<&mut Room>;
}
