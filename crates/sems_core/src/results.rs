use std::any::TypeId;

use crate::{State, Truth};

pub trait TransitionResult {
    fn insert_into(self, state: &mut State);
}

impl TransitionResult for () {
    fn insert_into(self, _: &mut State) {}
}

impl<T: Truth + 'static> TransitionResult for T {
    fn insert_into(self, state: &mut State) {
        state.insert(TypeId::of::<T>(), Box::new(self));
    }
}

impl<A, B> TransitionResult for (A, B) 
where 
    A: TransitionResult,
    B: TransitionResult
{
    fn insert_into(self, state: &mut State) {
        let (a,b) = self;
        a.insert_into(state);
        b.insert_into(state);
    }
}