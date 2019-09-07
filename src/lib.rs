pub trait Event {
    fn id() -> &'static str;
}

pub trait Provider {
    fn poll(&self) -> Option<String>;
}

pub trait Action {
    fn trigger(&mut self, data: String) -> bool;
}

#[derive(Default)]
pub struct Connector {
    connections: Vec<(Box<dyn Provider>, Box<dyn Action>)>,
}

impl Connector {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_connection(mut self, con: (Box<dyn Provider>, Box<dyn Action>)) -> Self {
        self.connections.push(con);
        self
    }

    pub fn run(&mut self) -> std::result::Result<(), &'static str> {
        loop {
            for (prov, act) in self.connections.iter_mut() {
                if let Some(data) = prov.poll() {
                    act.trigger(data);
                }
            }
        }
        Ok(())
    }
}
