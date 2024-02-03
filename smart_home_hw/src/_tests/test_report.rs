use crate::home::rooms::devices::device::Device;
use crate::home::rooms::devices::device_states::DeviceStates;
use crate::home::rooms::room::Room;
use crate::home::smart_home_api::SmartHomeService;
use crate::Home;

#[test]
fn test_new_report_with_device_checking() {
    let device_name = &String::from("plug_one");
    let plug_one = Device::Plug {
        name: String::from("plug_one"),
        power: Some(24),
        state: DeviceStates::Off,
    };

    let room_devices = vec![plug_one];
    let room = Room::new(String::from("Small Room"), room_devices);
    let home = Home::new(String::from("Test Smart Home"), vec![room]);
    let report = home.report(Some(device_name.clone()));

    let unwrapped_report = report.expect("Failed to get report");
    assert_eq!(unwrapped_report.home_name, "Test Smart Home");
}

#[test]
fn test_new_report_without_device_checking() {
    let plug_one = Device::Plug {
        name: String::from("plug_one"),
        power: Some(24),
        state: DeviceStates::Off,
    };

    let room_devices = vec![plug_one];
    let room = Room::new(String::from("Small Room"), room_devices);
    let home = Home::new(String::from("Test Smart Home"), vec![room]);
    let report = home.report(None);

    let unwrapped_report = report.expect("Failed to get report");
    assert_eq!(unwrapped_report.home_name, "Test Smart Home");
}
