#[derive(Debug)]
pub enum DeviceErrors {
    NoDataToProvide(String),
    BrokenDevice(String),
}
