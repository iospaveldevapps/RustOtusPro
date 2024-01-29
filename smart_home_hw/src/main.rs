use devices::device::SmartDeviceClone;
use home::rooms::{devices, room::Room};
use home::smart_home::{Home, SmartHomeService};

pub mod home;
pub mod reports;

fn main() {
    // devices
    let plug_one =
        devices::plug::SmartPlug::new(String::from("plug_one"), 24, devices::plug::PlugStates::Off);

    let thermometer_one =
        devices::thermometer::SmartThermometer::new(String::from("thermo_one"), -22);

    let plug_two =
        devices::plug::SmartPlug::new(String::from("plug_two"), 24, devices::plug::PlugStates::Off);

    let small_room_devices: Vec<Box<dyn SmartDeviceClone>> =
        vec![Box::new(plug_one), Box::new(thermometer_one)];
    let big_room_devices: Vec<Box<dyn SmartDeviceClone>> = vec![Box::new(plug_two)];

    // rooms
    let small_room = Room::new(String::from("Small Room"), small_room_devices);
    let big_room = Room::new(String::from("Big Room"), big_room_devices);

    // home
    let smart_home = Home::new(String::from("Test Smart Home"), vec![small_room, big_room]);

    // report
    let report = smart_home.device_report("thermo_one".to_string());
    println!("-----------------------------------------------------------");
    println!("{:?}", report);
    println!("-----------------------------------------------------------");
}

#[cfg(test)]
mod _tests {
    mod test_home;
    mod test_plug;
    mod test_report;
    mod test_room;
    mod test_smart_home_service;
    mod test_thermometer;
}
