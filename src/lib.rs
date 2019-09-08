use async_trait::async_trait;
use fake_static::make_static;
use std::sync::Arc;

#[async_trait]
pub trait Trigger: Send + Sync {
    async fn poll(&self) -> bool;
}

#[async_trait]
pub trait Action: Send + Sync {
    async fn act(&self) -> bool;
}

#[derive(Default)]
pub struct Connector {
    connections: Vec<(Arc<dyn Trigger>, Arc<dyn Action>)>,
}

impl<'a> Connector {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_connection<T: Trigger + 'static, A: Action + 'static>(
        &mut self,
        trigger: T,
        action: A,
    ) {
        self.connections.push((Arc::new(trigger), Arc::new(action)));
    }

    pub fn run(self) -> ! {
        let Connector { connections } = self;
        let con = make_static(&connections);
        loop {
            // Temporary value is dropped while borrowed, must have 'static lifetime
            for (trigger, action) in con {
                tokio::spawn(async move {
                    if trigger.poll().await {
                        action.act().await;
                    }
                });
            }
        }
    }
}
