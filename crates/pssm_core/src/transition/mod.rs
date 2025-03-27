use crate::{Id, State};
use std::{collections::HashSet, fmt::Debug};

mod andthen;
mod into;
mod intomut;
mod intoonce;
mod params;
mod results;

pub use andthen::{AndThen, AndThenMut, AndThenOnce};
pub use into::IntoTransition;
pub use intomut::IntoTransitionMut;
pub use intoonce::IntoTransitionOnce;
pub use params::TransitionParam;
pub use results::TransitionResult;

pub struct SingleMarker();
pub struct UnknownParameter();

/// A transition is a function that can be executed on a state.
/// 
/// Transitions can be used to change the state of a state machine.
/// They can be created using the `IntoTransition` trait.
/// 
/// This transition is a side-effect free transition.
/// For transitions that have side-effects, see `TransitionMut`.
/// For transitions that can only be run once, see `TransitionOnce`.
pub struct Transition<'a> {
    pub(crate) func: Box<dyn Fn(&mut State) + 'a>,
    pub(crate) requires: HashSet<crate::Id>,
    pub(crate) produces: HashSet<crate::Id>
}

/// A transition is a function that can be executed on a state.
/// 
/// Transitions can be used to change the state of a state machine.
/// They can be created using the `IntoTransitionMut` trait.
/// 
/// This transition is a transition that can have side-effects.
/// For transitions that are side-effect free, see `Transition`.
/// For transitions that can only be run once, see `TransitionOnce`.
pub struct TransitionMut<'a> {
    pub(crate) func: Box<dyn FnMut(&mut State) + 'a>,
    pub(crate) requires: HashSet<crate::Id>,
    pub(crate) produces: HashSet<crate::Id>
}

/// A transition is a function that can be executed on a state.
/// 
/// Transitions can be used to change the state of a state machine.
/// They can be created using the `IntoTransitionOnce` trait.
/// 
/// This transition is a transition that can only be run once.
/// For transitions that are side-effect free, see `Transition`.
/// For transitions that have side-effects, see `TransitionMut`.

pub struct TransitionOnce<'a> {
    pub(crate) func: Box<dyn FnOnce(&mut State) + 'a>,
    pub(crate) requires: HashSet<crate::Id>,
    pub(crate) produces: HashSet<crate::Id>
}

impl<'a> Transition<'a> {
    pub(crate) fn new<F>(func: F, requires: HashSet<Id>, produces: HashSet<Id>) -> Self 
    where 
        F: Fn(&mut State) + 'a
    {
        Self {
            func: Box::new(func),
            requires,
            produces
        }
    }

    pub(crate) fn run(&self, state: &mut State) {
        (self.func)(state);
    }

    pub(crate) fn requires(&self) -> &HashSet<Id> {
        &self.requires
    }
}

impl<'a> TransitionMut<'a> {
    pub(crate) fn new<F>(func: F, requires: HashSet<Id>, produces: HashSet<Id>) -> Self 
    where 
        F: FnMut(&mut State) + 'a
    {
        Self {
            func: Box::new(func),
            requires,
            produces
        }
    }

    pub(crate) fn run(&mut self, state: &mut State) {
        (self.func)(state);
    }

    pub(crate) fn requires(&self) -> &HashSet<Id> {
        &self.requires
    }
}

impl<'a> TransitionOnce<'a> {
    pub(crate) fn new<F>(func: F, requires: HashSet<Id>, produces: HashSet<Id>) -> Self 
    where 
        F: FnOnce(&mut State) + 'a
    {
        Self {
            func: Box::new(func),
            requires,
            produces
        }
    }

    pub(crate) fn run(self, state: &mut State) {
        (self.func)(state);
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