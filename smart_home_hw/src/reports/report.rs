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
    pub home_report: HomeReport,
}

impl Report {
    pub fn new(home_name: String, home_report: HomeReport) -> Self {
        Report {
            home_name,
            home_report,
        }
    }

    pub fn is_name_empty(&self) -> bool {
        let mut is_empty = false;

        if self.home_name.is_empty() {
            is_empty = true;
        }

        is_empty
    }
}
