use crate::trigger::Trigger;
use std::time::{Duration, Instant};

pub struct Timer {
    due: Instant,
    triggered: bool,
}

impl Timer {
    pub fn with_instant(i: Instant) -> Self {
        Self {
            due: i,
            triggered: false,
        }
    }

    pub fn with_duration(d: Duration) -> Self {
        let now = Instant::now();
        Self {
            due: now + d,
            triggered: (now + d) < now,
        }
    }
}

impl Trigger<Duration, Duration> for Timer {
    fn check(&mut self) -> Result<Duration, Duration> {
        let now = Instant::now();

        if !self.triggered && self.due < now {
            self.triggered = true;
            return Ok(now.duration_since(self.due));
        }
        if self.triggered {
            Err(now.duration_since(self.due))
        } else {
            Err(self.due.duration_since(now))
        }
    }
}
