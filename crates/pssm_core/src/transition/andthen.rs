use std::collections::HashSet;

use crate::Id;

use super::{IntoTransition, IntoTransitionMut, IntoTransitionOnce, Transition, TransitionMut, TransitionOnce};

fn combine_requirements(
    mut requires1: HashSet<Id>,
    produces1: HashSet<Id>,
    requires2: HashSet<Id>,
    mut produces2: HashSet<Id>
) -> Result<(HashSet<Id>,HashSet<Id>),&'static str> {
    for id in requires1.intersection(&requires2) {
        if !produces1.contains(id) {
            return Err("Both transitions require the same parameter, but the first transition does not produce it.");
        }
    }

    requires1.extend(requires2.difference(&produces1).cloned());
    produces2.extend(produces1.difference(&requires2).cloned());

    Ok((requires1,produces2))
}

/// A trait for chaining transitions together.
/// 
/// This trait is used to chain transitions together, creating a new transition that runs the first transition
/// followed by the second transition.
/// 
/// # Examples
/// 
/// ```
/// use pssm_core::{Truth, StateMachine, transition::{Transition, IntoTransition, AndThen}};
/// use pssm_macro::*;
/// 
/// #[derive(Truth,Debug)]
/// struct A();
/// 
/// fn insert_a() -> A {
///    A()
/// }
/// 
/// fn consume_a(a: A) {
///    println!("{:?}", a);
/// }
/// 
/// let mut state_machine = StateMachine::new();
/// state_machine.run(insert_a.and_then(consume_a).unwrap());
/// ```
pub trait AndThen<'a,InA,MarkerA> {

    /// Chains this transition with the given transition.
    /// 
    /// This function will chain this transition with the given transition, creating a new transition that runs
    /// this transition followed by the given transition.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm_core::{Truth, StateMachine, transition::{Transition, IntoTransition, AndThen}};
    /// use pssm_macro::*;
    /// 
    /// #[derive(Truth,Debug)]
    /// struct A();
    /// 
    /// fn insert_a() -> A {
    ///   A()
    /// }
    /// 
    /// fn consume_a(a: A) {
    ///  println!("{:?}", a);
    /// }
    /// 
    /// let mut state_machine = StateMachine::new();
    /// state_machine.run(insert_a.and_then(consume_a).unwrap());
    /// ```
    fn and_then<Next,InB,MarkerB>(self, next: Next) -> Result<Transition<'a>,&'static str>
    where
        Next: IntoTransition<'a,InB,MarkerB>;
}

/// A trait for chaining mutable transitions together.
/// 
/// This trait is used to chain mutable transitions together, creating a new mutable transition that runs the first transition
/// followed by the second transition.
/// 
/// # Examples
/// 
/// ```
/// use pssm_core::{Truth, StateMachine, transition::{TransitionMut, IntoTransitionMut, AndThenMut}};
/// use pssm_macro::*;
/// 
/// #[derive(Truth,Debug)]
/// struct A();
/// 
/// fn insert_a() -> A {
///   A()
/// }
/// 
/// let mut vec = Vec::new();
/// 
/// let collect_a = |a: A| {
///     &mut vec.push(a);
/// };
/// 
/// let mut state_machine = StateMachine::new();
/// state_machine.run(insert_a.and_then(collect_a).unwrap());
/// 
/// assert_eq!(vec.len(), 1);
/// ```
pub trait AndThenMut<'a,InA,MarkerA> {
    /// Chains this transition with the given transition.
    /// 
    /// This function will chain this transition with the given transition, creating a new mutable transition that runs
    /// this transition followed by the given transition.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm_core::{Truth, StateMachine, transition::{TransitionMut, IntoTransitionMut, AndThenMut}};
    /// use pssm_macro::*;
    ///     
    /// #[derive(Truth,Debug)]
    /// struct A();
    /// 
    /// fn insert_a() -> A {
    ///  A()
    /// }
    /// 
    /// let mut vec = Vec::new();
    /// 
    /// let collect_a = |a: A| {
    ///    &mut vec.push(a);
    /// };
    /// 
    /// let mut state_machine = StateMachine::new();
    /// state_machine.run(insert_a.and_then(collect_a).unwrap());
    /// 
    /// assert_eq!(vec.len(), 1);
    /// ```
    fn and_then<Next,InB,MarkerB>(self, next: Next) -> Result<TransitionMut<'a>,&'static str>
    where
        Next: IntoTransitionMut<'a,InB,MarkerB>;
}

