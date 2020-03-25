pub mod discord_channel_change;
pub mod interval;
pub mod timer;

pub trait Trigger<T, E> {
    fn check(&mut self) -> Result<T, E>;
}
