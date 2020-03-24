use crate::Action;
use cpal;

use std::io;
use std::process::Command;

pub enum AudioDeviceType {
    Input,
    Output,
}

pub struct SelectAudioDevice {
    device_type: AudioDeviceType,
}

impl SelectAudioDevice {
    pub fn with_device_type(device_type: AudioDeviceType) -> Self {
        Self { device_type }
    }
}

impl<T: AsRef<str>> Action<(), io::Error, T> for SelectAudioDevice {
    fn act(&mut self, arg: T) -> Result<(), io::Error> {
        let typestr = match self.device_type {
            AudioDeviceType::Output => {
                let device = match cpal::default_output_device() {
                    Some(device) => device,
                    None => return Err(io::Error::from(io::ErrorKind::InvalidData)),
                };

                if device.name().starts_with(arg.as_ref()) {
                    return Ok(());
                }

                "1"
            }
            AudioDeviceType::Input => {
                let device = match cpal::default_input_device() {
                    Some(device) => device,
                    None => return Err(io::Error::from(io::ErrorKind::InvalidData)),
                };

                if device.name().starts_with(arg.as_ref()) {
                    return Ok(());
                }

                "2"
            }
        };

        Command::new("nircmd")
            .args(&[
                "setdefaultsounddevice",
                arg.as_ref(),
                typestr, // Device type, Output
            ])
            .status()
            .map(|_| ())
    }
}
