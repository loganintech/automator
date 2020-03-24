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

    pub fn with_offset(mut self, d: Duration) -> Self {
        self.prev += d;
        self
    }
}

impl Trigger<Duration, Duration> for Interval {
    fn check(&mut self) -> Result<Duration, Duration> {
        let now = Instant::now();
        if now < self.prev {
            return Err(self.prev.duration_since(now));
        }
        let elapsed = now.duration_since(self.prev);
        if elapsed > self.d {
            self.prev = now;
            return Ok(elapsed);
        }

        Err(elapsed)
    }
}
