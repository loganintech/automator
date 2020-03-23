use crate::Trigger;
use std::time::{Duration, Instant};

pub struct Interval {
    d: Duration,
    prev: Instant,
}

impl Interval {
    pub fn with_duration(d: Duration) -> Self {
        Interval {
            d,
            prev: Instant::now(),
        }
    }
}

impl Trigger<Duration, Duration> for Interval {
    fn check(&mut self) -> Result<Duration, Duration> {
        let elapsed = Instant::now().duration_since(self.prev);
        if elapsed > self.d {
            self.prev = Instant::now();
            return Ok(elapsed);
        }

        Err(elapsed)
    }
}
