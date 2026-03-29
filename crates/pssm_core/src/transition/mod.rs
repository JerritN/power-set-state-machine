use crate::{Id, State};
use std::{collections::HashSet, fmt::Debug};

mod andthen;
pub(crate) mod function;
mod into;

pub use andthen::{AndThen, AndThenMut, AndThenOnce};
pub use function::Param;
pub use into::{
    IntoTransition,
    IntoTransitionMut,
    IntoTransitionMutParameterized,
    IntoTransitionOnce,
    IntoTransitionOnceParameterized,
    IntoTransitionParameterized,
    UnknownInput,
};

/// An Error that can occur when running a transition.
/// 
/// This error can occur when a transition is run on a state that does not contain all of the required truths for the transition.
pub enum TransitionError {
    MissingTruth(Id)
}

impl Debug for TransitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransitionError::MissingTruth(id) => write!(f, "A required truth is missing from the State: {:?}", id)
        }
    }
}

/// An Error that can occur when creating a transition from a function.
/// 
/// This error can occur when a function is used to create a transition,
/// but the function requires or produces the same truth multiple times.
pub enum InvalidTransitionError {
    TruthRequiredMultipleTimes(Id),
    TruthProducedMultipleTimes(Id)
}

impl Debug for InvalidTransitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidTransitionError::TruthRequiredMultipleTimes(id) => write!(f, "Transition requires the same truth multiple times: {:?}", id),
            InvalidTransitionError::TruthProducedMultipleTimes(id) => write!(f, "Transition produces the same truth multiple times: {:?}", id)
        }
    }
}

/// A transition is a function that can be executed on a state.
/// 
/// Transitions can be used to change the state of a state machine.
/// They can be created using the `IntoTransition` and `IntoTransitionParameterized` traits.
/// 
/// This transition is side-effect free.
/// For transitions that have side-effects, see `TransitionMut`.
/// For transitions that can only be run once, see `TransitionOnce`.
pub struct Transition<'a> {
    pub(crate) func: Box<dyn Fn(&mut State) -> Result<(), TransitionError> + 'a>,
    pub(crate) requires: HashSet<crate::Id>,
    pub(crate) produces: HashSet<crate::Id>
}

/// A transition is a function that can be executed on a state.
/// 
/// Transitions can be used to change the state of a state machine.
/// They can be created using the `IntoTransitionMut` and `IntoTransitionMutParameterized` traits.
/// 
/// This transition can have side-effects.
/// For transitions that are side-effect free, see `Transition`.
/// For transitions that can only be run once, see `TransitionOnce`.
pub struct TransitionMut<'a> {
    pub(crate) func: Box<dyn FnMut(&mut State) -> Result<(), TransitionError> + 'a>,
    pub(crate) requires: HashSet<crate::Id>,
    pub(crate) produces: HashSet<crate::Id>
}

/// A transition is a function that can be executed on a state.
/// 
/// Transitions can be used to change the state of a state machine.
/// They can be created using the `IntoTransitionOnce` and `IntoTransitionOnceParameterized` traits.
/// 
/// This transition can only be run once.
/// For transitions that are side-effect free, see `Transition`.
/// For transitions that have side-effects, see `TransitionMut`.

pub struct TransitionOnce<'a> {
    pub(crate) func: Box<dyn FnOnce(&mut State) -> Result<(), TransitionError> + 'a>,
    pub(crate) requires: HashSet<crate::Id>,
    pub(crate) produces: HashSet<crate::Id>
}

impl<'a> Transition<'a> {
    pub(crate) fn new<F>(func: F, requires: HashSet<Id>, produces: HashSet<Id>) -> Self 
    where 
        F: Fn(&mut State) -> Result<(), TransitionError> + 'a
    {
        Self {
            func: Box::new(func),
            requires,
            produces
        }
    }

    pub(crate) fn run(&self, state: &mut State) -> Result<(), TransitionError> {
        (self.func)(state)
    }

    pub(crate) fn requires(&self) -> &HashSet<Id> {
        &self.requires
    }
}

impl<'a> TransitionMut<'a> {
    pub(crate) fn new<F>(func: F, requires: HashSet<Id>, produces: HashSet<Id>) -> Self 
    where 
        F: FnMut(&mut State) -> Result<(), TransitionError> + 'a
    {
        Self {
            func: Box::new(func),
            requires,
            produces
        }
    }

    pub(crate) fn run(&mut self, state: &mut State) -> Result<(), TransitionError> {
        (self.func)(state)
    }

    pub(crate) fn requires(&self) -> &HashSet<Id> {
        &self.requires
    }
}

impl<'a> TransitionOnce<'a> {
    pub(crate) fn new<F>(func: F, requires: HashSet<Id>, produces: HashSet<Id>) -> Self 
    where 
        F: FnOnce(&mut State) -> Result<(), TransitionError> + 'a
    {
        Self {
            func: Box::new(func),
            requires,
            produces
        }
    }

    pub(crate) fn run(self, state: &mut State) -> Result<(), TransitionError> {
        (self.func)(state)
    }

    pub(crate) fn requires(&self) -> &HashSet<crate::Id> {
        &self.requires
    }
}

impl<'a> Debug for Transition<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Transition({}->{})", self.requires.len(), self.produces.len()).as_str())
    }
}

impl<'a> Debug for TransitionMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("TransitionMut({}->{})", self.requires.len(), self.produces.len()).as_str())
    }
}

impl<'a> Debug for TransitionOnce<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("TransitionOnce({}->{})", self.requires.len(), self.produces.len()).as_str())
    }
}