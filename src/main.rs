use automator::actions::{change_audio_device::*};
use automator::triggers::{discord_channel_change::*};
use automator::*;

use std::time::Duration;

fn main() {
    let task = TaskBuilder::new()
        .with_trigger(DiscordChannelChecker)
        .map(|d| if d == "" { "Speakers" } else { "Headphones" })
        .with_action(SelectAudioDevice::with_device_type(AudioDeviceType::Output))
        .must_build();

    let mut tasks = vec![task];
    loop {
        for task in &mut tasks {
            if let Err(e) = task.check() {
                match e {
                    ErrorSource::ActionError(_ae) => {
                        eprintln!("How the fuck did you fail a printout.");
                    }
                    ErrorSource::TriggerError(te) => {
                        println!("Trigger Error {:?}.", te);
                    }
                }
            }
        }

        std::thread::sleep(Duration::from_secs(1));
    }
}
