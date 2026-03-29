mod statemachine;

pub mod transition;

use std::{any::{Any, TypeId}, collections::HashMap};

pub use statemachine::StateMachine;

use crate::transition::{InvalidTransitionError, TransitionError};

type State = HashMap<Id, Box<dyn Any>>;
type Id = TypeId;

/// An Error that can occur when calling an object that can be converted into a transition.
/// 
/// This error can occur when calling an object that can be converted into a transition, but the object is not a valid transition,
/// or when the transition is run on a state that does not contain all of the required truths for the transition.
pub enum TransitionCallError {
    InvalidTransition(InvalidTransitionError),
    TransitionError(TransitionError)
}

impl From<InvalidTransitionError> for TransitionCallError {
    fn from(value: InvalidTransitionError) -> Self {
        TransitionCallError::InvalidTransition(value)
    }
}

impl From<TransitionError> for TransitionCallError {
    fn from(value: TransitionError) -> Self {
        TransitionCallError::TransitionError(value)
    }
}

impl std::fmt::Debug for TransitionCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransitionCallError::InvalidTransition(e) => e.fmt(f),
            TransitionCallError::TransitionError(e) => e.fmt(f)
        }
    }
}

/// A trait that represents a truth.
/// 
/// A truth is a piece of data that can be stored in a state machine.
/// 
/// # Examples
/// 
/// ```
/// use pssm::prelude::*;
/// 
/// #[derive(Truth)]
/// struct A();
/// 
/// assert_eq!(A::id(), std::any::TypeId::of::<A>());
/// ```
pub trait Truth {
    fn id() -> Id;
}