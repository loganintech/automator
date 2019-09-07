use automator::{Action, Connector, Provider};
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    let mut con = Connector::new().with_connection((Box::new(Hello), Box::new(Print)));
    let _ = con.run();
}

struct Hello;

impl Provider for Hello {
    fn poll(&self) -> Option<String> {
        Some("Hello World!".to_string())
    }
}

struct Print;

impl Action for Print {
    fn trigger(&mut self, data: String) -> bool {
        println!("{}", data);
        true
    }
}
