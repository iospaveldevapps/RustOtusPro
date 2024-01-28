use std::collections::HashMap;

#[derive(Debug)]
#[non_exhaustive]
pub enum HomeReport {
    CommonReport {
        number_of_rooms: usize,
        common_count_of_devices: usize,
        room_devices: HashMap<String, Vec<String>>,
    },
}

#[derive(Debug)]
pub struct Report {
    pub home_name: String,
    pub rooms_report: HomeReport,
}

impl Report {
    pub fn new(home_name: String, rooms_report: HomeReport) -> Self {
        Report {
            home_name,
            rooms_report,
        }
    }
}
