use crate::{results::TransitionResult, TransitionParam};

use super::{SingleMarker, Transition, TransitionMut};

pub trait IntoTransitionMut<In,Marker>
{
    fn into_transition_mut(self) -> TransitionMut<In>;
}

impl<In: 'static> IntoTransitionMut<In,()> for Transition<In>
{
    fn into_transition_mut(self) -> TransitionMut<In> {
        TransitionMut::new(move |args| {
            self.run(args);
        })
    }
}

impl<In> IntoTransitionMut<In,()> for TransitionMut<In>
{
    fn into_transition_mut(self) -> TransitionMut<In> {
        self
    }
}

impl<Res,Fun> IntoTransitionMut<(),()> for Fun
where 
    Res: TransitionResult,
    Fun: FnMut() -> Res + 'static
{
    fn into_transition_mut(mut self) -> TransitionMut<()> {
        TransitionMut::new(move |args| {
            let res = self();
            res.insert_into(args);
        })
    }
}

impl<A,Res,Fun> IntoTransitionMut<A,SingleMarker> for Fun
where 
    A: TransitionParam,
    Res: TransitionResult,
    Fun: FnMut(A) -> Res + 'static
{
    fn into_transition_mut(mut self) -> TransitionMut<A> {
        TransitionMut::new(move |args| {
            let a = A::take_from(args);
            let res = self(a);
            res.insert_into(args);
        })
    }
}

impl<A,B,Res,Fun> IntoTransitionMut<(A,B),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    Res: TransitionResult,
    Fun: FnMut(A,B) -> Res + 'static
{
    fn into_transition_mut(mut self) -> TransitionMut<(A,B)> {
        TransitionMut::new(move |args| {
            let p = <(A,B)>::take_from(args);
            let res = self(p.0,p.1);
            res.insert_into(args);
        })
    }
}

impl<A,B,C,Res,Fun> IntoTransitionMut<(A,B,C),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    Res: TransitionResult,
    Fun: FnMut(A,B,C) -> Res + 'static
{
    fn into_transition_mut(mut self) -> TransitionMut<(A,B,C)> {
        TransitionMut::new(move |args| {
            let p = <(A,B,C)>::take_from(args);
            let res = self(p.0,p.1,p.2);
            res.insert_into(args);
        })
    }
}

impl<A,B,C,D,Res,Fun> IntoTransitionMut<(A,B,C,D),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    Res: TransitionResult,
    Fun: FnMut(A,B,C,D) -> Res + 'static
{
    fn into_transition_mut(mut self) -> TransitionMut<(A,B,C,D)> {
        TransitionMut::new(move |args| {
            let p = <(A,B,C,D)>::take_from(args);
            let res = self(p.0,p.1,p.2,p.3);
            res.insert_into(args);
        })
    }
}

impl<A,B,C,D,E,Res,Fun> IntoTransitionMut<(A,B,C,D,E),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    E: TransitionParam,
    Res: TransitionResult,
    Fun: FnMut(A,B,C,D,E) -> Res + 'static
{
    fn into_transition_mut(mut self) -> TransitionMut<(A,B,C,D,E)> {
        TransitionMut::new(move |args| {
            let p = <(A,B,C,D,E)>::take_from(args);
            let res = self(p.0,p.1,p.2,p.3,p.4);
            res.insert_into(args);
        })
    }
}

impl<A,B,C,D,E,F,Res,Fun> IntoTransitionMut<(A,B,C,D,E,F),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    E: TransitionParam,
    F: TransitionParam,
    Res: TransitionResult,
    Fun: FnMut(A,B,C,D,E,F) -> Res + 'static
{
    fn into_transition_mut(mut self) -> TransitionMut<(A,B,C,D,E,F)> {
        TransitionMut::new(move |args| {
            let p = <(A,B,C,D,E,F)>::take_from(args);
            let res = self(p.0,p.1,p.2,p.3,p.4,p.5);
            res.insert_into(args);
        })
    }
}

impl<A,B,C,D,E,F,G,Res,Fun> IntoTransitionMut<(A,B,C,D,E,F,G),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    E: TransitionParam,
    F: TransitionParam,
    G: TransitionParam,
    Res: TransitionResult,
    Fun: FnMut(A,B,C,D,E,F,G) -> Res + 'static
{
    fn into_transition_mut(mut self) -> TransitionMut<(A,B,C,D,E,F,G)> {
        TransitionMut::new(move |args| {
            let p = <(A,B,C,D,E,F,G)>::take_from(args);
            let res = self(p.0,p.1,p.2,p.3,p.4,p.5,p.6);
            res.insert_into(args);
        })
    }
}

impl<A,B,C,D,E,F,G,H,Res,Fun> IntoTransitionMut<(A,B,C,D,E,F,G,H),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    E: TransitionParam,
    F: TransitionParam,
    G: TransitionParam,
    H: TransitionParam,
    Res: TransitionResult,
    Fun: FnMut(A,B,C,D,E,F,G,H) -> Res + 'static
{
    fn into_transition_mut(mut self) -> TransitionMut<(A,B,C,D,E,F,G,H)> {
        TransitionMut::new(move |args| {
            let p = <(A,B,C,D,E,F,G,H)>::take_from(args);
            let res = self(p.0,p.1,p.2,p.3,p.4,p.5,p.6,p.7);
            res.insert_into(args);
        })
    }
}