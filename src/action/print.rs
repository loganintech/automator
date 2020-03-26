use crate::action::Action;
use async_trait::async_trait;
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

#[async_trait]
impl<'a, T: Debug + Send> Action<(), (), T> for StdOut {
    async fn act(&mut self, arg: T) -> Result<(), ()> {
        println!("{}{:?}", self.head, arg);
        Ok(())
    }
}
