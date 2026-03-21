use super::{Transition, TransitionMut, TransitionOnce};
use super::function::{TransitionFunction, TransitionFunctionMut, TransitionFunctionOnce, TransitionInput, TransitionOutput};

/// A marker type for transitions that take an unknown input. 
pub struct UnknownInput();

/// A trait that allows an object to be converted into a `Transition` with some parameters.
/// 
/// This trait is implemented for:
/// 
/// - The `Transition` type.
/// - `Fn` types that take up to 4 inputs and up to 2 parameters
/// 
/// # Parameters
/// 
/// Function parameters must be ordered as inputs first, then parameters.
/// - Inputs must implement `TransitionInput`
/// - Parameters must be of type `Param<P>`.
/// - Return type must implement `TransitionOutput`
pub trait IntoTransitionParameterized<'a,In,Param> {
    fn into_transition_with(self, params: Param) -> Result<Transition<'a>,&'static str>;
}

/// A trait that allows an object to be converted into a `TransitionMut` with some parameters.
/// 
/// This trait is implemented for:
/// 
/// - The `TransitionMut` type.
/// - The `Transition` type.
/// - `FnMut` types that take up to 4 inputs and up to 2 parameters.
/// 
/// # Parameters
/// 
/// Function parameters must be ordered as inputs first, then parameters.
/// - Inputs must implement `TransitionInput`
/// - Parameters must be of type `Param<P>`.
/// - Return type must implement `TransitionOutput`
pub trait IntoTransitionMutParameterized<'a,In,Param> {
    fn into_transition_mut_with(self, params: Param) -> Result<TransitionMut<'a>,&'static str>;
}

/// A trait that allows an object to be converted into a `TransitionOnce` with some parameters.
/// 
/// This trait is implemented for:
/// 
/// - The `TransitionOnce` type.
/// - The `TransitionMut` type.
/// - The `Transition` type.
/// - `FnOnce` types that take up to 4 inputs and up to 2 parameters.
/// 
/// # Parameters
/// 
/// Function parameters must be ordered as inputs first, then parameters.
/// - Inputs must implement `TransitionInput`
/// - Parameters must be of type `Param<P>`.
/// - Return type must implement `TransitionOutput`
pub trait IntoTransitionOnceParameterized<'a,In,Param> {
    fn into_transition_once_with(self, params: Param) -> Result<TransitionOnce<'a>,&'static str>;
}

impl<'a> IntoTransitionParameterized<'a, UnknownInput, ()> for Transition<'a>
{
    fn into_transition_with(self, _params: ()) -> Result<Transition<'a>,&'static str> {
        Ok(self)
    }
}

impl<'a,In,Param,F> IntoTransitionParameterized<'a,In,Param> for F
where 
    In: TransitionInput,
    Param: TransitionInput + Clone + 'a,
    F: TransitionFunction<In,Param> + 'a
{
    fn into_transition_with(self, params: Param) -> Result<Transition<'a>,&'static str> {
        Ok(Transition::new(
            move |args| {
                let input = <In>::take_from(args);
                let res = self.call(input, params.clone());
                res.insert_into(args);
            },
            <In>::required()?,
            F::Result::produces()?
        ))
    }
}

impl<'a> IntoTransitionMutParameterized<'a, UnknownInput, ()> for TransitionMut<'a>
{
    fn into_transition_mut_with(self, _params: ()) -> Result<TransitionMut<'a>,&'static str> {
        Ok(self)
    }
}

impl<'a> IntoTransitionMutParameterized<'a, UnknownInput, ()> for Transition<'a>
{
    fn into_transition_mut_with(self, _params: ()) -> Result<TransitionMut<'a>,&'static str> {
        Ok(TransitionMut::new(
            move |args| (self.func)(args),
            self.requires,
            self.produces
        ))
    }
}

