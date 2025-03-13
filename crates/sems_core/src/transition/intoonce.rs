use super::{params::TransitionParam, results::TransitionResult, SingleMarker, Transition, TransitionMut, TransitionOnce, UnknownParameter};

/// A trait that allows an object to be converted into a `TransitionOnce`.
/// 
/// This trait is implemented for:
/// 
/// - The `Transition` type
/// - The `TransitionMut` type
/// - The `TransitionOnce` type
/// - `FnOnce` types that take up to 8 parameters of types that implement `TransitionParam`
/// and return a type that implements `TransitionResult`
pub trait IntoTransitionOnce<'a,In,Marker>
{
    /// Converts the object into a `TransitionOnce`.
    /// 
    /// This function will convert the object into a `TransitionOnce`.
    /// If the object cannot be converted into a `TransitionOnce`, this function will return an error.
    fn into_transition_once(self) -> Result<TransitionOnce<'a>,&'static str>;
}

impl<'a> IntoTransitionOnce<'a,UnknownParameter,()> for Transition<'a>
{
    fn into_transition_once(self) -> Result<TransitionOnce<'a>,&'static str> {
        Ok(TransitionOnce::new(
            self.func,
            self.requires
        ))
    }
}

impl<'a> IntoTransitionOnce<'a,UnknownParameter,()> for TransitionMut<'a>
{
    fn into_transition_once(mut self) -> Result<TransitionOnce<'a>,&'static str> {
        Ok(TransitionOnce::new(
            move |args| (self.func)(args),
            self.requires
        ))
    }
}

impl<'a> IntoTransitionOnce<'a,UnknownParameter,()> for TransitionOnce<'a>
{
    fn into_transition_once(self) -> Result<TransitionOnce<'a>,&'static str> {
        Ok(self)
    }
}

impl<'a,Res,Fun> IntoTransitionOnce<'a,(),()> for Fun
where 
    Res: TransitionResult,
    Fun: FnOnce() -> Res + 'a
{
    fn into_transition_once(self) -> Result<TransitionOnce<'a>,&'static str> {
        Ok(TransitionOnce::new(
            move |args| {
                let res = self();
                res.insert_into(args);
            },
            <()>::required()?
        ))
    }
}

impl<'a,A,Res,Fun> IntoTransitionOnce<'a,A,SingleMarker> for Fun
where 
    A: TransitionParam,
    Res: TransitionResult,
    Fun: FnOnce(A) -> Res + 'a
{
    fn into_transition_once(self) -> Result<TransitionOnce<'a>,&'static str> {
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

impl<'a,A,B,Res,Fun> IntoTransitionOnce<'a,(A,B),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    Res: TransitionResult,
    Fun: FnOnce(A,B) -> Res + 'a
{
    fn into_transition_once(self) -> Result<TransitionOnce<'a>,&'static str> {
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

impl<'a,A,B,C,Res,Fun> IntoTransitionOnce<'a,(A,B,C),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    Res: TransitionResult,
    Fun: FnOnce(A,B,C) -> Res + 'a
{
    fn into_transition_once(self) -> Result<TransitionOnce<'a>,&'static str> {
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

impl<'a,A,B,C,D,Res,Fun> IntoTransitionOnce<'a,(A,B,C,D),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    Res: TransitionResult,
    Fun: FnOnce(A,B,C,D) -> Res + 'a
{
    fn into_transition_once(self) -> Result<TransitionOnce<'a>,&'static str> {
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

impl<'a,A,B,C,D,E,Res,Fun> IntoTransitionOnce<'a,(A,B,C,D,E),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    E: TransitionParam,
    Res: TransitionResult,
    Fun: FnOnce(A,B,C,D,E) -> Res + 'a
{
    fn into_transition_once(self) -> Result<TransitionOnce<'a>,&'static str> {
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

impl<'a,A,B,C,D,E,F,Res,Fun> IntoTransitionOnce<'a,(A,B,C,D,E,F),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    E: TransitionParam,
    F: TransitionParam,
    Res: TransitionResult,
    Fun: FnOnce(A,B,C,D,E,F) -> Res + 'a
{
    fn into_transition_once(self) -> Result<TransitionOnce<'a>,&'static str> {
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

impl<'a,A,B,C,D,E,F,G,Res,Fun> IntoTransitionOnce<'a,(A,B,C,D,E,F,G),()> for Fun
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    E: TransitionParam,
    F: TransitionParam,
    G: TransitionParam,
    Res: TransitionResult,
    Fun: FnOnce(A,B,C,D,E,F,G) -> Res + 'a
{
    fn into_transition_once(self) -> Result<TransitionOnce<'a>,&'static str> {
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

impl<'a,A,B,C,D,E,F,G,H,Res,Fun> IntoTransitionOnce<'a,(A,B,C,D,E,F,G,H),()> for Fun
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
    Fun: FnOnce(A,B,C,D,E,F,G,H) -> Res + 'a
{
    fn into_transition_once(self) -> Result<TransitionOnce<'a>,&'static str> {
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