use std::fmt::Debug;

use crate::Id;
use crate::transition::InvalidTransitionError;

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
    /// Converts this object into a `Transition` with the given parameters.
    /// 
    /// This function will convert this object into a `Transition` with the given parameters.
    /// Returns an error if this object cannot be converted into a `Transition` with the given parameters.
    /// 
    /// For a simple way to convert a transition function with parameters into a `Transition` type, see the `into_transition_with!` macro.
    fn into_transition_with(self, params: Param) -> Result<Transition<'a>,InvalidTransitionError>;
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
    /// Converts this object into a `TransitionMut` with the given parameters.
    /// 
    /// This function will convert this object into a `TransitionMut` with the given parameters.
    /// Returns an error if this object cannot be converted into a `TransitionMut` with the given parameters.
    /// 
    /// For a simple way to convert a transition function with parameters into a `TransitionMut` type, see the `into_transition_mut_with!` macro.
    fn into_transition_mut_with(self, params: Param) -> Result<TransitionMut<'a>,InvalidTransitionError>;
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
    /// Converts this object into a `TransitionOnce` with the given parameters.
    /// 
    /// This function will convert this object into a `TransitionOnce` with the given parameters.
    /// Returns an error if this object cannot be converted into a `TransitionOnce` with the given parameters.
    /// 
    /// For a simple way to convert a transition function with parameters into a `TransitionOnce` type, see the `into_transition_once_with!` macro.
    fn into_transition_once_with(self, params: Param) -> Result<TransitionOnce<'a>,InvalidTransitionError>;
}

impl<'a> IntoTransitionParameterized<'a, UnknownInput, ()> for Transition<'a>
{
    fn into_transition_with(self, _params: ()) -> Result<Transition<'a>,InvalidTransitionError> {
        Ok(self)
    }
}

impl<'a,In,Param,F> IntoTransitionParameterized<'a,In,Param> for F
where 
    In: TransitionInput,
    Param: Clone + 'a,
    F: TransitionFunction<In,Param> + 'a
{
    fn into_transition_with(self, params: Param) -> Result<Transition<'a>,InvalidTransitionError> {
        Ok(Transition::new(
            move |args| {
                let input = <In>::try_take_from(args)?;
                let res = self.call(input, params.clone());
                res.insert_into(args);
                Ok(())
            },
            <In>::required()?,
            F::Result::produces()?
        ))
    }
}

impl<'a> IntoTransitionMutParameterized<'a, UnknownInput, ()> for TransitionMut<'a>
{
    fn into_transition_mut_with(self, _params: ()) -> Result<TransitionMut<'a>,InvalidTransitionError> {
        Ok(self)
    }
}

impl<'a> IntoTransitionMutParameterized<'a, UnknownInput, ()> for Transition<'a>
{
    fn into_transition_mut_with(self, _params: ()) -> Result<TransitionMut<'a>,InvalidTransitionError> {
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
    Param: Clone + 'a,
    F: TransitionFunctionMut<In,Param> + 'a
{
    fn into_transition_mut_with(mut self, params: Param) -> Result<TransitionMut<'a>,InvalidTransitionError> {
        Ok(TransitionMut::new(
            move |args| {
                let input = <In>::try_take_from(args)?;
                let res = self.call(input, params.clone());
                res.insert_into(args);
                Ok(())
            },
            <In>::required()?,
            F::Result::produces()?
        ))
    }
}

impl<'a> IntoTransitionOnceParameterized<'a, UnknownInput, ()> for TransitionOnce<'a>
{
    fn into_transition_once_with(self, _params: ()) -> Result<TransitionOnce<'a>,InvalidTransitionError> {
        Ok(self)
    }
}

impl<'a> IntoTransitionOnceParameterized<'a, UnknownInput, ()> for TransitionMut<'a>
{
    fn into_transition_once_with(mut self, _params: ()) -> Result<TransitionOnce<'a>,InvalidTransitionError> {
        Ok(TransitionOnce::new(
            move |args| (self.func)(args),
            self.requires,
            self.produces
        ))
    }
}

