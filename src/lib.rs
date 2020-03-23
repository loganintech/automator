pub mod actions;
pub mod triggers;

pub trait Action<T, E, A> {
    fn act(&mut self, arg: A) -> Result<T, E>;
}

pub trait Trigger<T, E> {
    fn check(&mut self) -> Result<T, E>;
}

// Action Type
// Trigger Type
// Action Error
// Trigger Error
pub struct Task<AT, TT, AE, TE, AA> {
    a: Box<dyn Action<AT, AE, AA>>,
    t: Box<dyn Trigger<TT, TE>>,
}

pub enum ErrorSource<AE, TE> {
    ActionError(AE),
    TriggerError(TE),
}

impl<'a, AT, TT, AE, TE, AA> Task<AT, TT, AE, TE, AA>
where
    TT: Into<AA>,
{
    pub fn check(&'a mut self) -> Result<(), ErrorSource<AE, TE>> {
        match self.t.check() {
            Ok(ready) => match self.a.act(ready.into()) {
                Ok(_) => Ok(()),
                Err(e) => Err(ErrorSource::ActionError(e)),
            },
            Err(e) => Err(ErrorSource::TriggerError(e)),
        }
    }
}

#[derive(Default)]
pub struct TaskBuilder<AT, TT, AE, TE, AA> {
    a: Option<Box<dyn Action<AT, AE, AA>>>,
    t: Option<Box<dyn Trigger<TT, TE>>>,
}

impl<AT, TT, AE, TE, AA> TaskBuilder<AT, TT, AE, TE, AA>
where
    TT: Into<AA>,
{
    pub fn with_action(mut self, a: impl Action<AT, AE, AA> + 'static) -> Self {
        self.a = Some(Box::new(a));
        self
    }

    pub fn with_trigger(mut self, t: impl Trigger<TT, TE> + 'static) -> Self {
        self.t = Some(Box::new(t));
        self
    }

    pub fn must_build(self) -> Task<AT, TT, AE, TE, AA> {
        if self.a.is_none() || self.t.is_none() {
            panic!("Couldn't build the task.");
        }

        Task {
            a: self.a.unwrap(),
            t: self.t.unwrap(),
        }
    }
}
