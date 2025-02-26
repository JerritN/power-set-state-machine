mod transition;
mod statemachine;
mod params;
mod results;

use std::{any::{Any, TypeId}, collections::HashMap};

pub use transition::Transition;
pub use statemachine::StateMachine;

pub type Id = TypeId;
type State = HashMap<Id, Box<dyn Any>>;

pub trait Truth {
    fn id() -> Id;
}