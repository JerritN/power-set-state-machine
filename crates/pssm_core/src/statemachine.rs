use std::collections::HashMap;

use crate::{State, TransitionCallError, Truth};
use crate::transition::{InvalidTransitionError, IntoTransitionOnce, IntoTransitionOnceParameterized, Transition, TransitionError, TransitionMut, TransitionOnce};
use crate::transition::function::TransitionInput;

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
/// use pssm::prelude::*;
/// 
/// #[derive(Truth)]
/// struct A(i32);
/// 
/// #[derive(Truth)]
/// struct B(i32);
/// 
/// fn insert_a(Param(param): Param<i32>) -> A {
///    A(param)
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
/// let first = into_transition_with!(insert_a, 5).unwrap();
/// let second = insert_b.and_then(combine).unwrap();
/// state_machine.run(first).unwrap();
/// state_machine.run(second).unwrap();
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
    /// use pssm_core::StateMachine;
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
    /// use pssm::prelude::*;
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
    #[allow(private_bounds)]
    pub fn can_run<'a,T,In>(&self, _: &T) -> Result<bool,InvalidTransitionError>
    where 
        In: TransitionInput,
        T: IntoTransitionOnce<'a,In>
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
    /// use pssm::prelude::*;
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
    /// use pssm::prelude::*;
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
    /// use pssm::prelude::*;
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
    /// use pssm::prelude::*;
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
    pub fn run<'a,T,In>(&mut self, transition: T) -> Result<(),TransitionCallError>
    where 
        T: IntoTransitionOnce<'a,In>
    {
        let transition = transition.into_transition_once()?;
        transition.requires().iter().try_for_each(|id| {
            if self.state.contains_key(id) {
                Ok(())
            } else {
                Err(TransitionError::MissingTruth(*id))
            }
        })?;
        transition.run(&mut self.state)?;
        Ok(())
    }

    /// Runs a transition with parameters.
    /// 
    /// This function will run the transition if all the required truths are in the state.
    /// If the transition requires a truth that is not in the state, this function will return an error.
    /// 
    /// If the `IntoTransitionOnceParameterized` object can not be converted into a `TransitionOnce`, this function will return an error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm::prelude::*;
    /// 
    /// #[derive(Truth,Debug)]
    /// struct A(i32);
    /// 
    /// fn add(a: A, Param(amount): Param<i32>) -> A {
    ///   A(a.0 + amount)
    /// }
    /// 
    /// let mut state_machine = StateMachine::new();
    /// state_machine.set_truth(A(5));
    /// 
    /// state_machine.run_with(add, (10,)).unwrap();
    /// 
    /// let a = state_machine.unset_truth::<A>().unwrap();
    /// assert_eq!(a.0, 15);
    /// ```
    pub fn run_with<'a,T,In,Param>(&mut self, transition: T, params: Param) -> Result<(),TransitionCallError>
    where 
        T: IntoTransitionOnceParameterized<'a,In,Param>
    {
        let transition = transition.into_transition_once_with(params)?;
        transition.requires().iter().try_for_each(|id| {
            if self.state.contains_key(id) {
                Ok(())
            } else {
                Err(TransitionError::MissingTruth(*id))
            }
        })?;
        transition.run(&mut self.state)?;
        Ok(())
    }

    /// Runs a `TransitionOnce`.
    /// 
    /// This function will run the `TransitionOnce` if all the required truths are in the state.
    /// If the `TransitionOnce` requires a truth that is not in the state, this function will panic.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm::prelude::*;
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
        transition.run(&mut self.state).unwrap();
    }

    /// Runs a `Transition`.
    /// 
    /// This function will run the `Transition` if all the required truths are in the state.
    /// If the `Transition` requires a truth that is not in the state, this function will panic.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm::prelude::*;
    /// 
    /// #[derive(Truth,Debug)]
    /// struct A();
    /// 
    /// let mut state_machine = StateMachine::new();
    /// 
    /// let create_a = || A();
    /// let transition = create_a.into_transition().unwrap();
    /// state_machine.run_ref_unchecked(&transition);
    /// 
    /// assert!(state_machine.has_truth::<A>());
    /// ```
    pub fn run_ref_unchecked(&mut self, transition: &Transition)
    {
        transition.run(&mut self.state).unwrap();
    }
    
    /// Runs a `TransitionMut`.
    /// 
    /// This function will run the `TransitionMut` if all the required truths are in the state.
    /// If the `TransitionMut` requires a truth that is not in the state, this function will panic.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm::prelude::*;
    /// 
    /// #[derive(Truth,Debug)]
    /// struct A();
    /// 
    /// let mut state_machine = StateMachine::new();
    /// state_machine.set_truth(A());
    /// 
    /// let mut vec = Vec::new();
    /// let take_a = |a: A| _ = &mut vec.push(a);
    /// let mut transition = take_a.into_transition_mut().unwrap();
    /// state_machine.run_ref_mut_unchecked(&mut transition);
    /// 
    /// drop(transition);
    /// 
    /// assert_eq!(vec.len(), 1);
    /// ```
    pub fn run_ref_mut_unchecked(&mut self, transition: &mut TransitionMut)
    {
        transition.run(&mut self.state).unwrap();
    }

    /// Sets a truth in the state.
    /// 
    /// This function will insert the truth into the state.
    /// If a truth of the same type is already in the state, this function will replace it.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm::prelude::*;
    /// 
    /// #[derive(Truth)]
    /// struct A();
    /// 
    /// let mut state_machine = StateMachine::new();
    /// state_machine.set_truth(A());
    /// 
    /// assert!(state_machine.has_truth::<A>());
    /// ```
    pub fn set_truth<T: Truth + 'static>(&mut self, element: T) {
        self.state.insert(T::id(), Box::new(element));
    }

    /// Checks if a truth is in the state.
    /// 
    /// This function will return true if the truth is in the state.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm::prelude::*;
    /// 
    /// #[derive(Truth)]
    /// struct A();
    /// 
    /// let mut state_machine = StateMachine::new();
    /// 
    /// assert!(!state_machine.has_truth::<A>());
    /// state_machine.set_truth(A());
    /// assert!(state_machine.has_truth::<A>());
    /// ```
    pub fn has_truth<T: Truth + 'static>(&self) -> bool {
        self.state.contains_key(&T::id())
    }

    /// Unsets a truth in the state.
    /// 
    /// This function will remove the truth from the state and return it.
    /// If the truth is not in the state, this function will return None.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm::prelude::*;
    /// 
    /// #[derive(Truth)]
    /// struct A();
    /// 
    /// let mut state_machine = StateMachine::new();
    /// 
    /// assert!(state_machine.unset_truth::<A>().is_err());
    /// state_machine.set_truth(A());
    /// assert!(state_machine.unset_truth::<A>().is_ok());
    /// ```
    pub fn unset_truth<T: Truth + 'static>(&mut self) -> Result<T, TransitionError> {
        T::try_take_from(&mut self.state)
    }
}