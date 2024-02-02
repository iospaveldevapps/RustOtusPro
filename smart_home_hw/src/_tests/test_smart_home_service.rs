use crate::home::rooms::devices::device::Device;
use crate::home::rooms::devices::device_states::DeviceStates;
use crate::home::rooms::room::Room;
use crate::Home;
use crate::SmartHomeService;

#[test]
fn test_home_rooms() {
    let plug_one = Device::Plug {
        name: String::from("plug_one"),
        power: Some(24),
        state: DeviceStates::Off,
    };

    let room_devices = vec![plug_one];
    let room = Room::new(String::from("Small Room"), room_devices);
    let home = Home::new(String::from("Test Smart Home"), vec![room]);

    assert_eq!(home.rooms().len(), 1);
}

#[test]
fn test_home_device_report() {
    let device_name = &String::from("plug_one");
    let plug_one = Device::Plug {
        name: device_name.clone(),
        power: Some(24),
        state: DeviceStates::Off,
    };

    let room_devices = vec![plug_one];
    let room = Room::new(String::from("Small Room"), room_devices);
    let home = Home::new(String::from("Test Smart Home"), vec![room]);

    let unwrapped_report = home
        .report(Some(device_name.clone()))
        .expect("Failed to get report");
    assert_eq!(unwrapped_report.home_name, home.name);
}

#[test]
fn test_home_devices() {
    let plug_one = Device::Plug {
        name: String::from("plug_one"),
        power: Some(24),
        state: DeviceStates::Off,
    };

    let room_devices = vec![plug_one];
    let room = Room::new(String::from("Small Room"), room_devices);
    let home = Home::new(String::from("Test Smart Home"), vec![room.clone()]);

    assert_eq!(home.devices(room).len(), 1);
}
