use crate::home::rooms::devices::{device_states::DeviceStates, thermometer::SmartThermometer};

#[test]
fn test_new_thermometer() {
    let thermometer = SmartThermometer::new(
        String::from("thermo_one"),
        Some(12),
        Some(-22),
        DeviceStates::On,
    );
    assert_eq!(thermometer.name, "thermo_one");
}
