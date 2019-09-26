use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub trait Action {
    fn key(&self) -> &str;
    fn trigger(&mut self, data: String) -> bool;
}

#[derive(Default)]
pub struct Actions<'a> {
    actions: HashMap<&'a str, Rc<dyn Action>>,
}

impl<'a> Actions<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_action<T: Action + 'a>(&mut self, action: T) -> &mut Self {
        let act = Rc::new(action);
        let weak = act.clone();
        let weak = Rc::downgrade(&weak);
        self.actions.insert(weak.key(), act);
        self
    }

    pub fn run(&mut self) -> std::result::Result<(), &'static str> {
        Ok(())
    }
}


