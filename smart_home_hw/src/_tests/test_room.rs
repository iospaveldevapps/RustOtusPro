use crate::home::rooms::devices::device::Device;
use crate::home::rooms::devices::device_states::DeviceStates;
use crate::home::rooms::room::Room;

#[test]
fn test_new_room() {
    let thermometer_one = Device::Thermometer {
        name: String::from("thermo_one"),
        power: Some(16),
        temperature: Some(-22),
        state: DeviceStates::On,
    };

    let room_devices = vec![thermometer_one];
    let room = Room::new(String::from("Small Room"), room_devices);
    assert_eq!(room.devices.len(), 1);
}
