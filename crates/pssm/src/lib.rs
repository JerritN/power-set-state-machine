pub mod prelude {
    pub use pssm_core::{
        StateMachine,
        Truth,
        TransitionCallError,
        into_transition_with,
        into_transition_mut_with,
        into_transition_once_with,
        transition::*,
    };
    pub use pssm_dictionary::{Dictionary, TransitionDictionary};
    pub use pssm_macro::Truth;
}

pub use pssm_core as core;
pub use pssm_dictionary as dictionary;
pub use pssm_macro::*;