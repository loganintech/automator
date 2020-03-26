pub mod change_audio_device;
pub mod print;

use async_trait::async_trait;

#[async_trait]
pub trait Action<T, E, A> {
    async fn act(&mut self, arg: A) -> Result<T, E>;
}
