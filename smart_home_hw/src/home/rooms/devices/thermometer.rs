use super::device::Device;
use super::device::SmartDevice;
use super::device_data_types::DeviceDataTypes;
use super::device_errors::DeviceErrors;
use super::device_states::DeviceStates;

#[derive(Debug, Clone)]
pub struct SmartThermometer {
    pub name: String,
    power: Option<u8>,
    temperature: Option<i8>,
    state: DeviceStates,
}

impl SmartThermometer {
    pub fn new(
        name: String,
        power: Option<u8>,
        temperature: Option<i8>,
        state: DeviceStates,
    ) -> Self {
        SmartThermometer {
            name,
            power,
            temperature,
            state,
        }
    }
}

impl Device for SmartThermometer {
    fn full_info(&self) -> String {
        if let (Some(power), Some(tmp)) = (self.power, self.temperature) {
            format!(
                "Thermometer ID: {}, Power: {:?}, Temperature: {:?}, State: {:?}",
                self.name, power, tmp, self.state
            )
        } else {
            format!(
                "Thermometer ID: {}, is : {:?}, and no any info about the device power and temperature!",
                self.name, self.state
            )
        }
    }

    fn info_about(&self, data: &DeviceDataTypes) {
        match data {
            DeviceDataTypes::ThermometerTemperature => {
                if let Some(tmp) = self.temperature {
                    println!("------");
                    println!(
                        "The temperature of the thermometer with ID {} is {}",
                        self.name, tmp
                    );
                    println!("------");
                } else {
                    println!(
                        "{:?}",
                        DeviceErrors::BrokenDevice(format!(
                            "The thermometer with ID {} can't provide the actual temperature",
                            self.name
                        ))
                    );
                }
            }
            DeviceDataTypes::PowerConsumption => {
                if let Some(power) = self.power {
                    println!("------");
                    println!(
                        "The power of the thermometer with ID {} is {}",
                        self.name, power
                    );
                    println!("------");
                } else {
                    println!(
                        "{:?}",
                        DeviceErrors::BrokenDevice(format!(
                            "The thermometer with ID {} can't provide the actual info about the device power!",
                            self.name
                        ))
                    );
                }
            }
        }
    }

    fn toggle(&mut self, state: DeviceStates) {
        self.state = state;
    }
}

impl SmartDevice for SmartThermometer {
    fn name(&self) -> &String {
        &self.name
    }
}
