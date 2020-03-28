use crate::Action;

#[cfg(windows)]
use cpal;

#[cfg(unix)]
use strsim;

use std::io;
use std::process::Command;

#[cfg(windows)]
pub enum AudioDeviceType {
    Input,
    Output,
}

#[cfg(windows)]
pub struct SelectAudioDevice {
    device_type: AudioDeviceType,
}

#[cfg(windows)]
impl SelectAudioDevice {
    pub fn with_device_type(device_type: AudioDeviceType) -> Self {
        Self { device_type }
    }
}

#[cfg(unix)]
pub struct SelectAudioDevice;

impl<T: AsRef<str>> Action<(), io::Error, T> for SelectAudioDevice {
    #[cfg(windows)]
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

    #[cfg(unix)]
    fn act(&mut self, arg: T) -> Result<(), io::Error> {
        let arg: &str = arg.as_ref();

        let sinks = Command::new("pactl")
            .args(&["list", "sinks", "short"])
            .output()?
            .stdout;
        let sinks =
            String::from_utf8(sinks).map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;
        let sinks: Vec<[String; 2]> = sinks
            .split('\n')
            .map(|line| line.split('\t').map(String::from).collect::<Vec<String>>())
            .filter(|sl| sl.len() > 2)
            .map(|split_line| [split_line[0].to_string(), split_line[1].to_string()])
            .collect::<Vec<[String; 2]>>();

        let mut number_str = "".to_string();
        let mut highest_similarity = 0.0;
        let mut closest_sink = "".to_string();
        for [number, sink] in sinks {
            let similarity: f64 = strsim::sorensen_dice(&sink, arg);
            if highest_similarity < similarity {
                highest_similarity = similarity;
                closest_sink = sink.to_string();
                number_str = number.to_string();
            }
        }

        Command::new("pacmd")
            .args(&["set-default-sink", &closest_sink])
            .status()?;

        let sources = Command::new("pactl")
            .args(&["list", "short", "sink-inputs"])
            .output()?
            .stdout;
        let sources =
            String::from_utf8(sources).map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;
        let source_ids = sources
            .split('\n')
            .map(|line| line.split('\t').map(String::from).collect::<Vec<String>>())
            .filter(|sl| sl.len() > 1)
            .map(|split_line| split_line[0].to_string())
            .collect::<Vec<String>>();

        for id in source_ids {
            Command::new("pacmd")
                .args(&["move-sink-input", &id, &number_str])
                .status()
                .map(|_| ())?;
        }

        Ok(())
    }
}
