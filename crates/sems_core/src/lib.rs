mod statemachine;

pub mod transition;

use std::{any::{Any, TypeId}, collections::HashMap};

pub use statemachine::StateMachine;

type State = HashMap<Id, Box<dyn Any>>;
type Id = TypeId;

/// A trait that represents a truth.
/// 
/// A truth is a piece of data that can be stored in a state machine.
/// 
/// # Examples
/// 
/// ```
/// use sems_core::Truth;
/// use sems_macro::*;
/// 
/// #[derive(Truth)]
/// struct A();
/// 
/// assert_eq!(A::id(), std::any::TypeId::of::<A>());
/// ```
pub trait Truth {
    fn id() -> Id;
}