pub mod change_audio_device;
pub mod print;

pub trait Action<T, E, A> {
    fn act(&mut self, arg: A) -> Result<T, E>;
}
