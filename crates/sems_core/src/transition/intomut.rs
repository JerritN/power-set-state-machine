use super::{params::TransitionParam, results::TransitionResult, SingleMarker, Transition, TransitionMut, UnknownParameter};

pub trait IntoTransitionMut<In,Marker>
{
    fn into_transition_mut(self) -> Result<TransitionMut,&'static str>;
}

impl IntoTransitionMut<UnknownParameter,()> for Transition
{
    fn into_transition_mut(self) -> Result<TransitionMut,&'static str> {
        Ok(TransitionMut::new(
            self.func,
            self.requires
        ))
    }
}

impl IntoTransitionMut<UnknownParameter,()> for TransitionMut
{
    fn into_transition_mut(self) -> Result<TransitionMut,&'static str> {
        Ok(self)
    }
}

impl<Res,Fun> IntoTransitionMut<(),()> for Fun
where 
    Res: TransitionResult,
    Fun: FnMut() -> Res + 'static
{
    fn into_transition_mut(mut self) -> Result<TransitionMut,&'static str> {
        Ok(TransitionMut::new(
            move |args| {
                let res = self();
                res.insert_into(args);
            },
            <()>::required()?
        ))
    }
}

impl<A,Res,Fun> IntoTransitionMut<A,SingleMarker> for Fun
where 
    A: TransitionParam,
    Res: TransitionResult,
    Fun: FnMut(A) -> Res + 'static
{
    fn into_transition_mut(mut self) -> Result<TransitionMut,&'static str> {
        Ok(TransitionMut::new(
            move |args| {
                let p = <A>::take_from(args);
                let res = self(p);
                res.insert_into(args);
            },
            A::required()?
        ))
    }
}

impl<A,B,Res,Fun> IntoTransitionMut<(A,B),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    Res: TransitionResult,
    Fun: FnMut(A,B) -> Res + 'static
{
    fn into_transition_mut(mut self) -> Result<TransitionMut,&'static str> {
        Ok(TransitionMut::new(
            move |args| {
                let (p1,p2) = <(A,B)>::take_from(args);
                let res = self(p1,p2);
                res.insert_into(args);
            },
            <(A,B)>::required()?
        ))
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
    fn into_transition_mut(mut self) -> Result<TransitionMut,&'static str> {
        Ok(TransitionMut::new(
            move |args| {
                let (p1,p2,p3) = <(A,B,C)>::take_from(args);
                let res = self(p1,p2,p3);
                res.insert_into(args);
            },
            <(A,B,C)>::required()?
        ))
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
    fn into_transition_mut(mut self) -> Result<TransitionMut,&'static str> {
        Ok(TransitionMut::new(
            move |args| {
                let (p1,p2,p3,p4) = <(A,B,C,D)>::take_from(args);
                let res = self(p1,p2,p3,p4);
                res.insert_into(args);
            },
            <(A,B,C,D)>::required()?
        ))
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
    fn into_transition_mut(mut self) -> Result<TransitionMut,&'static str> {
        Ok(TransitionMut::new(
            move |args| {
                let (p1,p2,p3,p4,p5) = <(A,B,C,D,E)>::take_from(args);
                let res = self(p1,p2,p3,p4,p5);
                res.insert_into(args);
            },
            <(A,B,C,D,E)>::required()?
        ))
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
    fn into_transition_mut(mut self) -> Result<TransitionMut,&'static str> {
        Ok(TransitionMut::new(
            move |args| {
                let (p1,p2,p3,p4,p5,p6) = <(A,B,C,D,E,F)>::take_from(args);
                let res = self(p1,p2,p3,p4,p5,p6);
                res.insert_into(args);
            },
            <(A,B,C,D,E,F)>::required()?
        ))
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
    fn into_transition_mut(mut self) -> Result<TransitionMut,&'static str> {
        Ok(TransitionMut::new(
            move |args| {
                let (p1,p2,p3,p4,p5,p6,p7) = <(A,B,C,D,E,F,G)>::take_from(args);
                let res = self(p1,p2,p3,p4,p5,p6,p7);
                res.insert_into(args);
            },
            <(A,B,C,D,E,F,G)>::required()?
        ))
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
    fn into_transition_mut(mut self) -> Result<TransitionMut,&'static str> {
        Ok(TransitionMut::new(
            move |args| {
                let (p1,p2,p3,p4,p5,p6,p7,p8) = <(A,B,C,D,E,F,G,H)>::take_from(args);
                let res = self(p1,p2,p3,p4,p5,p6,p7,p8);
                res.insert_into(args);
            },
            <(A,B,C,D,E,F,G,H)>::required()?
        ))
    }
}