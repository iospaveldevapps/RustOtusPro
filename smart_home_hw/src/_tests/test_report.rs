use crate::home::rooms::devices::device::SmartDeviceClone;
use crate::home::rooms::devices::plug::{PlugStates, SmartPlug};
use crate::home::rooms::room::Room;
use crate::home::smart_home::SmartHomeService;
use crate::Home;

#[test]
fn test_new_report() {
    let device_name = &String::from("plug_one");
    let plug = SmartPlug::new(device_name.clone(), 24, PlugStates::Off);
    let room_devices: Vec<Box<dyn SmartDeviceClone>> = vec![Box::new(plug)];
    let room = Room::new(String::from("Small Room"), room_devices);
    let home = Home::new(String::from("Test Smart Home"), vec![room]);
    let report = home.device_report(device_name.clone());

    assert_eq!(report.home_name, "Test Smart Home");
}
