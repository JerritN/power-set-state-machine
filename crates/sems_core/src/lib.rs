mod transition;
mod statemachine;
mod params;
mod results;

use std::{any::{Any, TypeId}, collections::HashMap};

type State = HashMap<Id, Box<dyn Any>>;

pub use statemachine::StateMachine;
pub type Id = TypeId;

pub trait Truth {
    fn id() -> Id;
}