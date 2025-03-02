use crate::State;
use std::marker::PhantomData;

mod into;
mod intomut;
mod intoonce;

pub use into::IntoTransition;
pub use intomut::IntoTransitionMut;
pub use intoonce::IntoTransitionOnce;

pub struct SingleMarker();

pub struct Transition<In> {
    func: Box<dyn Fn(&mut State)>,
    _in: PhantomData<In>
}

pub struct TransitionMut<In> {
    func: Box<dyn FnMut(&mut State)>,
    _in: PhantomData<In>
}

pub struct TransitionOnce<In> {
    func: Box<dyn FnOnce(&mut State)>,
    _in: PhantomData<In>
}

impl<In> Transition<In> {
    pub(crate) fn new<F>(func: F) -> Self
    where
        F: Fn(&mut State) + 'static
    {
        Transition {
            func: Box::new(func),
            _in: PhantomData
        }
    }

    pub fn run(&self, state: &mut State) {
        (self.func)(state);
    }
}

impl<In> TransitionMut<In> {
    pub(crate) fn new<F>(func: F) -> Self
    where
        F: FnMut(&mut State) + 'static
    {
        TransitionMut {
            func: Box::new(func),
            _in: PhantomData
        }
    }

    pub fn run(&mut self, state: &mut State) {
        (self.func)(state);
    }
}

impl<In> TransitionOnce<In> {
    pub(crate) fn new<F>(func: F) -> Self
    where
        F: FnOnce(&mut State) + 'static
    {
        TransitionOnce {
            func: Box::new(func),
            _in: PhantomData
        }
    }

    pub fn run(self, state: &mut State) {
        (self.func)(state);
    }
}