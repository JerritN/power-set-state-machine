pub mod prelude {
    pub use pssm_core::{StateMachine,transition::{IntoTransitionMut, TransitionMut}};
    pub use pssm_dictionary::{Dictionary, TransitionDictionary};
    pub use pssm_macro::Truth;
}

pub use pssm_core::*;
pub use pssm_dictionary::*;
pub use pssm_macro::*;