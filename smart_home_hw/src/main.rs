use smart_home::home::{Home, SmartHomeService};
use smart_home::rooms::devices;
use smart_home::rooms::devices::device::SmartDevice;
use smart_home::rooms::room::Room;

mod reports;
mod smart_home;

fn main() {
    // devices
    let plug_one =
        devices::plug::SmartPlug::new(String::from("plug_one"), 24, devices::plug::PlugStates::Off);

    let thermometer_one =
        devices::thermometer::SmartThermometer::new(String::from("thermo_one"), -22);

    let plug_two =
        devices::plug::SmartPlug::new(String::from("plug_two"), 24, devices::plug::PlugStates::Off);

    // rooms
    let small_room_devices: Vec<Box<dyn SmartDevice>> =
        vec![Box::new(plug_one), Box::new(thermometer_one)];

    let big_room_devices: Vec<Box<dyn SmartDevice>> = vec![Box::new(plug_two)];

    let small_room = Room::new(String::from("Small Room"), small_room_devices);

    let big_room = Room::new(String::from("Big Room"), big_room_devices);

    // home
    let smart_home = Home::new(String::from("Test Smart Home"), vec![small_room, big_room]);
    println!("This home {:?} is created", smart_home.name);

    let report = smart_home.device_report("thermo_one".to_string());
    println!("Full report here {:?}", report);
}
