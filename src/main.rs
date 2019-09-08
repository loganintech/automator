use async_trait::async_trait;
use automator::*;
use std::cell::RefCell;
use std::sync::Arc;
use std::time::{Duration, Instant};

fn main() {
    let mut con = Connector::new();
    // con.add_connection();

    con.run();
}

struct Clock {
    last: Arc<RefCell<Instant>>,
    interval: Arc<RefCell<Duration>>,
}

#[async_trait]
impl Trigger for Clock {
    async fn poll(&self) -> bool {
        let last = *self.last;
        let int = *self.interval;
        match (last.get_mut(), int.get_mut()) {
            (last, interval) => true,
            _ => false,
        }
    }
}
