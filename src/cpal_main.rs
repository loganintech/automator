use cpal;
use std::process::Command;

pub enum AudioDeviceType {
    Input,
    Output,
}

fn set_default_audio_device(device_type: AudioDeviceType, name: String) -> String {
    let devices = cpal::devices();

    let comparison = |device_name| {
        println!("Checking {} against {}", device_name, name);
        device_name == name
    };

    if devices.map(|device| device.name()).any(comparison) {
        String::from("Audio device changed.")
    } else {
        String::from("idk")
    }
}

fn default_audio_device_type(device_type: AudioDeviceType) -> String {
    let device = match device_type {
        AudioDeviceType::Output => cpal::default_output_device(),
        AudioDeviceType::Input => cpal::default_input_device(),
    };

    match device {
        Some(device) => device.name(),
        _ => String::from("No supported default audio device."),
    }
}

fn audio_devices() -> String {
    let devices = cpal::devices();

    let mut devices_string: String = String::new();
    for device in devices {
        devices_string.push_str(format!("{}\n", device.name()).as_str());
    }

    devices_string
}

fn audio_device_type(device_type: AudioDeviceType) -> String {
    let devices = match device_type {
        AudioDeviceType::Output => cpal::output_devices(),
        AudioDeviceType::Input => cpal::input_devices(),
    };

    let mut devices_string: String = String::new();
    for device in devices {
        devices_string.push_str(format!("{}\n", device.name()).as_str());
    }

    devices_string
}

fn main() {
    Command::new("nircmd")
        .args(&[
            "setdefaultsounddevice",
            "Speakers",
            "1", // Device type, Output
        ])
        .status()
        .map(|_| ());

    println!("{}", audio_devices());
}
