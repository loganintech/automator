use automator::*;
use std::time::{Duration, Instant};

fn main() {
    let mut con = Connector::new();
    con.add_connection(Timer::new(Duration::from_secs(5)), HelloWorld {});
    con.add_connection(Timer::new(Duration::from_millis(250)), Counter { cnt: 0 });

    con.run();
}

struct Timer {
    last: Instant,
    interval: Duration,
}

impl Timer {
    fn new(interval: Duration) -> Self {
        Self {
            last: Instant::now(),
            interval,
        }
    }
}

impl Trigger for Timer {
    fn poll(&mut self) -> bool {
        if Instant::now().duration_since(self.last) > self.interval {
            self.last = Instant::now();
            return true;
        }

        false
    }
}

struct HelloWorld;

impl Action for HelloWorld {
    fn act(&mut self) -> bool {
        println!("Hello, World!");
        true
    }
}

struct Counter {
    cnt: usize,
}

impl Action for Counter {
    fn act(&mut self) -> bool {
        self.cnt = self.cnt.wrapping_add(1);
        println!("{}", self.cnt);
        true
    }
}
