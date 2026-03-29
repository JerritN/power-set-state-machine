use std::{collections::HashSet, fmt::Debug};

use crate::{Id, transition::InvalidTransitionError};

use super::{IntoTransition, IntoTransitionMut, IntoTransitionOnce, Transition, TransitionMut, TransitionOnce};

fn combine_requirements(
    mut requires1: HashSet<Id>,
    produces1: HashSet<Id>,
    requires2: HashSet<Id>,
    mut produces2: HashSet<Id>
) -> Result<(HashSet<Id>,HashSet<Id>),AndThenError> {
    for id in requires1.intersection(&requires2) {
        if !produces1.contains(id) {
            return Err(AndThenError::ConflictingRequirements);
        }
    }

    requires1.extend(requires2.difference(&produces1).cloned());
    produces2.extend(produces1.difference(&requires2).cloned());

    Ok((requires1,produces2))
}

pub enum AndThenError {
    ConflictingRequirements,
    IntoTransitionError(InvalidTransitionError)
}

impl From<InvalidTransitionError> for AndThenError {
    fn from(value: InvalidTransitionError) -> Self {
        AndThenError::IntoTransitionError(value)
    }
}

impl Debug for AndThenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AndThenError::ConflictingRequirements => write!(f, "Both transitions require the same input, but the first transition does not produce it."),
            AndThenError::IntoTransitionError(e) => e.fmt(f)
        }
    }
}

/// A trait for chaining transitions together.
/// 
/// This trait is used to chain transitions together, creating a new transition that runs the first transition
/// followed by the second transition.
/// 
/// # Examples
/// 
/// ```
/// use pssm::prelude::*;
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
pub trait AndThen<'a,InA> {

    /// Chains this transition with the given transition.
    /// 
    /// This function will chain this transition with the given transition, creating a new transition that runs
    /// this transition followed by the given transition.
    /// 
    /// If both transitions require the same input, but the first transition does not produce it, an error will be returned.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm::prelude::*;
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
    fn and_then<Next,InB>(self, next: Next) -> Result<Transition<'a>,AndThenError>
    where
        Next: IntoTransition<'a,InB>;
}

/// A trait for chaining mutable transitions together.
/// 
/// This trait is used to chain mutable transitions together, creating a new mutable transition that runs the first transition
/// followed by the second transition.
/// 
/// # Examples
/// 
/// ```
/// use pssm::prelude::*;
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
/// state_machine.run(insert_a.and_then_mut(collect_a).unwrap());
/// 
/// assert_eq!(vec.len(), 1);
/// ```
pub trait AndThenMut<'a,InA> {
    /// Chains this transition with the given transition.
    /// 
    /// This function will chain this transition with the given transition, creating a new mutable transition that runs
    /// this transition followed by the given transition.
    /// 
    /// If both transitions require the same input, but the first transition does not produce it, an error will be returned.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm::prelude::*;
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
    /// state_machine.run(insert_a.and_then_mut(collect_a).unwrap());
    /// 
    /// assert_eq!(vec.len(), 1);
    /// ```
    fn and_then_mut<Next,InB>(self, next: Next) -> Result<TransitionMut<'a>,AndThenError>
    where
        Next: IntoTransitionMut<'a,InB>;
}

/// A trait for chaining transitions together.
/// 
/// This trait is used to chain transitions together, creating a new transition that runs the first transition
/// followed by the second transition.
/// 
/// # Examples
/// 
/// ```
/// use pssm::prelude::*;
/// 
/// #[derive(Truth,Debug)]
/// struct A();
/// 
/// let a = A();
/// let insert_a = move || a;
/// 
/// let consume_a = |a: A| {
///    println!("{:?}", a);
/// };
/// 
/// let mut state_machine = StateMachine::new();
/// state_machine.run(insert_a.and_then_once(consume_a).unwrap());
/// ```
pub trait AndThenOnce<'a,InA> {

    /// Chains this transition with the given transition.
    /// 
    /// This function will chain this transition with the given transition, creating a new transition that runs
    /// this transition followed by the given transition.
    /// 
    /// If both transitions require the same input, but the first transition does not produce it, an error will be returned.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm::prelude::*;
    /// 
    /// #[derive(Truth,Debug)]
    /// struct A();
    /// 
    /// let a = A();
    /// let insert_a = move || a;
    /// 
    /// let consume_a = |a: A| {
    ///   println!("{:?}", a);
    /// };
    /// 
    /// let mut state_machine = StateMachine::new();
    /// state_machine.run(insert_a.and_then_once(consume_a).unwrap());
    /// ```
    fn and_then_once<Next,InB>(self, next: Next) -> Result<TransitionOnce<'a>,AndThenError>
    where
        Next: IntoTransitionOnce<'a,InB>;
}

impl<'a,I,InA> AndThen<'a,InA> for I
where 
    I: IntoTransition<'a,InA>
{
    fn and_then<Next,InB>(self, next: Next) -> Result<Transition<'a>,AndThenError>
    where Next: IntoTransition<'a,InB> {
        let t1 = self.into_transition()?;
        let t2 = next.into_transition()?;

        let (requires,produces) = combine_requirements(t1.requires,t1.produces,t2.requires,t2.produces)?;

        Ok(Transition::new(
            move |args| {
                (t1.func)(args)?;
                (t2.func)(args)?;
                Ok(())
            },
            requires,
            produces
        ))
    }
}

impl<'a,I,InA> AndThenMut<'a,InA> for I
where 
    I: IntoTransitionMut<'a,InA>
{
    fn and_then_mut<Next,InB>(self, next: Next) -> Result<TransitionMut<'a>,AndThenError>
    where Next: IntoTransitionMut<'a,InB> {
        let mut t1 = self.into_transition_mut()?;
        let mut t2 = next.into_transition_mut()?;

        let (requires,produces) = combine_requirements(t1.requires,t1.produces,t2.requires,t2.produces)?;

        Ok(TransitionMut::new(
            move |args| {
                (t1.func)(args)?;
                (t2.func)(args)?;
                Ok(())
            },
            requires,
            produces
        ))
    }
}

impl<'a,I,InA> AndThenOnce<'a,InA> for I
where 
    I: IntoTransitionOnce<'a,InA>
{
    fn and_then_once<Next,InB>(self, next: Next) -> Result<TransitionOnce<'a>,AndThenError>
    where Next: IntoTransitionOnce<'a,InB> {
        let t1 = self.into_transition_once()?;
        let t2 = next.into_transition_once()?;

        let (requires,produces) = combine_requirements(t1.requires,t1.produces,t2.requires,t2.produces)?;

        Ok(TransitionOnce::new(
            move |args| {
                (t1.func)(args)?;
                (t2.func)(args)?;
                Ok(())
            },
            requires,
            produces
        ))
    }
}