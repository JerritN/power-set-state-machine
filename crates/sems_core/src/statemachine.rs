use std::collections::HashMap;

use crate::{params::TransitionParam, transition::IntoTransition, State, Truth};

pub struct StateMachine {
    state: State
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    pub fn can_run<T,In,Marker>(&self, _: T) -> bool 
    where 
        In: TransitionParam,
        T: IntoTransition<In,Marker>
    {
        In::ids().iter().all(|id| self.state.contains_key(id))
    }

    pub fn run<T,In,Marker>(&mut self, transition: T) -> Result<(), &str>
    where 
        In: TransitionParam,
        T: IntoTransition<In,Marker> + Copy
    {
        if self.can_run(transition) {
            self.run_unchecked(transition);
            Ok(())
        } else {
            Err("Cannot run transition on current state")
        }
    }

    pub fn run_unchecked<T,In,Marker>(&mut self, transition: T)
    where 
        In: TransitionParam,
        T: IntoTransition<In,Marker>
    {
        transition.into_transition().run(&mut self.state);
    }

    pub fn set_truth<T: Truth + 'static>(&mut self, element: T) {
        self.state.insert(T::id(), Box::new(element));
    }

    pub fn has_truth<T: Truth + 'static>(&self) -> bool {
        self.state.contains_key(&T::id())
    }

    pub fn unset_truth<T: Truth + 'static>(&mut self) -> Option<T> {
        Option::<T>::take_from(&mut self.state)
    }
}