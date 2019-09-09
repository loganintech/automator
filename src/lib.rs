use std::cell::RefCell;
use std::time::{Duration, Instant};

pub trait Trigger {
    fn poll(&mut self) -> bool;
}

pub trait Action {
    fn act(&mut self) -> bool;
}

type DynamicTrigger = Box<RefCell<dyn Trigger>>;
type DynamicAction = Box<RefCell<dyn Action>>;

#[derive(Default)]
pub struct Connector {
    connections: Vec<(DynamicTrigger, DynamicAction)>,
}

impl Connector {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_connection<T: Trigger + 'static, A: Action + 'static>(
        &mut self,
        trigger: T,
        action: A,
    ) {
        self.connections.push((
            Box::new(RefCell::new(trigger)),
            Box::new(RefCell::new(action)),
        ));
    }

    pub fn run(self) -> ! {
        let mut connections = self.connections;
        let mut idx = 0;
        loop {
            if idx == connections.len() - 1 {
                idx = 0;
            } else {
                idx += 1;
            }
            let (trigger, action) = &mut connections[idx];

            if trigger.get_mut().poll() {
                action.get_mut().act();
            }
        }
    }
}

pub struct Interval {
    last: Instant,
    interval: Duration,
}

impl Interval {
    pub fn new(interval: Duration) -> Self {
        Self {
            last: Instant::now(),
            interval,
        }
    }
}

impl Trigger for Interval {
    fn poll(&mut self) -> bool {
        if Instant::now().duration_since(self.last) > self.interval {
            self.last = Instant::now();
            return true;
        }

        false
    }
}

pub struct DebugAction;

impl Action for DebugAction {
    fn act(&mut self) -> bool {
        println!("Debug");
        true
    }
}
