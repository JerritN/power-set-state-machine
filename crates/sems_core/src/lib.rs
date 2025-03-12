mod statemachine;

pub mod transition;

use std::{any::{Any, TypeId}, collections::HashMap};

pub use statemachine::StateMachine;

type State = HashMap<Id, Box<dyn Any>>;
type Id = TypeId;

pub trait Truth {
    fn id() -> Id;
}