impl<'a> IntoTransitionOnceParameterized<'a, UnknownInput, ()> for Transition<'a>
{
    fn into_transition_once_with(self, _params: ()) -> Result<TransitionOnce<'a>,InvalidTransitionError> {
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
    Param: 'a,
    F: TransitionFunctionOnce<In,Param> + 'a
{
    fn into_transition_once_with(self, params: Param) -> Result<TransitionOnce<'a>,InvalidTransitionError> {
        Ok(TransitionOnce::new(
            move |args| {
                let input = <In>::try_take_from(args)?;
                let res = self.call(input, params);
                res.insert_into(args);
                Ok(())
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
    fn into_transition(self) -> Result<Transition<'a>,InvalidTransitionError>;
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
    fn into_transition_mut(self) -> Result<TransitionMut<'a>,InvalidTransitionError>;
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
    fn into_transition_once(self) -> Result<TransitionOnce<'a>,InvalidTransitionError>;
}

impl<'a,In,F> IntoTransition<'a,In> for F
where 
    F: IntoTransitionParameterized<'a, In, ()>
{
    fn into_transition(self) -> Result<Transition<'a>,InvalidTransitionError> {
        self.into_transition_with(())
    }
}

impl<'a,In,F> IntoTransitionMut<'a,In> for F
where 
    F: IntoTransitionMutParameterized<'a, In, ()>
{
    fn into_transition_mut(self) -> Result<TransitionMut<'a>,InvalidTransitionError> {
        self.into_transition_mut_with(())
    }
}

impl<'a,In,F> IntoTransitionOnce<'a,In> for F
where 
    F: IntoTransitionOnceParameterized<'a, In, ()>
{
    fn into_transition_once(self) -> Result<TransitionOnce<'a>,InvalidTransitionError> {
        self.into_transition_once_with(())
    }
}

/// A macro to convert a transition function with parameters into a `Transition` type.
/// 
/// The first argument is the transition function, and the rest of the arguments are the parameters to be passed to the transition function.
/// 
/// This calls `into_transition_with` on the transition function, which will convert it into a `Transition` type.
/// If the transition function is not a valid transition function, an error will be returned.
/// See `IntoTransitionParameterized` for more details.
/// 
/// # Examples
/// 
/// ```
/// use pssm::prelude::*;
/// 
/// #[derive(Truth,Debug)]
/// struct A(i32);
/// 
/// fn insert_a_with_param(Param(param): Param<i32>) -> A {
///   A(param)
/// }
/// 
/// let mut state_machine = StateMachine::new();
/// let transition = into_transition_with!(insert_a_with_param, 42).unwrap();
/// state_machine.run(transition);
/// 
/// let a = state_machine.unset_truth::<A>().unwrap();
/// assert_eq!(a.0, 42);
/// ```
#[macro_export]
macro_rules! into_transition_with {
    ($transition:ident, $($param:expr),*) => {
        $transition.into_transition_with(($($param,)*))
    };
}

/// A macro to convert a transition function with parameters into a `TransitionMut` type.
///
/// The first argument is the transition function, and the rest of the arguments are the parameters to be passed to the transition function.
/// 
/// This calls `into_transition_mut_with` on the transition function, which will convert it into a `TransitionMut` type.
/// If the transition function is not a valid transition function, an error will be returned.
/// See `IntoTransitionMutParameterized` for more details.
/// 
/// # Examples
/// 
/// ```
/// use pssm::prelude::*;
/// 
/// #[derive(Truth,Debug)]
/// struct A(i32);
/// 
/// fn insert_a_with_param(Param(param): Param<i32>) -> A {
///     A(param)
/// }
/// 
/// let mut state_machine = StateMachine::new();
/// let transition = into_transition_mut_with!(insert_a_with_param, 42).unwrap();
/// state_machine.run(transition);
/// 
/// let a = state_machine.unset_truth::<A>().unwrap();
/// assert_eq!(a.0, 42);
/// ```
#[macro_export]
macro_rules! into_transition_mut_with {
    ($transition:ident, $($param:expr),*) => {
        $transition.into_transition_mut_with(($($param,)*))
    };
}

/// A macro to convert a transition function with parameters into a `TransitionOnce` type.
/// 
/// The first argument is the transition function, and the rest of the arguments are the parameters to be passed to the transition function.
/// 
/// This calls `into_transition_once_with` on the transition function, which will convert it into a `TransitionOnce` type.
/// If the transition function is not a valid transition function, an error will be returned.
/// See `IntoTransitionOnceParameterized` for more details.
///
/// # Examples
///
/// ```
/// use pssm::prelude::*;
/// 
/// #[derive(Truth,Debug)]
/// struct A(i32);
/// 
/// fn insert_a_with_param(Param(param): Param<i32>) -> A {
///     A(param)
/// }
/// 
/// let mut state_machine = StateMachine::new();
/// let transition = into_transition_once_with!(insert_a_with_param, 42).unwrap();
/// state_machine.run(transition);
/// 
/// let a = state_machine.unset_truth::<A>().unwrap();
/// assert_eq!(a.0, 42);
/// ```

#[macro_export]
macro_rules! into_transition_once_with {
    ($transition:ident, $($param:expr),*) => {
        $transition.into_transition_once_with(($($param,)*))
    };
}