mod statemachine;
mod params;
mod results;

pub mod transition;

use std::{any::{Any, TypeId}, collections::HashMap};

pub use statemachine::StateMachine;
pub use params::TransitionParam;

pub type State = HashMap<Id, Box<dyn Any>>;
pub type Id = TypeId;

pub trait Truth {
    fn id() -> Id;
}