use crate::{params::TransitionParam, results::TransitionResult, State};

pub struct Transition
{
    run: Box<dyn Fn(&mut State)>
}

impl Transition
{
    pub fn new<F>(run: F) -> Self
    where
        F: Fn(&mut State) + 'static
    {
        Transition {
            run: Box::new(run)
        }
    }

    pub fn run(&self, state: &mut State) {
        (self.run)(state);
    }
}

pub trait IntoTransition<In,Marker>
{
    fn into_transition(self) -> Transition;
}

impl<Res,Fun> IntoTransition<(),()> for Fun
where 
    Res: TransitionResult,
    Fun: Fn() -> Res + 'static
{
    fn into_transition(self) -> Transition {
        Transition::new(move |args| {
            let res = self();
            res.insert_into(args);
        })
    }
}

pub struct SingleMarker();

impl<A,Res,Fun> IntoTransition<A,SingleMarker> for Fun
where 
    A: TransitionParam,
    Res: TransitionResult,
    Fun: Fn(A) -> Res + 'static
{
    fn into_transition(self) -> Transition {
        Transition::new(move |args| {
            let a = A::take_from(args);
            let res = self(a);
            res.insert_into(args);
        })
    }
}

impl<A,B,Res,Fun> IntoTransition<(A,B),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    Res: TransitionResult,
    Fun: Fn(A,B) -> Res + 'static
{
    fn into_transition(self) -> Transition {
        Transition::new(move |args| {
            let a = A::take_from(args);
            let b = B::take_from(args);
            let res = self(a,b);
            res.insert_into(args);
        })
    }
}