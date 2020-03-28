pub mod discord_channel_change;
pub mod interval;
pub mod timer;

use std::future::Future;

pub trait Trigger<T, E> {
    fn check(&mut self) -> Box<dyn Future<Output = Result<T, E>>>;
}
