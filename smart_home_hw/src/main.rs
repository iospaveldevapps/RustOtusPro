use home::rooms::{devices, room::Room};
use home::smart_home::{Home, SmartHomeService};

use crate::devices::device::Device;
use crate::home::rooms::devices::device_api::DeviceInfo;
use crate::home::rooms::devices::device_info_types::DeviceInfoTypes;
use crate::home::rooms::devices::device_states::DeviceStates;

pub mod home;
pub mod reports;

fn main() {
    // devices
    let plug_one = Device::Plug {
        name: String::from("plug_one"),
        power: Some(24),
        state: DeviceStates::Off,
    };
    let plug_two = Device::Plug {
        name: String::from("plug_two"),
        power: Some(26),
        state: DeviceStates::Off,
    };
    let thermometer_one = Device::Thermometer {
        name: String::from("thermo_one"),
        power: Some(16),
        temperature: Some(-22),
        state: DeviceStates::On,
    };

    // get info about device
    Device::info_about(&thermometer_one, &DeviceInfoTypes::ThermometerTemperature);
    Device::info_about(&plug_two, &DeviceInfoTypes::PowerConsumption);

    // get full info
    Device::full_info(&plug_two);

    let small_room_devices: Vec<Device> = vec![plug_one, thermometer_one];
    let big_room_devices: Vec<Device> = vec![plug_two];

    // rooms
    let small_room = Room::new(String::from("Small Room"), small_room_devices);
    let big_room = Room::new(String::from("Big Room"), big_room_devices);

    // home
    let smart_home = Home::new(
        String::from("Test Smart Home"),
        vec![small_room, big_room.clone()],
    );

    // info about rooms and devices
    println!("Rooms:");
    println!("{:?}", smart_home.rooms());
    // info about devices in the room
    println!("All devices:");
    println!("{:?}", smart_home.devices(big_room));

    // report
    let report = smart_home.report(Some("thermo_one".to_string()));
    println!("-----------------------------------------------------------");
    println!("{:?}", report);
    println!("-----------------------------------------------------------");
}

#[cfg(test)]
mod _tests {
    mod test_device;
    mod test_home;
    mod test_report;
    mod test_room;
    mod test_smart_home_service;
}
