use home::rooms::{devices, room::Room};
use home::smart_home::Home;
use home::smart_home_api::SmartHomeService;

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

    let thermometer_two = Device::Thermometer {
        name: String::from("thermo_two"),
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
    let big_room_devices: Vec<Device> = vec![plug_two, thermometer_two];

    // rooms
    let small_room = Room::new(String::from("Small Room"), small_room_devices);
    let big_room = Room::new(String::from("Big Room"), big_room_devices);

    // home
    let mut smart_home = Home::new(
        String::from("Test Smart Home"),
        vec![small_room, big_room.clone()],
    );

    // info about rooms and devices
    _ = smart_home.all_rooms();
    _ = smart_home.all_room_devices(big_room);

    // add room and device for that room
    let orange_room_plug = Device::Plug {
        name: String::from("orange_plug"),
        power: Some(26),
        state: DeviceStates::Off,
    };

    let orange_room = Room::new(String::from("Orange Room"), vec![]);
    smart_home.add_room(orange_room);

    match smart_home.add_device(
        String::from("Orange Room").as_str(),
        orange_room_plug.clone(),
    ) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("{:?}", e),
    }

    // remove device and room (uncommite to check)
    // match smart_home.remove_device(String::from("Orange Room").as_str(), &orange_room_plug) {
    //     Ok(msg) => println!("{}", msg),
    //     Err(e) => println!("{:?}", e),
    // }

    // If the room will be deletead separatly, all devices in this room woll be deleted too
    // match smart_home.remove_room(String::from("Orange Room").as_str()) {
    //     Ok(msg) => println!("{}", msg),
    //     Err(e) => println!("{:?}", e),
    // }

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
