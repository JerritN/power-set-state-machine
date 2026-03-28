pub mod prelude {
    pub use pssm_core::{
        StateMachine,
        Truth,
        transition::*,
    };
    pub use pssm_dictionary::{Dictionary, TransitionDictionary};
    pub use pssm_macro::Truth;
}

pub use pssm_core::{*, transition::*};
pub use pssm_dictionary as dictionary;
pub use pssm_macro::*;