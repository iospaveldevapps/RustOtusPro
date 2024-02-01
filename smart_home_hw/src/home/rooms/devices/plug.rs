use super::device::Device;
use super::device::SmartDevice;
use super::device_data_types::DeviceDataTypes;
use super::device_errors::DeviceErrors;
use super::device_states::DeviceStates;

#[derive(Debug, Clone)]
pub struct SmartPlug {
    pub name: String,
    power: Option<u8>,
    state: DeviceStates,
}

impl SmartPlug {
    pub fn new(name: String, power: Option<u8>, state: DeviceStates) -> Self {
        SmartPlug { name, power, state }
    }
}

impl Device for SmartPlug {
    fn full_info(&self) -> String {
        if let Some(power) = self.power {
            format!(
                "Smart Plug ID: {}, Power: {:?}, State: {:?}",
                self.name, power, self.state
            )
        } else {
            format!(
                "Smart Plug ID: {}, is : {:?}, and no any info about the device power!",
                self.name, self.state
            )
        }
    }

    fn info_about(&self, data: &DeviceDataTypes) {
        match data {
            DeviceDataTypes::PowerConsumption => {
                if let Some(power) = self.power {
                    println!("------");
                    println!("The power of the plug with ID {} is {}", self.name, power);
                    println!("------");
                } else {
                    println!(
                        "{:?}",
                        DeviceErrors::BrokenDevice(format!(
                            "The plug with ID {} can't provide the actual info about the device power!",
                            self.name
                        ))
                    );
                }
            }
            // will be fixed later
            DeviceDataTypes::ThermometerTemperature => {}
        }
    }

    fn toggle(&mut self, state: DeviceStates) {
        self.state = state;
    }
}

impl SmartDevice for SmartPlug {
    fn name(&self) -> &String {
        &self.name
    }
}
