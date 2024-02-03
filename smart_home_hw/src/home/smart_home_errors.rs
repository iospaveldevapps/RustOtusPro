#[derive(Debug)]
pub enum SmartHomeServiceErrors {
    NoConnectedRooms(i8),
    NoConnectedDevices(i8),
    NoDeviceWithID(String),
    EmptyReportName(String),
    DeviceAlreadyExists,
    DeviceNotFound,
    RoomNotFound,
    RoomIsNotRemoved,
}
