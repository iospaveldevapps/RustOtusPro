use super::device::SmartDevice;

#[non_exhaustive]
enum ThermometerData {
    Temperature(i8),
}

impl ThermometerData {
    fn info(data: &ThermometerData) -> String {
        match data {
            ThermometerData::Temperature(temperature) => {
                format!("ThermometerData '{}'", temperature)
            }
        }
    }
}

trait Thermometer {
    fn provide(&self, data: Option<&ThermometerData>) -> String;
}

#[derive(Debug, Clone)]
pub struct SmartThermometer {
    pub name: String,
    temperature: i8,
}

impl SmartThermometer {
    pub fn new(name: String, temperature: i8) -> Self {
        SmartThermometer { name, temperature }
    }
}

impl Thermometer for SmartThermometer {
    fn provide(&self, data: Option<&ThermometerData>) -> String {
        println!(
            "Smart Thermometer Name: {}, Temperature: {}",
            self.name, self.temperature
        );

        if let Some(value) = data {
            ThermometerData::info(value)
        } else {
            let tmp_data = ThermometerData::Temperature(-20);
            ThermometerData::info(&tmp_data)
        }
    }
}

impl SmartDevice for SmartThermometer {
    fn name(&self) -> &String {
        &self.name
    }
}
