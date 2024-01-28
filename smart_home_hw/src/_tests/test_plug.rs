use crate::home::rooms::devices::plug::PlugStates;
use crate::home::rooms::devices::plug::SmartPlug;

#[test]
fn test_new_plug() {
    let plug = SmartPlug::new(String::from("plug_one"), 24, PlugStates::Off);
    assert_eq!(plug.name, "plug_one");
}
