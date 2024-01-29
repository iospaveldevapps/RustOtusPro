use super::device::SmartDevice;

#[derive(Debug)]
#[non_exhaustive]
#[derive(Clone)]
pub enum PlugStates {
    _On,
    Off,
}

#[non_exhaustive]
enum PlugData {
    PowerConsumption(u8),
}

impl PlugData {
    fn info(data: &PlugData) -> String {
        match data {
            PlugData::PowerConsumption(power) => format!("Power Consumption is'{}'", power),
        }
    }
}

trait Plug {
    fn provide_description(&self) -> String;
    fn provide(&self, data: Option<&PlugData>) -> String;
    fn toggle(&mut self, state: PlugStates);
}

#[derive(Debug, Clone)]
pub struct SmartPlug {
    pub name: String,
    power: u8,
    state: PlugStates,
}

impl SmartPlug {
    pub fn new(name: String, power: u8, state: PlugStates) -> Self {
        SmartPlug { name, power, state }
    }
}

impl Plug for SmartPlug {
    fn provide_description(&self) -> String {
        format!(
            "Smart Plug Name: {}, Power: {}, State: {:?}",
            self.name, self.power, self.state
        )
    }

    fn provide(&self, data: Option<&PlugData>) -> String {
        if let Some(value) = data {
            PlugData::info(value)
        } else {
            let power_data = PlugData::PowerConsumption(24);
            PlugData::info(&power_data)
        }
    }

    fn toggle(&mut self, state: PlugStates) {
        self.state = state;
    }
}

impl SmartDevice for SmartPlug {
    fn name(&self) -> &String {
        &self.name
    }
}
