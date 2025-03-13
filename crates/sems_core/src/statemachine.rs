use std::collections::HashMap;

use crate::{transition::{IntoTransitionOnce, Transition, TransitionMut, TransitionOnce, TransitionParam}, State, Truth};

/// A state machine that has a state and can run transitions.
/// 
/// The state machine stores the a state in the form of a collection of truths.
/// It can only store one truth of each type.
/// 
/// The state machine can run transitions on the state.
/// It can be checked if a transition can be run before running it.
/// 
/// # Examples
/// 
/// ```
/// use sems_core::{StateMachine, Truth};
/// use sems_macro::*;
/// 
/// #[derive(Truth)]
/// struct A(i32);
/// 
/// #[derive(Truth)]
/// struct B(i32);
/// 
/// fn insert_a() -> A {
///    A(5)
/// }
/// 
/// fn insert_b() -> B {
///   B(10)
/// }
/// 
/// fn combine(a: A, b: B) -> A {
///   A(a.0 + b.0)
/// }
///
/// let mut state_machine = StateMachine::new();
/// 
/// state_machine.run(insert_a).unwrap();
/// state_machine.run(insert_b).unwrap();
/// state_machine.run(combine).unwrap();
/// 
/// let a = state_machine.unset_truth::<A>().unwrap();
/// 
/// assert_eq!(a.0, 15);
/// ```
pub struct StateMachine {
    state: State
}

impl StateMachine {
    /// Creates a new state machine with an empty state.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_core::StateMachine;
    /// 
    /// let state_machine = StateMachine::new();
    /// ```
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    /// Checks if a transition can be run.
    /// 
    /// This function will check if the required truths for the transition are in the state.
    /// 
    /// If the `IntoTransitionOnce` object can not be converted into a `TransitionOnce`, this function will return an error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_core::{StateMachine, Truth};
    /// use sems_macro::*;
    /// 
    /// #[derive(Truth,Debug)]
    /// struct A();
    /// 
    /// fn consume_a(a: A) {
    ///     println!("{:?}", a);
    /// }
    /// 
    /// let mut state_machine = StateMachine::new();
    /// 
    /// assert!(!state_machine.can_run(&consume_a).unwrap());
    /// state_machine.set_truth(A());
    /// assert!(state_machine.can_run(&consume_a).unwrap());
    /// ```
    pub fn can_run<'a,T,In,Marker>(&self, _: &T) -> Result<bool,&'static str>
    where 
        In: TransitionParam,
        T: IntoTransitionOnce<'a,In,Marker>
    {
        Ok(In::required()?.iter().all(|id| self.state.contains_key(id)))
    }

    /// Checks if a `Transition` can be run.
    /// 
    /// This function will check if the required truths for the `Transition` are in the state.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_core::{StateMachine, Truth, transition::IntoTransition};
    /// use sems_macro::*;
    /// 
    /// #[derive(Truth,Debug)]
    /// struct A();
    /// 
    /// fn consume_a(a: A) {
    ///    println!("{:?}", a);
    /// }
    /// 
    /// let mut state_machine = StateMachine::new();
    /// let transition = consume_a.into_transition().unwrap();
    /// 
    /// assert!(!state_machine.can_run_transition(&transition));
    /// state_machine.set_truth(A());
    /// assert!(state_machine.can_run_transition(&transition));
    /// ```
    pub fn can_run_transition(&self, transition: &Transition) -> bool {
        transition.requires().iter().all(|id| self.state.contains_key(id))
    }
    
    /// Checks if a `TransitionMut` can be run.
    /// 
    /// This function will check if the required truths for the `TransitionMut` are in the state.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_core::{StateMachine, Truth, transition::IntoTransitionMut};
    /// use sems_macro::*;
    /// 
    /// #[derive(Truth,Debug)]
    /// struct A();
    /// 
    /// fn consume_a(a: A) {
    ///   println!("{:?}", a);
    /// }
    /// 
    /// let mut state_machine = StateMachine::new();
    /// let transition = consume_a.into_transition_mut().unwrap();
    /// 
    /// assert!(!state_machine.can_run_transition_mut(&transition));
    /// state_machine.set_truth(A());
    /// assert!(state_machine.can_run_transition_mut(&transition));
    /// ```
    pub fn can_run_transition_mut(&self, transition: &TransitionMut) -> bool {
        transition.requires().iter().all(|id| self.state.contains_key(id))
    }

    /// Checks if a `TransitionOnce` can be run.
    /// 
    /// This function will check if the required truths for the `TransitionOnce` are in the state.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_core::{StateMachine, Truth, transition::IntoTransitionOnce};
    /// use sems_macro::*;
    /// 
    /// #[derive(Truth,Debug)]
    /// struct A();
    /// 
    /// fn consume_a(a: A) {
    ///  println!("{:?}", a);
    /// }
    /// 
    /// let mut state_machine = StateMachine::new();
    /// let transition = consume_a.into_transition_once().unwrap();
    /// 
    /// assert!(!state_machine.can_run_transition_once(&transition));
    /// state_machine.set_truth(A());
    /// assert!(state_machine.can_run_transition_once(&transition));
    /// ```
    pub fn can_run_transition_once(&self, transition: &TransitionOnce) -> bool {
        transition.requires().iter().all(|id| self.state.contains_key(id))
    }

    /// Runs a transition.
    /// 
    /// This function will run the transition if all the required truths are in the state.
    /// If the transition requires a truth that is not in the state, this function will return an error.
    /// 
    /// If the `IntoTransitionOnce` object can not be converted into a `TransitionOnce`, this function will return an error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_core::{StateMachine, Truth};
    /// use sems_macro::*;
    /// 
    /// #[derive(Truth,Debug)]
    /// struct A(i32);
    /// 
    /// fn add_one(a: A) -> A {
    ///    A(a.0 + 1)
    /// }
    /// 
    /// let mut state_machine = StateMachine::new();
    /// state_machine.set_truth(A(5));
    /// 
    /// state_machine.run(add_one).unwrap();
    /// 
    /// let a = state_machine.unset_truth::<A>().unwrap();
    /// 
    /// assert_eq!(a.0, 6);
    /// ```
    pub fn run<'a,T,In,Marker>(&mut self, transition: T) -> Result<(),&'static str>
    where 
        T: IntoTransitionOnce<'a,In,Marker>
    {
        let transition = transition.into_transition_once()?;
        if transition.requires().iter().all(|id| self.state.contains_key(id)) {
            transition.run(&mut self.state);
            Ok(())
        } else {
            Err("Missing a required truth")
        }
    }

    /// Runs a `TransitionOnce`.
    /// 
    /// This function will run the `TransitionOnce` if all the required truths are in the state.
    /// If the `TransitionOnce` requires a truth that is not in the state, this function will panic.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use sems_core::{StateMachine, Truth, transition::IntoTransitionOnce};
    /// use sems_macro::*;
    /// 
    /// #[derive(Truth,Debug)]
    /// struct A();
    /// 
    /// let mut state_machine = StateMachine::new();
    /// 
    /// let a = A();
    /// let insert_a = move || a;
    /// let transition = insert_a.into_transition_once().unwrap();
    /// state_machine.run_unchecked(transition);
    ///
    /// assert!(state_machine.has_truth::<A>());
    /// ```
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