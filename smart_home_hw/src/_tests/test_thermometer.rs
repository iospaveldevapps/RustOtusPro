use crate::home::rooms::devices::thermometer::SmartThermometer;

#[test]
fn test_new_thermometer() {
    let thermometer = SmartThermometer::new(String::from("thermo_one"), -22);
    assert_eq!(thermometer.name, "thermo_one");
}
