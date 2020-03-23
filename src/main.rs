use automator::actions::print::*;
use automator::triggers::interval::*;
use automator::*;

use std::time::Duration;

fn main() {
    let interval = Interval::with_duration(Duration::from_secs(5));
    let out = StdOut;
    let mut task: Task<(), Duration, (), Duration, Duration> = TaskBuilder::default()
        .with_trigger(interval)
        .with_action(out)
        .must_build();

    loop {
        if let Err(e) = task.check() {
            match e {
                ErrorSource::ActionError(_ae) => {
                    eprintln!("How the fuck did you fail a printout.");
                }
                ErrorSource::TriggerError(te) => {
                    println!("Timer has been running for {}ms.", te.as_millis());
                }
            }
        }
    }
}
