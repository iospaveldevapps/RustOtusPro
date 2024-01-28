use crate::home::rooms::devices::device::SmartDevice;
use crate::home::rooms::devices::plug::PlugStates;
use crate::home::rooms::devices::plug::SmartPlug;
use crate::home::rooms::room::Room;

#[test]
fn test_new_room() {
    let plug = SmartPlug::new(String::from("plug_one"), 24, PlugStates::Off);
    let room_devices: Vec<Box<dyn SmartDevice>> = vec![Box::new(plug)];
    let room = Room::new(String::from("Small Room"), room_devices);
    assert_eq!(room.devices.len(), 1);
}
