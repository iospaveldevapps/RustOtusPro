use crate::home::rooms::devices::device::Device;
use crate::home::rooms::devices::device_states::DeviceStates;
use crate::home::rooms::room::Room;
use crate::Home;

#[test]
fn test_new_home() {
    let plug_one = Device::Plug {
        name: String::from("plug_one"),
        power: Some(24),
        state: DeviceStates::Off,
    };
    let room_devices = vec![plug_one];
    let room = Room::new(String::from("Small Room"), room_devices);
    let home = Home::new(String::from("Test Smart Home"), vec![room]);
    assert_eq!(home.name, "Test Smart Home");
}
