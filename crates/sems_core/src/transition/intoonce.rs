use crate::{results::TransitionResult, TransitionParam};

use super::{SingleMarker, Transition, TransitionMut, TransitionOnce};

pub trait IntoTransitionOnce<In,Marker>
{
    fn into_transition_once(self) -> TransitionOnce<In>;
}

impl<In: 'static> IntoTransitionOnce<In,()> for Transition<In>
{
    fn into_transition_once(self) -> TransitionOnce<In> {
        TransitionOnce::new(move |args| {
            self.run(args);
        })
    }
}

impl<In: 'static> IntoTransitionOnce<In,()> for TransitionMut<In>
{
    fn into_transition_once(mut self) -> TransitionOnce<In> {
        TransitionOnce::new(move |args| {
            self.run(args);
        })
    }
}

impl<In> IntoTransitionOnce<In,()> for TransitionOnce<In>
{
    fn into_transition_once(self) -> TransitionOnce<In> {
        self
    }
}

impl<Res,Fun> IntoTransitionOnce<(),()> for Fun
where 
    Res: TransitionResult,
    Fun: FnOnce() -> Res + 'static
{
    fn into_transition_once(self) -> TransitionOnce<()> {
        TransitionOnce::new(move |args| {
            let res = self();
            res.insert_into(args);
        })
    }
}

impl<A,Res,Fun> IntoTransitionOnce<A,SingleMarker> for Fun
where 
    A: TransitionParam,
    Res: TransitionResult,
    Fun: FnOnce(A) -> Res + 'static
{
    fn into_transition_once(self) -> TransitionOnce<A> {
        TransitionOnce::new(move |args| {
            let a = A::take_from(args);
            let res = self(a);
            res.insert_into(args);
        })
    }
}

impl<A,B,Res,Fun> IntoTransitionOnce<(A,B),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    Res: TransitionResult,
    Fun: FnOnce(A,B) -> Res + 'static
{
    fn into_transition_once(self) -> TransitionOnce<(A,B)> {
        TransitionOnce::new(move |args| {
            let p = <(A,B)>::take_from(args);
            let res = self(p.0,p.1);
            res.insert_into(args);
        })
    }
}

impl<A,B,C,Res,Fun> IntoTransitionOnce<(A,B,C),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    Res: TransitionResult,
    Fun: FnOnce(A,B,C) -> Res + 'static
{
    fn into_transition_once(self) -> TransitionOnce<(A,B,C)> {
        TransitionOnce::new(move |args| {
            let p = <(A,B,C)>::take_from(args);
            let res = self(p.0,p.1,p.2);
            res.insert_into(args);
        })
    }
}

impl<A,B,C,D,Res,Fun> IntoTransitionOnce<(A,B,C,D),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    Res: TransitionResult,
    Fun: FnOnce(A,B,C,D) -> Res + 'static
{
    fn into_transition_once(self) -> TransitionOnce<(A,B,C,D)> {
        TransitionOnce::new(move |args| {
            let p = <(A,B,C,D)>::take_from(args);
            let res = self(p.0,p.1,p.2,p.3);
            res.insert_into(args);
        })
    }
}

impl<A,B,C,D,E,Res,Fun> IntoTransitionOnce<(A,B,C,D,E),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    E: TransitionParam,
    Res: TransitionResult,
    Fun: FnOnce(A,B,C,D,E) -> Res + 'static
{
    fn into_transition_once(self) -> TransitionOnce<(A,B,C,D,E)> {
        TransitionOnce::new(move |args| {
            let p = <(A,B,C,D,E)>::take_from(args);
            let res = self(p.0,p.1,p.2,p.3,p.4);
            res.insert_into(args);
        })
    }
}

impl<A,B,C,D,E,F,Res,Fun> IntoTransitionOnce<(A,B,C,D,E,F),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    E: TransitionParam,
    F: TransitionParam,
    Res: TransitionResult,
    Fun: FnOnce(A,B,C,D,E,F) -> Res + 'static
{
    fn into_transition_once(self) -> TransitionOnce<(A,B,C,D,E,F)> {
        TransitionOnce::new(move |args| {
            let p = <(A,B,C,D,E,F)>::take_from(args);
            let res = self(p.0,p.1,p.2,p.3,p.4,p.5);
            res.insert_into(args);
        })
    }
}

impl<A,B,C,D,E,F,G,Res,Fun> IntoTransitionOnce<(A,B,C,D,E,F,G),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    E: TransitionParam,
    F: TransitionParam,
    G: TransitionParam,
    Res: TransitionResult,
    Fun: FnOnce(A,B,C,D,E,F,G) -> Res + 'static
{
    fn into_transition_once(self) -> TransitionOnce<(A,B,C,D,E,F,G)> {
        TransitionOnce::new(move |args| {
            let p = <(A,B,C,D,E,F,G)>::take_from(args);
            let res = self(p.0,p.1,p.2,p.3,p.4,p.5,p.6);
            res.insert_into(args);
        })
    }
}

impl<A,B,C,D,E,F,G,H,Res,Fun> IntoTransitionOnce<(A,B,C,D,E,F,G,H),()> for Fun
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
    Fun: FnOnce(A,B,C,D,E,F,G,H) -> Res + 'static
{
    fn into_transition_once(self) -> TransitionOnce<(A,B,C,D,E,F,G,H)> {
        TransitionOnce::new(move |args| {
            let p = <(A,B,C,D,E,F,G,H)>::take_from(args);
            let res = self(p.0,p.1,p.2,p.3,p.4,p.5,p.6,p.7);
            res.insert_into(args);
        })
    }
}