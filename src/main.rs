use automator::action::change_audio_device::*;
use automator::task::*;
use automator::trigger::discord_channel_change::*;

use std::time::Duration;

fn main() {
    let task = TaskBuilder::new()
        .with_trigger(DiscordChannelChecker)
        .map(|d| {
            if d == "" {
                "Speakers".to_string()
            } else {
                "Headphones".to_string()
            }
        })
        .with_action(SelectAudioDevice::with_device_type(AudioDeviceType::Output))
        .must_build();

    let mut tasks = vec![task];
    loop {
        for task in &mut tasks {
            if let Err(e) = task.check() {
                match e {
                    ErrorSource::ActionError(ae) => {
                        eprintln!("Action Error {:?}", ae);
                    }
                    ErrorSource::TriggerError(te) => {
                        eprintln!("Trigger Error {:?}.", te);
                    }
                }
            }
        }

        std::thread::sleep(Duration::from_secs(1));
    }
}
