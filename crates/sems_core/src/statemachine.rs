use std::collections::HashMap;

use crate::{transition::{IntoTransitionOnce, Transition, TransitionMut, TransitionOnce, TransitionParam}, State, Truth};

pub struct StateMachine {
    state: State
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    pub fn can_run<T,In,Marker>(&self, _: &T) -> Result<bool,&'static str>
    where 
        In: TransitionParam,
        T: IntoTransitionOnce<In,Marker>
    {
        Ok(In::required()?.iter().all(|id| self.state.contains_key(id)))
    }

    pub fn can_run_transition(&self, transition: &Transition) -> bool {
        transition.requires().iter().all(|id| self.state.contains_key(id))
    }

    pub fn can_run_transition_mut(&self, transition: &TransitionMut) -> bool {
        transition.requires().iter().all(|id| self.state.contains_key(id))
    }

    pub fn can_run_transition_once(&self, transition: &TransitionOnce) -> bool {
        transition.requires().iter().all(|id| self.state.contains_key(id))
    }

    pub fn run<T,In,Marker>(&mut self, transition: T) -> Result<(),&'static str>
    where 
        T: IntoTransitionOnce<In,Marker>
    {
        let transition = transition.into_transition_once()?;
        if transition.requires().iter().all(|id| self.state.contains_key(id)) {
            transition.run(&mut self.state);
            Ok(())
        } else {
            Err("Missing a required truth")
        }
    }

    pub fn run_unchecked(&mut self, transition: TransitionOnce)
    {
        transition.run(&mut self.state);
    }

    pub fn run_ref_unchecked(&mut self, transition: &Transition)
    {
        transition.run(&mut self.state);
    }

    pub fn run_ref_mut_unchecked(&mut self, transition: &mut TransitionMut)
    {
        transition.run(&mut self.state);
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