use std::collections::HashSet;

use crate::{Id, State, Truth};

/// A trait that represents a transition parameter.
/// 
/// A transition parameter is a piece of data that can be passed as a function parameter to a transition.
/// 
/// It is implemented for:
/// 
/// - `Truth` types
/// - `Option<Truth>` types
/// - Tuples of up to 8 `TransitionParam` types
pub trait TransitionParam {

    /// Takes the required truth from the state.
    /// 
    /// This function will take the required truth from the state and return it. If the state does not
    /// contain the required truth, this function will panic.
    /// 
    /// # Panics
    /// 
    /// This function will panic if the state does not contain the required truth.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm_core::{Truth, transition::TransitionParam};
    /// use pssm_macro::*;
    /// use std::collections::HashMap;
    /// use std::any::Any;
    /// 
    /// #[derive(Truth,Debug)]
    /// struct A();
    /// 
    /// let mut state = HashMap::new();
    /// state.insert(A::id(), Box::new(A()) as Box<dyn Any>);
    /// 
    /// A::take_from(&mut state);
    /// ```
    fn take_from(state: &mut State) -> Self;

    /// Collects the required truths for this parameter.
    /// 
    /// This function will call the given closure with the id of each required truth. If the closure
    /// returns an error, this function will return that error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm_core::{Truth, transition::TransitionParam};
    /// use pssm_macro::*;
    /// 
    /// #[derive(Truth)]
    /// struct A();
    /// 
    /// let mut vec = Vec::new();
    /// 
    /// A::collect_required(&mut |id| {
    ///    vec.push(id);
    ///    Ok::<(),()>(())
    /// }).unwrap();
    /// 
    /// assert_eq!(vec.len(), 1);
    /// ```
    fn collect_required<C,E>(collector: &mut C) -> Result<(),E>
    where 
        C: FnMut(Id) -> Result<(),E>;

    /// Collects the required truths for this parameter.
    /// 
    /// This function will return a set of the required truth ids for this parameter. If the same truth
    /// is required multiple times, this function will return an error.
    /// 
    /// # Examples	
    /// 
    /// ```
    /// use pssm_core::{Truth, transition::TransitionParam};
    /// use pssm_macro::*;
    /// 
    /// #[derive(Truth,Debug,PartialEq,Eq)]
    /// struct A();
    /// 
    /// let ids = A::required().unwrap();
    /// 
    /// assert_eq!(ids.len(), 1);
    /// ```
    fn required() -> Result<HashSet<Id>,&'static str> {
        let mut ids = HashSet::new();
        Self::collect_required(&mut |id| { 
            if ids.contains(&id) {
                Err("Transition requires the same truth multiple times")
            } else {
                ids.insert(id);
                Ok(())
            }
        }).map(|_| ids)
    }
}

impl<T> TransitionParam for T 
where 
    T: Truth + 'static
{
    fn take_from(state: &mut State) -> Self {
        *state.remove(&T::id())
            .expect("State does not contain a required truth")
            .downcast::<T>()
            .expect("Invalid type stored for truth")
    }

    fn collect_required<C,E>(collector: &mut C) -> Result<(),E>
    where 
        C: FnMut(Id) -> Result<(),E>
    {
        collector(T::id())
    }
}

impl<T> TransitionParam for Option<T> 
where 
    T: Truth + 'static
{
    fn take_from(state: &mut State) -> Self {
        state.remove(&T::id())
            .map(|val| *val.downcast::<T>().expect("Invalid type stored for truth"))
    }

    fn collect_required<C,E>(_: &mut C) -> Result<(),E>
    where 
        C: FnMut(Id) -> Result<(),E>
    {
        Ok(())
    }
}

impl TransitionParam for () {
    fn take_from(_: &mut State) -> Self {
        ()
    }

    fn collect_required<C,E>(_: &mut C) -> Result<(),E>
    where 
        C: FnMut(Id) -> Result<(),E>
    {
        Ok(())
    }
}

impl<A> TransitionParam for (A,) 
where 
    A: TransitionParam
{
    fn take_from(state: &mut State) -> Self {
        (A::take_from(state),)
    }

    fn collect_required<C,E>(collector: &mut C) -> Result<(),E>
    where 
        C: FnMut(Id) -> Result<(),E>
    {
        A::collect_required(collector)
    }
}

