use crate::devices::device::Device;
use crate::devices::device_states::DeviceStates;
use crate::home::rooms::devices::device_api::DeviceFieldData;

#[test]
fn test_new_thermometer() {
    let thermometer_one = Device::Thermometer {
        name: String::from("thermo_one"),
        power: Some(16),
        temperature: Some(-22),
        state: DeviceStates::On,
    };
    assert_eq!(thermometer_one.name(), "thermo_one");
}
