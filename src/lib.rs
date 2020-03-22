trait Action<'a, T, E> {
    fn action(&'a mut self) -> Result<T, E>;
}

trait Trigger<'a, T, E> {
    fn listen(&'a mut self) -> Result<T, E>;
}

// Action Type
// Trigger Type
// Action Error
// Trigger Error
struct Bridge<'a, AT, TT, AE, TE> {
    a: Box<dyn Action<'a, AT, AE>>,
    t: Box<dyn Trigger<'a, TT, TE>>,
}

struct BridgeBuilder<AT, TT, AE, TE, 'a> {
    a: Option<Box<dyn Action<'a, AT, AE>>>,
    t: Option<Box<dyn Trigger<'a, TT, TE>>>,
}

impl<'a, AT, TT, AE, TE> BridgeBuilder<'a, AT, TT, AE, TE> {
    fn new() -> Self {
        Self { a: None, t: None }
    }

    fn with_action(self, a: Box<dyn Action<'a, AT, AE>>) -> Self {
        self.a = Some(a);
        self
    }

    fn with_trigger(self, t: Box<dyn Trigger<'a, TT, TE>>) -> Self {
        self.t = Some(t);
        self
    }

    fn must_build(self) -> Bridge<'a, AT, TT, AE, TE> {
        if !self.a.is_some() || !self.t.is_some() {
            panic!("Couldn't build the bridge.");
        }

        Bridge {
            a: self.a.unwrap(),
            t: self.t.unwrap(),
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn try_something() {}
}
