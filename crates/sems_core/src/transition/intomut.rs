use super::{params::TransitionParam, results::TransitionResult, SingleMarker, Transition, TransitionMut, UnknownParameter};

/// A trait that allows an object to be converted into a `TransitionMut`.
/// 
/// This trait is implemented for:
/// 
/// - The `Transition` type
/// - The `TransitionMut` type
/// - `FnMut` types that take up to 8 parameters of types that implement `TransitionParam`
/// and return a type that implements `TransitionResult`
pub trait IntoTransitionMut<'a,In,Marker>
{
    /// Converts the object into a `TransitionMut`.
    /// 
    /// This function will convert the object into a `TransitionMut`.
    /// If the object cannot be converted into a `TransitionMut`, this function will return an error.
    fn into_transition_mut(self) -> Result<TransitionMut<'a>,&'static str>;
}

impl<'a> IntoTransitionMut<'a,UnknownParameter,()> for Transition<'a>
{
    fn into_transition_mut(self) -> Result<TransitionMut<'a>,&'static str> {
        Ok(TransitionMut::new(
            self.func,
            self.requires
        ))
    }
}

impl<'a> IntoTransitionMut<'a,UnknownParameter,()> for TransitionMut<'a>
{
    fn into_transition_mut(self) -> Result<TransitionMut<'a>,&'static str> {
        Ok(self)
    }
}

impl<'a,Res,Fun> IntoTransitionMut<'a,(),()> for Fun
where 
    Res: TransitionResult,
    Fun: FnMut() -> Res + 'a
{
    fn into_transition_mut(mut self) -> Result<TransitionMut<'a>,&'static str> {
        Ok(TransitionMut::new(
            move |args| {
                let res = self();
                res.insert_into(args);
            },
            <()>::required()?
        ))
    }
}

impl<'a,A,Res,Fun> IntoTransitionMut<'a,A,SingleMarker> for Fun
where 
    A: TransitionParam,
    Res: TransitionResult,
    Fun: FnMut(A) -> Res + 'a
{
    fn into_transition_mut(mut self) -> Result<TransitionMut<'a>,&'static str> {
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

impl<'a,A,B,Res,Fun> IntoTransitionMut<'a,(A,B),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    Res: TransitionResult,
    Fun: FnMut(A,B) -> Res + 'a
{
    fn into_transition_mut(mut self) -> Result<TransitionMut<'a>,&'static str> {
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

impl<'a,A,B,C,Res,Fun> IntoTransitionMut<'a,(A,B,C),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    Res: TransitionResult,
    Fun: FnMut(A,B,C) -> Res + 'a
{
    fn into_transition_mut(mut self) -> Result<TransitionMut<'a>,&'static str> {
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

impl<'a,A,B,C,D,Res,Fun> IntoTransitionMut<'a,(A,B,C,D),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    Res: TransitionResult,
    Fun: FnMut(A,B,C,D) -> Res + 'a
{
    fn into_transition_mut(mut self) -> Result<TransitionMut<'a>,&'static str> {
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

impl<'a,A,B,C,D,E,Res,Fun> IntoTransitionMut<'a,(A,B,C,D,E),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    E: TransitionParam,
    Res: TransitionResult,
    Fun: FnMut(A,B,C,D,E) -> Res + 'a
{
    fn into_transition_mut(mut self) -> Result<TransitionMut<'a>,&'static str> {
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

impl<'a,A,B,C,D,E,F,Res,Fun> IntoTransitionMut<'a,(A,B,C,D,E,F),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    E: TransitionParam,
    F: TransitionParam,
    Res: TransitionResult,
    Fun: FnMut(A,B,C,D,E,F) -> Res + 'a
{
    fn into_transition_mut(mut self) -> Result<TransitionMut<'a>,&'static str> {
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

impl<'a,A,B,C,D,E,F,G,Res,Fun> IntoTransitionMut<'a,(A,B,C,D,E,F,G),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    E: TransitionParam,
    F: TransitionParam,
    G: TransitionParam,
    Res: TransitionResult,
    Fun: FnMut(A,B,C,D,E,F,G) -> Res + 'a
{
    fn into_transition_mut(mut self) -> Result<TransitionMut<'a>,&'static str> {
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

impl<'a,A,B,C,D,E,F,G,H,Res,Fun> IntoTransitionMut<'a,(A,B,C,D,E,F,G,H),()> for Fun
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
    Fun: FnMut(A,B,C,D,E,F,G,H) -> Res + 'a
{
    fn into_transition_mut(mut self) -> Result<TransitionMut<'a>,&'static str> {
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