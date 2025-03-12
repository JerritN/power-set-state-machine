use crate::State;
use std::{collections::HashSet, marker::PhantomData};

mod into;
mod intomut;
mod intoonce;

pub use into::IntoTransition;
pub use intomut::IntoTransitionMut;
pub use intoonce::IntoTransitionOnce;

pub struct SingleMarker();
pub struct UnknownParameter();

pub struct Transition {
    pub(crate) func: Box<dyn Fn(&mut State)>,
    pub(crate) requires: HashSet<crate::Id>
}

pub struct TransitionMut {
    pub(crate) func: Box<dyn FnMut(&mut State)>,
    pub(crate) requires: HashSet<crate::Id>
}

pub struct TransitionOnce {
    pub(crate) func: Box<dyn FnOnce(&mut State)>,
    pub(crate) requires: HashSet<crate::Id>
}

impl Transition {
    pub(crate) fn new<F>(func: F, requires: HashSet<crate::Id>) -> Self 
    where 
        F: Fn(&mut State) + 'static
    {
        Self {
            func: Box::new(func),
            requires
        }
    }

    pub(crate) fn run(&self, state: &mut State) {
        (self.func)(state);
    }

    pub(crate) fn requires(&self) -> &HashSet<crate::Id> {
        &self.requires
    }
}

impl TransitionMut {
    pub(crate) fn new<F>(func: F, requires: HashSet<crate::Id>) -> Self 
    where 
        F: FnMut(&mut State) + 'static
    {
        Self {
            func: Box::new(func),
            requires
        }
    }

    pub(crate) fn run(&mut self, state: &mut State) {
        (self.func)(state);
    }

    pub(crate) fn requires(&self) -> &HashSet<crate::Id> {
        &self.requires
    }
}

impl TransitionOnce {
    pub(crate) fn new<F>(func: F, requires: HashSet<crate::Id>) -> Self 
    where 
        F: FnOnce(&mut State) + 'static
    {
        Self {
            func: Box::new(func),
            requires
        }
    }

    pub(crate) fn run(self, state: &mut State) {
        (self.func)(state);
    }

    pub(crate) fn requires(&self) -> &HashSet<crate::Id> {
        &self.requires
    }
}