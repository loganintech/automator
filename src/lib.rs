pub mod actions;
pub mod triggers;

pub trait Action<T, E, A> {
    fn act(&mut self, arg: A) -> Result<T, E>;
}

pub trait Trigger<T, E> {
    fn check(&mut self) -> Result<T, E>;
}

// AT: Action Type
// TT: Trigger Type
// AE: Action Error
// TE: Trigger Error
// AA: Action Arg
pub struct Task<AT, TT, AE, TE, AA> {
    a: Box<dyn Action<AT, AE, AA>>,
    t: Box<dyn Trigger<TT, TE>>,
    conv: Box<dyn Fn(TT) -> AA>,
}

pub enum ErrorSource<AE, TE> {
    ActionError(AE),
    TriggerError(TE),
}

impl<'a, AT, TT, AE, TE, AA> Task<AT, TT, AE, TE, AA> {
    pub fn check(&'a mut self) -> Result<(), ErrorSource<AE, TE>> {
        let conv = &self.conv;
        match self.t.check() {
            Ok(ready) => match self.a.act(conv(ready)) {
                Ok(_) => Ok(()),
                Err(e) => Err(ErrorSource::ActionError(e)),
            },
            Err(e) => Err(ErrorSource::TriggerError(e)),
        }
    }
}

pub struct TaskBuilder<AT, TT, AE, TE, AA> {
    a: Option<Box<dyn Action<AT, AE, AA>>>,
    t: Option<Box<dyn Trigger<TT, TE>>>,
    conv: Option<Box<dyn Fn(TT) -> AA>>,
}

impl<AT, TT, AE, TE, AA> TaskBuilder<AT, TT, AE, TE, AA>
where
    AE: std::fmt::Debug,
    TE: std::fmt::Debug,
{
    pub fn with_action(mut self, a: impl Action<AT, AE, AA> + 'static) -> Self {
        self.a = Some(Box::new(a));
        self
    }

    pub fn with_trigger(mut self, t: impl Trigger<TT, TE> + 'static) -> Self {
        self.t = Some(Box::new(t));
        self
    }

    pub fn map(mut self, conv: impl Fn(TT) -> AA + 'static) -> Self {
        self.conv = Some(Box::new(conv));
        self
    }

    pub fn must_build(self) -> Task<AT, TT, AE, TE, AA> {
        if self.a.is_none() || self.t.is_none() || self.conv.is_none() {
            panic!("Couldn't build the task.");
        }

        Task {
            a: self.a.unwrap(),
            t: self.t.unwrap(),
            conv: self.conv.unwrap(),
        }
    }

    pub fn new() -> Self {
        Self {
            a: None,
            t: None,
            conv: None,
        }
    }
}
