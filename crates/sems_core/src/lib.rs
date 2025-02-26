mod transition;
mod statemachine;
mod truth;

pub use transition::Transition;
pub use truth::Truth;
pub use statemachine::StateMachine;

pub type Id = &'static str;