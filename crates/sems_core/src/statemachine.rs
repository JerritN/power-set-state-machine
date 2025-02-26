use std::{any::Any, collections::HashMap};

use crate::{transition::IntoTransition, truth::{Deconstructable, Requestable}, Truth};

pub struct StateMachine {
    state: HashMap<&'static str, Box<dyn Any>>,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    pub fn can_run<T,In,Out>(&self, _: T) -> bool 
    where 
        In: Requestable,
        Out: Deconstructable,
        T: IntoTransition<(In,Out)>
    {
        In::ids().iter().all(|id| self.state.contains_key(id))
    }

    pub fn run<T,In,Out>(&mut self, transition: T) -> Result<(), &str>
    where 
        In: Requestable,
        Out: Deconstructable,
        T: IntoTransition<(In,Out)> + Copy
    {
        if self.can_run(transition) {
            self.run_unchecked(transition);
            Ok(())
        } else {
            Err("Cannot run transition on current state")
        }
    }

    pub fn run_unchecked<T,In,Out>(&mut self, transition: T)
    where 
        In: Requestable,
        Out: Deconstructable,
        T: IntoTransition<(In,Out)>
    {
        let args = In::ids()
            .iter()
            .map(|id| self.state.remove(id)
                .expect(format!("State does not contain required truth {}", id).as_str())
            ).collect();

        let res = transition.into_transition().run(args);

        for (id,val) in res {
            self.state.insert(id,val);
        }
    }

    pub fn set_truth<T: Truth + 'static>(&mut self, element: T) {
        self.state.insert(T::id(), Box::new(element));
    }

    pub fn has_truth<T: Truth + 'static>(&self) -> bool {
        self.state.contains_key(T::id())
    }

    pub fn unset_truth<T: Truth + 'static>(&mut self) -> Option<T> {
        if self.has_truth::<T>() {
            Some(Self::unset_truth_unchecked::<T>(self))
        } else {
            None
        }
    }

    pub fn unset_truth_unchecked<T: Truth + 'static>(&mut self) -> T {
        *self.state.remove(T::id())
            .expect(format!("State does not contain required truth {}", T::id()).as_str())
            .downcast::<T>()
            .expect(format!("State has invalid type for truth {}", T::id()).as_str())
    }
}