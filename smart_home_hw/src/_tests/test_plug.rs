use crate::home::rooms::devices::device_states::DeviceStates;
use crate::home::rooms::devices::plug::SmartPlug;

#[test]
fn test_new_plug() {
    let plug = SmartPlug::new(String::from("plug_one"), Some(24), DeviceStates::Off);
    assert_eq!(plug.name, "plug_one");
}
