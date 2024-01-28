use super::rooms::devices::device::SmartDevice;
use super::rooms::room::Room;
use crate::reports::report::{HomeReport, Report};
use std::collections::HashMap;

pub trait SmartHomeService {
    fn rooms(&self) -> &Vec<Room>;
    fn devices(&self, room: Room) -> Vec<Box<dyn SmartDevice>>;
    fn device_report(&self, device_name: String) -> Report;
}

pub struct Home {
    pub name: String,
    pub rooms: Vec<Room>,
}

impl SmartHomeService for Home {
    fn rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    fn devices(&self, room: Room) -> Vec<Box<dyn SmartDevice>> {
        room.devices
    }

    fn device_report(&self, device_name: String) -> Report {
        let mut common_devices_count = 0;
        let mut devices_rooms_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut room_devices_names: Vec<String> = Vec::new();

        for room in &self.rooms {
            for device in &room.devices {
                common_devices_count += 1;
                let name = device.name();
                room_devices_names.push(name.to_string());
            }
            devices_rooms_map.insert(room.name.clone(), room_devices_names.clone());
        }

        if room_devices_names.contains(&device_name) {
            println!("Device with id: {} is exist!", device_name);
        } else {
            println!("Device with id: {} is not exist!", device_name);
        }

        Report::new(
            self.name.clone(),
            HomeReport::CommonReport {
                number_of_rooms: self.rooms.len(),
                common_count_of_devices: common_devices_count,
                room_devices: devices_rooms_map,
            },
        )
    }
}

impl Home {
    pub fn new(name: String, rooms: Vec<Room>) -> Self {
        Home { name, rooms }
    }
}
