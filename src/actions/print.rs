use crate::Action;
use std::fmt::Debug;
pub struct StdOut;

impl<'a, T: Debug> Action<(), (), T> for StdOut {
    fn act(&mut self, arg: T) -> Result<(), ()> {
        println!("{:?}", arg);
        Ok(())
    }
}
