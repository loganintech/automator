use crate::action::Action;
use std::fmt::Debug;
#[derive(Default)]
pub struct StdOut {
    head: &'static str,
}

impl StdOut {
    pub fn new() -> StdOut {
        Self::default()
    }

    pub fn with_head(head: &'static str) -> Self {
        Self { head }
    }
}

impl<'a, T: Debug> Action<(), (), T> for StdOut {
    fn act(&mut self, arg: T) -> Result<(), ()> {
        println!("{}{:?}", self.head, arg);
        Ok(())
    }
}
