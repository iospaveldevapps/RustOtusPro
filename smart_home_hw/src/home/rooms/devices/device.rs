/// Used to group all type devices
pub trait SmartDevice {
    fn name(&self) -> &String;
}