impl<'a,In,Param,F> IntoTransitionMutParameterized<'a,In,Param> for F
where 
    In: TransitionInput,
    Param: TransitionInput + Clone + 'a,
    F: TransitionFunctionMut<In,Param> + 'a
{
    fn into_transition_mut_with(mut self, params: Param) -> Result<TransitionMut<'a>,&'static str> {
        Ok(TransitionMut::new(
            move |args| {
                let input = <In>::take_from(args);
                let res = self.call(input, params.clone());
                res.insert_into(args);
            },
            <In>::required()?,
            F::Result::produces()?
        ))
    }
}

impl<'a> IntoTransitionOnceParameterized<'a, UnknownInput, ()> for TransitionOnce<'a>
{
    fn into_transition_once_with(self, _params: ()) -> Result<TransitionOnce<'a>,&'static str> {
        Ok(self)
    }
}

impl<'a> IntoTransitionOnceParameterized<'a, UnknownInput, ()> for TransitionMut<'a>
{
    fn into_transition_once_with(mut self, _params: ()) -> Result<TransitionOnce<'a>,&'static str> {
        Ok(TransitionOnce::new(
            move |args| (self.func)(args),
            self.requires,
            self.produces
        ))
    }
}

impl<'a> IntoTransitionOnceParameterized<'a, UnknownInput, ()> for Transition<'a>
{
    fn into_transition_once_with(self, _params: ()) -> Result<TransitionOnce<'a>,&'static str> {
        Ok(TransitionOnce::new(
            self.func,
            self.requires,
            self.produces
        ))
    }
}

impl<'a,In,Param,F> IntoTransitionOnceParameterized<'a,In,Param> for F
where 
    In: TransitionInput,
    Param: TransitionInput + 'a,
    F: TransitionFunctionOnce<In,Param> + 'a
{
    fn into_transition_once_with(self, params: Param) -> Result<TransitionOnce<'a>,&'static str> {
        Ok(TransitionOnce::new(
            move |args| {
                let input = <In>::take_from(args);
                let res = self.call(input, params);
                res.insert_into(args);
            },
            <In>::required()?,
            F::Result::produces()?
        ))
    }
}

/// A trait that allows an object to be converted into a `Transition`.
/// 
/// This trait is implemented for:
/// 
/// - The `Transition` type.
/// - `Fn` types that take up to 4 inputs of types that implement `TransitionInput`
///     and return a type that implements `TransitionOutput`.
pub trait IntoTransition<'a,In> {
    fn into_transition(self) -> Result<Transition<'a>,&'static str>;
}

/// A trait that allows an object to be converted into a `TransitionMut`.
/// 
/// This trait is implemented for:
/// 
/// - The `TransitionMut` type.
/// - The `Transition` type.
/// - `FnMut` types that take up to 4 inputs of types that implement `TransitionInput`
///    and return a type that implements `TransitionOutput`.
pub trait IntoTransitionMut<'a,In> {
    fn into_transition_mut(self) -> Result<TransitionMut<'a>,&'static str>;
}

/// A trait that allows an object to be converted into a `TransitionOnce`.
/// 
/// This trait is implemented for:
/// 
/// - The `TransitionOnce` type.
/// - The `TransitionMut` type.
/// - The `Transition` type.
/// - `FnOnce` types that take up to 4 inputs of types that implement `TransitionInput`
///   and return a type that implements `TransitionOutput`.
pub trait IntoTransitionOnce<'a,In> {
    fn into_transition_once(self) -> Result<TransitionOnce<'a>,&'static str>;
}

impl<'a,In,F> IntoTransition<'a,In> for F
where 
    F: IntoTransitionParameterized<'a, In, ()>
{
    fn into_transition(self) -> Result<Transition<'a>,&'static str> {
        self.into_transition_with(())
    }
}

impl<'a,In,F> IntoTransitionMut<'a,In> for F
where 
    F: IntoTransitionMutParameterized<'a, In, ()>
{
    fn into_transition_mut(self) -> Result<TransitionMut<'a>,&'static str> {
        self.into_transition_mut_with(())
    }
}

impl<'a,In,F> IntoTransitionOnce<'a,In> for F
where 
    F: IntoTransitionOnceParameterized<'a, In, ()>
{
    fn into_transition_once(self) -> Result<TransitionOnce<'a>,&'static str> {
        self.into_transition_once_with(())
    }
}