use crate::home::rooms::devices::device::SmartDeviceClone;
use crate::home::rooms::devices::plug::{PlugStates, SmartPlug};
use crate::home::rooms::room::Room;
use crate::Home;

#[test]
fn test_new_home() {
    let plug = SmartPlug::new(String::from("plug_one"), 24, PlugStates::Off);
    let room_devices: Vec<Box<dyn SmartDeviceClone>> = vec![Box::new(plug)];
    let room = Room::new(String::from("Small Room"), room_devices);
    let home = Home::new(String::from("Test Smart Home"), vec![room]);
    assert_eq!(home.name, "Test Smart Home");
}
