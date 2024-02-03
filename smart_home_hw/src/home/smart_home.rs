use super::rooms::devices::device_api::DeviceFieldData;
use super::rooms::room::Room;
use super::smart_home_api::SmartHomeService;
use super::smart_home_errors::SmartHomeServiceErrors;
use crate::devices::device::Device;
use crate::reports::report::{HomeReport, Report};
use std::collections::HashMap;

pub struct Home {
    pub name: String,
    pub rooms: Vec<Room>,
}

// devices
impl SmartHomeService for Home {
    fn add_device(
        &mut self,
        room_name: &str,
        device: Device,
    ) -> Result<String, SmartHomeServiceErrors> {
        if let Some(room) = self.get_mut_room(room_name) {
            if room.devices.contains(&device) {
                return Err(SmartHomeServiceErrors::DeviceAlreadyExists);
            } else {
                room.devices.push(device.clone());
                return Ok(format!("Device added to {}", room.name));
            }
        }
        Err(SmartHomeServiceErrors::RoomNotFound)
    }

    fn remove_device(
        &mut self,
        room_name: &str,
        device: &Device,
    ) -> Result<String, SmartHomeServiceErrors> {
        if let Some(room) = self.rooms.iter_mut().find(|r| r.name == room_name) {
            match room.devices.iter().position(|d| d == device) {
                Some(index) => {
                    room.devices.remove(index);
                    Ok(format!("Device removed from {}", room.name))
                }
                None => Err(SmartHomeServiceErrors::DeviceNotFound),
            }
        } else {
            Err(SmartHomeServiceErrors::RoomNotFound)
        }
    }

    fn remove_room(&mut self, room_name: &str) -> Result<String, SmartHomeServiceErrors> {
        if let Some(index) = self.rooms.iter().position(|x| x.name == room_name) {
            let room = self.rooms.swap_remove(index);
            Ok(format!(
                "Room with ID {} is successfully removed",
                room.name
            ))
        } else {
            Err(SmartHomeServiceErrors::RoomNotFound)
        }
    }

    fn get_mut_room(&mut self, room_name: &str) -> Option<&mut Room> {
        self.rooms.iter_mut().find(|room| room.name == room_name)
    }

    fn all_rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }

    fn all_room_devices(&self, room: Room) -> Vec<Device> {
        room.devices
    }

    fn report(&self, device_name: Option<String>) -> Result<Report, SmartHomeServiceErrors> {
        let mut common_devices_count = 0;
        let mut devices_rooms_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut all_devices_names: Vec<String> = vec![];
        let mut room_devices_names: Vec<String> = vec![];

        for room in &self.rooms {
            for device in &room.devices {
                common_devices_count += 1;
                let name = device.name();
                room_devices_names.push(name.to_string());
                all_devices_names.push(name.to_string());
            }
            devices_rooms_map.insert(room.name.clone(), room_devices_names.clone());
            room_devices_names = vec![];
        }

        self.check_device_by_id(device_name, all_devices_names);

        let report = Report::new(
            self.name.clone(),
            HomeReport::CommonReport {
                number_of_rooms: self.rooms.len(),
                common_count_of_devices: common_devices_count,
                room_devices: devices_rooms_map,
            },
        );

        if report.is_name_empty() {
            return Err(SmartHomeServiceErrors::EmptyReportName(
                "Please, check the report name!".to_string(),
            ));
        }

        if self.rooms.is_empty() {
            return Err(SmartHomeServiceErrors::NoConnectedRooms(0));
        }

        if common_devices_count == 0 {
            return Err(SmartHomeServiceErrors::NoConnectedDevices(0));
        }

        Ok(report)
    }
}

impl Home {
    pub fn new(name: String, rooms: Vec<Room>) -> Self {
        Home { name, rooms }
    }

    fn check_device_by_id(&self, device_name: Option<String>, devices: Vec<String>) {
        if let Some(device) = device_name {
            if devices.contains(&device) {
                println!("Device with ID {} is exist", device);
            } else {
                let error = SmartHomeServiceErrors::NoDeviceWithID(device.to_string());
                println!("{:?}", error);
            }
        }
    }
}
