use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
trait Trigger: Send + Sync {
    async fn poll(&self) -> bool;
}

#[async_trait]
trait Action: Send + Sync {
    async fn act(&self) -> bool;
}

struct Connector {
    connections: Arc<Vec<(Arc<dyn Trigger>, Arc<dyn Action>)>>,
}

impl Connector {
    fn new() -> Self {
        Self {
            connections: Arc::new(Vec::new()),
        }
    }

    fn run(self) -> ! {
        let Connector { connections } = self;
        let cons = Arc::new(connections);
        loop {
            // Temporary value is dropped while borrowed, must have 'static lifetime
            for (trigger, action) in cons.clone().iter() {
                tokio::spawn(async move {
                    if trigger.poll().await {
                        action.act().await;
                    }
                });
            }
        }
    }
}