impl<A,B> TransitionParam for (A, B) 
where 
    A: TransitionParam,
    B: TransitionParam
{
    fn take_from(state: &mut State) -> Self {
        (A::take_from(state), B::take_from(state))
    }

    fn collect_required<C,E>(collector: &mut C) -> Result<(),E>
    where 
        C: FnMut(Id) -> Result<(),E>
    {
        A::collect_required(collector)?;
        B::collect_required(collector)
    }
}

impl<A,B,C> TransitionParam for (A, B, C) 
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam
{
    fn take_from(state: &mut State) -> Self {
        (A::take_from(state), B::take_from(state), C::take_from(state))
    }

    fn collect_required<Col,Err>(collector: &mut Col) -> Result<(),Err>
    where 
        Col: FnMut(Id) -> Result<(),Err>
    {
        A::collect_required(collector)?;
        B::collect_required(collector)?;
        C::collect_required(collector)
    }
}

impl<A,B,C,D> TransitionParam for (A, B, C, D) 
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam
{
    fn take_from(state: &mut State) -> Self {
        (A::take_from(state), B::take_from(state), C::take_from(state), D::take_from(state))
    }

    fn collect_required<Col,Err>(collector: &mut Col) -> Result<(),Err>
    where 
        Col: FnMut(Id) -> Result<(),Err>
    {
        A::collect_required(collector)?;
        B::collect_required(collector)?;
        C::collect_required(collector)?;
        D::collect_required(collector)
    }
}

impl<A,B,C,D,E> TransitionParam for (A, B, C, D, E) 
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    E: TransitionParam
{
    fn take_from(state: &mut State) -> Self {
        (A::take_from(state), B::take_from(state), C::take_from(state), D::take_from(state), E::take_from(state))
    }

    fn collect_required<Col,Err>(collector: &mut Col) -> Result<(),Err>
    where 
        Col: FnMut(Id) -> Result<(),Err>
    {
        A::collect_required(collector)?;
        B::collect_required(collector)?;
        C::collect_required(collector)?;
        D::collect_required(collector)?;
        E::collect_required(collector)
    }
}

impl<A,B,C,D,E,F> TransitionParam for (A, B, C, D, E, F) 
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    E: TransitionParam,
    F: TransitionParam
{
    fn take_from(state: &mut State) -> Self {
        (A::take_from(state), B::take_from(state), C::take_from(state), D::take_from(state), E::take_from(state), F::take_from(state))
    }

    fn collect_required<Col,Err>(collector: &mut Col) -> Result<(),Err>
    where 
        Col: FnMut(Id) -> Result<(),Err>
    {
        A::collect_required(collector)?;
        B::collect_required(collector)?;
        C::collect_required(collector)?;
        D::collect_required(collector)?;
        E::collect_required(collector)?;
        F::collect_required(collector)
    }
}

impl<A,B,C,D,E,F,G> TransitionParam for (A, B, C, D, E, F, G) 
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    E: TransitionParam,
    F: TransitionParam,
    G: TransitionParam
{
    fn take_from(state: &mut State) -> Self {
        (A::take_from(state), B::take_from(state), C::take_from(state), D::take_from(state), E::take_from(state), F::take_from(state), G::take_from(state))
    }

    fn collect_required<Col,Err>(collector: &mut Col) -> Result<(),Err>
    where 
        Col: FnMut(Id) -> Result<(),Err>
    {
        A::collect_required(collector)?;
        B::collect_required(collector)?;
        C::collect_required(collector)?;
        D::collect_required(collector)?;
        E::collect_required(collector)?;
        F::collect_required(collector)?;
        G::collect_required(collector)
    }
}

impl<A,B,C,D,E,F,G,H> TransitionParam for (A, B, C, D, E, F, G, H) 
where 
    A: TransitionParam,
    B: TransitionParam,
    C: TransitionParam,
    D: TransitionParam,
    E: TransitionParam,
    F: TransitionParam,
    G: TransitionParam,
    H: TransitionParam
{
    fn take_from(state: &mut State) -> Self {
        (A::take_from(state), B::take_from(state), C::take_from(state), D::take_from(state), E::take_from(state), F::take_from(state), G::take_from(state), H::take_from(state))
    }

    fn collect_required<Col,Err>(collector: &mut Col) -> Result<(),Err>
    where 
        Col: FnMut(Id) -> Result<(),Err>
    {
        A::collect_required(collector)?;
        B::collect_required(collector)?;
        C::collect_required(collector)?;
        D::collect_required(collector)?;
        E::collect_required(collector)?;
        F::collect_required(collector)?;
        G::collect_required(collector)?;
        H::collect_required(collector)
    }
}