use super::device_api::{DeviceFieldData, DeviceInfo, DeviceStateManager};
use super::device_errors::DeviceErrors;
use super::device_info_types::DeviceInfoTypes;
use super::device_states::DeviceStates;

#[derive(Clone, Debug, PartialEq)]
pub enum Device {
    Plug {
        name: String,
        power: Option<u8>,
        state: DeviceStates,
    },
    Thermometer {
        name: String,
        power: Option<u8>,
        temperature: Option<i8>,
        state: DeviceStates,
    },
}

impl DeviceInfo for Device {
    fn full_info(device: &Device) -> String {
        match device {
            Device::Plug { name, power, state } => {
                if let Some(power) = power {
                    format!(
                        "Smart Plug ID: {}, Power: {:?}, State: {:?}",
                        name, power, state
                    )
                } else {
                    format!(
                        "Smart Plug ID: {}, is : {:?}, and no any info about the device power!",
                        name, state
                    )
                }
            }
            Device::Thermometer {
                name,
                power,
                temperature,
                state,
            } => {
                if let (Some(power), Some(tmp)) = (power, temperature) {
                    format!(
                        "Thermometer ID: {}, Power: {:?}, Temperature: {:?}, State: {:?}",
                        name, power, tmp, state
                    )
                } else {
                    format!(
                      "Thermometer ID: {}, is : {:?}, and no any info about the device power and temperature!",
                      name, state
                  )
                }
            }
        }
    }

    fn info_about(device: &Device, data: &DeviceInfoTypes) {
        match data {
            DeviceInfoTypes::PowerConsumption => match device {
                Device::Plug { name, power, .. } => {
                    if let Some(power) = power {
                        println!("------");
                        println!("The power of the plug with ID {} is {}", name, power);
                        println!("------");
                    } else {
                        println!(
                              "{:?}",
                              DeviceErrors::BrokenDevice(format!(
                                  "The plug with ID {} can't provide the actual info about the device power!",
                                  name
                              ))
                          );
                    }
                }
                Device::Thermometer { name, power, .. } => {
                    if let Some(power) = power {
                        println!("------");
                        println!("The power of the thermometer with ID {} is {}", name, power);
                        println!("------");
                    } else {
                        println!(
                              "{:?}",
                              DeviceErrors::BrokenDevice(format!(
                                  "The thermometer with ID {} can't provide the actual info about the device power!",
                                  name
                              ))
                          );
                    }
                }
            },
            DeviceInfoTypes::ThermometerTemperature => match device {
                Device::Plug { .. } => {}
                Device::Thermometer {
                    name, temperature, ..
                } => {
                    if let Some(tmp) = temperature {
                        println!("------");
                        println!(
                            "The temperature of the thermometer with ID {} is {}",
                            name, tmp
                        );
                        println!("------");
                    } else {
                        println!(
                            "{:?}",
                            DeviceErrors::BrokenDevice(format!(
                                "The thermometer with ID {} can't provide the actual temperature",
                                name
                            ))
                        );
                    }
                }
            },
        }
    }
}

impl DeviceStateManager for Device {
    fn toggle(&mut self, new_state: DeviceStates) {
        match self {
            Device::Thermometer { state, .. } => {
                *state = new_state;
            }
            Device::Plug { state, .. } => {
                *state = new_state;
            }
        }
    }
}

impl DeviceFieldData for Device {
    fn name(&self) -> &String {
        match self {
            Device::Plug { name, .. } => name,
            Device::Thermometer { name, .. } => name,
        }
    }
}
