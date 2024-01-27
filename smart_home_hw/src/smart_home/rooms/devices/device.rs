/// Used to group all type devices
pub trait SmartDevice {
    fn exist_at_home(&self, cmp_device_name: &str) -> bool;
    fn name(&self) -> &String;
}
