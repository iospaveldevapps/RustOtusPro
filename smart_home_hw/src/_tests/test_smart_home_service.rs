use crate::home::rooms::devices::device::SmartDeviceClone;
use crate::home::rooms::devices::plug::{PlugStates, SmartPlug};
use crate::home::rooms::room::Room;
use crate::Home;
use crate::SmartHomeService;

#[test]
fn test_home_rooms() {
    let plug = SmartPlug::new(String::from("plug_one"), 24, PlugStates::Off);
    let room_devices: Vec<Box<dyn SmartDeviceClone>> = vec![Box::new(plug)];
    let room = Room::new(String::from("Small Room"), room_devices);
    let home = Home::new(String::from("Test Smart Home"), vec![room]);

    assert_eq!(home.rooms().len(), 1);
}

#[test]
fn test_home_device_report() {
    let device_name = &String::from("plug_one");
    let plug = SmartPlug::new(device_name.clone(), 24, PlugStates::Off);
    let room_devices: Vec<Box<dyn SmartDeviceClone>> = vec![Box::new(plug)];
    let room = Room::new(String::from("Small Room"), room_devices);
    let home = Home::new(String::from("Test Smart Home"), vec![room]);

    assert_eq!(home.device_report(device_name.clone()).home_name, home.name);
}

#[test]
fn test_home_devices() {
    let plug = SmartPlug::new(String::from("plug_one"), 24, PlugStates::Off);
    let room_devices: Vec<Box<dyn SmartDeviceClone>> = vec![Box::new(plug)];
    let room = Room::new(String::from("Small Room"), room_devices);
    let home = Home::new(String::from("Test Smart Home"), vec![room.clone()]);

    assert_eq!(home.devices(room).len(), 1);
}
