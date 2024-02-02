use super::rooms::devices::device_api::DeviceFieldData;
use super::rooms::room::Room;
use crate::devices::device::Device;
use crate::reports::report::{HomeReport, Report};
use std::collections::HashMap;

#[derive(Debug)]
pub enum SmartHomeServiceErrors {
    EmptyReportName(String),
    EmptyDeviceName(String),
    NoConnectedRooms(i8),
    NoConnectedDevices(i8),
    NoDeviceWithID(String),
}

pub trait SmartHomeService {
    fn report(&self, device_name: Option<String>) -> Result<Report, SmartHomeServiceErrors>;
    fn rooms(&self) -> &Vec<Room>;
    fn devices(&self, room: Room) -> Vec<Device>;
}

pub struct Home {
    pub name: String,
    pub rooms: Vec<Room>,
}

impl SmartHomeService for Home {
    fn rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    fn devices(&self, room: Room) -> Vec<Device> {
        room.devices
    }

    fn report(&self, device_name: Option<String>) -> Result<Report, SmartHomeServiceErrors> {
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

        self.check_device_by_id(device_name, room_devices_names);

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
        let mut wrapped_device_name = String::from("");

        if let Some(device) = device_name {
            wrapped_device_name = device
        }

        if !wrapped_device_name.is_empty() {
            if devices.contains(&wrapped_device_name) {
                println!("Device with ID {} is exist", wrapped_device_name);
            } else {
                let error = SmartHomeServiceErrors::NoDeviceWithID(wrapped_device_name.to_string());
                println!("No device with ID {:?}", error);
            }
        } else {
            let error = SmartHomeServiceErrors::EmptyDeviceName(wrapped_device_name.to_string());
            println!("Empty device ID {:?}", error);
        }
    }
}
