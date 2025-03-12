use super::{params::TransitionParam, results::TransitionResult, SingleMarker, Transition, TransitionMut, TransitionOnce, UnknownParameter};

pub trait IntoTransitionOnce<In,Marker>
{
    fn into_transition_once(self) -> Result<TransitionOnce,&'static str>;
}

impl IntoTransitionOnce<UnknownParameter,()> for Transition
{
    fn into_transition_once(self) -> Result<TransitionOnce,&'static str> {
        Ok(TransitionOnce::new(
            self.func,
            self.requires
        ))
    }
}

impl IntoTransitionOnce<UnknownParameter,()> for TransitionMut
{
    fn into_transition_once(self) -> Result<TransitionOnce,&'static str> {
        Ok(TransitionOnce::new(
            self.func,
            self.requires
        ))
    }
}

impl IntoTransitionOnce<UnknownParameter,()> for TransitionOnce
{
    fn into_transition_once(self) -> Result<TransitionOnce,&'static str> {
        Ok(self)
    }
}

impl<Res,Fun> IntoTransitionOnce<(),()> for Fun
where 
    Res: TransitionResult,
    Fun: FnOnce() -> Res + 'static
{
    fn into_transition_once(self) -> Result<TransitionOnce,&'static str> {
        Ok(TransitionOnce::new(
            move |args| {
                let res = self();
                res.insert_into(args);
            },
            <()>::required()?
        ))
    }
}

impl<A,Res,Fun> IntoTransitionOnce<A,SingleMarker> for Fun
where 
    A: TransitionParam,
    Res: TransitionResult,
    Fun: FnOnce(A) -> Res + 'static
{
    fn into_transition_once(self) -> Result<TransitionOnce,&'static str> {
        Ok(TransitionOnce::new(
            move |args| {
                let p = <A>::take_from(args);
                let res = self(p);
                res.insert_into(args);
            },
            A::required()?
        ))
    }
}

impl<A,B,Res,Fun> IntoTransitionOnce<(A,B),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    Res: TransitionResult,
    Fun: FnOnce(A,B) -> Res + 'static
{
    fn into_transition_once(self) -> Result<TransitionOnce,&'static str> {
        Ok(TransitionOnce::new(
            move |args| {
                let p = <(A,B)>::take_from(args);
                let res = self(p.0,p.1);
                res.insert_into(args);
            },
            <(A,B)>::required()?
        ))
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
    fn into_transition_once(self) -> Result<TransitionOnce,&'static str> {
        Ok(TransitionOnce::new(
            move |args| {
                let p = <(A,B,C)>::take_from(args);
                let res = self(p.0,p.1,p.2);
                res.insert_into(args);
            },
            <(A,B,C)>::required()?
        ))
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
    fn into_transition_once(self) -> Result<TransitionOnce,&'static str> {
        Ok(TransitionOnce::new(
            move |args| {
                let p = <(A,B,C,D)>::take_from(args);
                let res = self(p.0,p.1,p.2,p.3);
                res.insert_into(args);
            },
            <(A,B,C,D)>::required()?
        ))
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
    fn into_transition_once(self) -> Result<TransitionOnce,&'static str> {
        Ok(TransitionOnce::new(
            move |args| {
                let p = <(A,B,C,D,E)>::take_from(args);
                let res = self(p.0,p.1,p.2,p.3,p.4);
                res.insert_into(args);
            },
            <(A,B,C,D,E)>::required()?
        ))
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
    fn into_transition_once(self) -> Result<TransitionOnce,&'static str> {
        Ok(TransitionOnce::new(
            move |args| {
                let p = <(A,B,C,D,E,F)>::take_from(args);
                let res = self(p.0,p.1,p.2,p.3,p.4,p.5);
                res.insert_into(args);
            },
            <(A,B,C,D,E,F)>::required()?
        ))
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
    fn into_transition_once(self) -> Result<TransitionOnce,&'static str> {
        Ok(TransitionOnce::new(
            move |args| {
                let p = <(A,B,C,D,E,F,G)>::take_from(args);
                let res = self(p.0,p.1,p.2,p.3,p.4,p.5,p.6);
                res.insert_into(args);
            },
            <(A,B,C,D,E,F,G)>::required()?
        ))
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
    fn into_transition_once(self) -> Result<TransitionOnce,&'static str> {
        Ok(TransitionOnce::new(
            move |args| {
                let p = <(A,B,C,D,E,F,G,H)>::take_from(args);
                let res = self(p.0,p.1,p.2,p.3,p.4,p.5,p.6,p.7);
                res.insert_into(args);
            },
            <(A,B,C,D,E,F,G,H)>::required()?
        ))
    }
}