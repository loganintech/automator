use automator::actions::{change_audio_device::*, print::*};
use automator::triggers::{discord_channel_change::*, interval::*, timer::*};
use automator::*;

use std::time::Duration;

fn main() {
    // let interval = Interval::with_duration(Duration::from_secs(5));
    // let interval2 =
    //     Interval::with_duration(Duration::from_secs(5)).with_offset(Duration::from_millis(2500));
    // let timer = Timer::with_duration(Duration::from_secs(3));
    // let mut task = TaskBuilder::default()
    //     .with_trigger(interval)
    //     .map(|d| (d))
    //     .with_action(StdOut::with_head("[Interval]: "))
    //     .must_build();
    // let mut task2 = TaskBuilder::default()
    //     .with_trigger(timer)
    //     .map(|d| d)
    //     .with_action(StdOut::with_head("[Timer]: "))
    //     .must_build();

    // let task_converted = Box::new(task);

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