/// A trait for chaining transitions together.
/// 
/// This trait is used to chain transitions together, creating a new transition that runs the first transition
/// followed by the second transition.
/// 
/// # Examples
/// 
/// ```
/// use pssm_core::{Truth, StateMachine, transition::{TransitionOnce, IntoTransitionOnce, AndThenOnce}};
/// use pssm_macro::*;
/// 
/// #[derive(Truth,Debug)]
/// struct A();
/// 
/// let insert_a = || A();
/// 
/// let consume_a = |a: A| {
///    println!("{:?}", a);
/// };
/// 
/// let mut state_machine = StateMachine::new();
/// state_machine.run(insert_a.and_then(consume_a).unwrap());
/// ```
pub trait AndThenOnce<'a,InA,MarkerA> {

    /// Chains this transition with the given transition.
    /// 
    /// This function will chain this transition with the given transition, creating a new transition that runs
    /// this transition followed by the given transition.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm_core::{Truth, StateMachine, transition::{TransitionOnce, IntoTransitionOnce, AndThenOnce}};
    /// use pssm_macro::*;
    /// 
    /// #[derive(Truth,Debug)]
    /// struct A();
    /// 
    /// let insert_a = || A();
    /// 
    /// let consume_a = |a: A| {
    ///   println!("{:?}", a);
    /// };
    /// 
    /// let mut state_machine = StateMachine::new();
    /// state_machine.run(insert_a.and_then(consume_a).unwrap());
    /// ```
    fn and_then<Next,InB,MarkerB>(self, next: Next) -> Result<TransitionOnce<'a>,&'static str>
    where
        Next: IntoTransitionOnce<'a,InB,MarkerB>;
}

impl<'a,I,InA,MarkerA> AndThen<'a,InA,MarkerA> for I
where 
    I: IntoTransition<'a,InA,MarkerA>
{
    fn and_then<Next,InB,MarkerB>(self, next: Next) -> Result<Transition<'a>,&'static str>
    where Next: IntoTransition<'a,InB,MarkerB> {
        let t1 = self.into_transition()?;
        let t2 = next.into_transition()?;

        let (requires,produces) = combine_requirements(t1.requires,t1.produces,t2.requires,t2.produces)?;

        Ok(Transition::new(
            move |args| {
                (t1.func)(args);
                (t2.func)(args);
            },
            requires,
            produces
        ))
    }
}

impl<'a,I,InA,MarkerA> AndThenMut<'a,InA,MarkerA> for I
where 
    I: IntoTransitionMut<'a,InA,MarkerA>
{
    fn and_then<Next,InB,MarkerB>(self, next: Next) -> Result<TransitionMut<'a>,&'static str>
    where Next: IntoTransitionMut<'a,InB,MarkerB> {
        let mut t1 = self.into_transition_mut()?;
        let mut t2 = next.into_transition_mut()?;

        let (requires,produces) = combine_requirements(t1.requires,t1.produces,t2.requires,t2.produces)?;

        Ok(TransitionMut::new(
            move |args| {
                (t1.func)(args);
                (t2.func)(args);
            },
            requires,
            produces
        ))
    }
}

impl<'a,I,InA,MarkerA> AndThenOnce<'a,InA,MarkerA> for I
where 
    I: IntoTransitionOnce<'a,InA,MarkerA>
{
    fn and_then<Next,InB,MarkerB>(self, next: Next) -> Result<TransitionOnce<'a>,&'static str>
    where Next: IntoTransitionOnce<'a,InB,MarkerB> {
        let t1 = self.into_transition_once()?;
        let t2 = next.into_transition_once()?;

        let (requires,produces) = combine_requirements(t1.requires,t1.produces,t2.requires,t2.produces)?;

        Ok(TransitionOnce::new(
            move |args| {
                (t1.func)(args);
                (t2.func)(args);
            },
            requires,
            produces
        ))
    }
}