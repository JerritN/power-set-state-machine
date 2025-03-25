use std::collections::HashSet;

use crate::{Id, State, Truth};

/// A trait that represents a transition result.
/// 
/// A transition result is a piece of data that can be returned from a transition.
/// 
/// It is implemented for:
/// 
/// - `Truth` types
/// - `Option<Truth>` types
/// - Tuples of up to 8 `TransitionResult` types
pub trait TransitionResult {

    /// Inserts the transition result into the state.
    /// 
    /// This function will insert the transition result into the state.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm_core::{Truth, transition::TransitionResult};
    /// use pssm_macro::*;
    /// use std::collections::HashMap;
    /// 
    /// #[derive(Truth)]
    /// struct A();
    /// 
    /// let mut state = HashMap::new();
    /// A().insert_into(&mut state);
    /// 
    /// assert!(state.contains_key(&A::id()));
    /// ```
    fn insert_into(self, state: &mut State);

    /// Collects the ids produced by this transition result.
    /// 
    /// This function will call the given closure with the id of each truth produced by this transition result.
    /// Returns an error if the collected truth is already produced.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm_core::{Truth, transition::TransitionResult};
    /// use pssm_macro::*;
    /// use std::collections::HashSet;
    /// 
    /// #[derive(Truth)]
    /// struct A();
    /// 
    /// let mut set = HashSet::new();
    /// 
    /// A::collect_produces(|id| {
    ///   set.insert(id);
    ///  Ok::<(),()>(())
    /// });
    /// 
    /// assert_eq!(set.len(), 1);
    /// ```
    fn collect_produces<C,E>(collector: C) -> Result<(),E>
    where 
        C: FnMut(Id) -> Result<(),E>;

    /// Produces the ids produced by this transition result.
    /// 
    /// This function will return the ids produced by this transition result.
    /// Returns an error if the transition produces the same id multiple times.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use pssm_core::{Truth, transition::TransitionResult};
    /// use pssm_macro::*;
    /// use std::collections::HashSet;
    /// 
    /// #[derive(Truth)]
    /// struct A();
    /// 
    /// let ids = A::produces().unwrap();
    /// 
    /// assert_eq!(ids.len(), 1);
    /// ```
    fn produces() -> Result<HashSet<Id>,&'static str> {
        let mut ids = HashSet::new();
        Self::collect_produces(|id| {
            if ids.contains(&id) {
                Err("Transition produces the same id multiple times.")
            } else {
                ids.insert(id);
                Ok(())
            }
        }).map(|_| ids)
    }
}

impl<T: Truth + 'static> TransitionResult for T {
    fn insert_into(self, state: &mut State) {
        state.insert(T::id(), Box::new(self));
    }

    fn collect_produces<C,E>(mut collector: C) -> Result<(),E>
    where 
        C: FnMut(Id) -> Result<(),E>
    {
        collector(T::id())
    }
}

impl<A> TransitionResult for Option<A>
where 
    A: TransitionResult
{
    fn insert_into(self, state: &mut State) {
        if let Some(a) = self {
            a.insert_into(state);
        }
    }

    fn collect_produces<C,E>(_: C) -> Result<(),E>
    where 
        C: FnMut(Id) -> Result<(),E>
    {
        Ok(())
    }
}

impl TransitionResult for () {
    fn insert_into(self, _: &mut State) {}

    fn collect_produces<C,E>(_: C) -> Result<(),E>
    where 
        C: FnMut(Id) -> Result<(),E>
    {
        Ok(())
    }
}

impl<A> TransitionResult for (A,) 
where 
    A: TransitionResult
{
    fn insert_into(self, state: &mut State) {
        let (a,) = self;
        a.insert_into(state);
    }

    fn collect_produces<C,E>(mut collector: C) -> Result<(),E>
    where 
        C: FnMut(Id) -> Result<(),E>
    {
        A::collect_produces(&mut collector)
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

    fn collect_produces<C,E>(mut collector: C) -> Result<(),E>
    where 
        C: FnMut(Id) -> Result<(),E>
    {
        A::collect_produces(&mut collector)?;
        B::collect_produces(&mut collector)
    }
}

impl<A, B, C> TransitionResult for (A, B, C) 
where 
    A: TransitionResult,
    B: TransitionResult,
    C: TransitionResult
{
    fn insert_into(self, state: &mut State) {
        let (a,b,c) = self;
        a.insert_into(state);
        b.insert_into(state);
        c.insert_into(state);
    }

    fn collect_produces<Col,E>(mut collector: Col) -> Result<(),E>
    where 
        Col: FnMut(Id) -> Result<(),E>
    {
        A::collect_produces(&mut collector)?;
        B::collect_produces(&mut collector)?;
        C::collect_produces(&mut collector)
    }
}

impl<A, B, C, D> TransitionResult for (A, B, C, D) 
where 
    A: TransitionResult,
    B: TransitionResult,
    C: TransitionResult,
    D: TransitionResult
{
    fn insert_into(self, state: &mut State) {
        let (a,b,c,d) = self;
        a.insert_into(state);
        b.insert_into(state);
        c.insert_into(state);
        d.insert_into(state);
    }

    fn collect_produces<Col,E>(mut collector: Col) -> Result<(),E>
    where 
        Col: FnMut(Id) -> Result<(),E>
    {
        A::collect_produces(&mut collector)?;
        B::collect_produces(&mut collector)?;
        C::collect_produces(&mut collector)?;
        D::collect_produces(&mut collector)
    }
}

impl<A, B, C, D, E> TransitionResult for (A, B, C, D, E) 
where 
    A: TransitionResult,
    B: TransitionResult,
    C: TransitionResult,
    D: TransitionResult,
    E: TransitionResult
{
    fn insert_into(self, state: &mut State) {
        let (a,b,c,d,e) = self;
        a.insert_into(state);
        b.insert_into(state);
        c.insert_into(state);
        d.insert_into(state);
        e.insert_into(state);
    }

    fn collect_produces<Col,Err>(mut collector: Col) -> Result<(),Err>
    where 
        Col: FnMut(Id) -> Result<(),Err>
    {
        A::collect_produces(&mut collector)?;
        B::collect_produces(&mut collector)?;
        C::collect_produces(&mut collector)?;
        D::collect_produces(&mut collector)?;
        E::collect_produces(&mut collector)
    }
}

impl<A, B, C, D, E, F> TransitionResult for (A, B, C, D, E, F) 
where 
    A: TransitionResult,
    B: TransitionResult,
    C: TransitionResult,
    D: TransitionResult,
    E: TransitionResult,
    F: TransitionResult
{
    fn insert_into(self, state: &mut State) {
        let (a,b,c,d,e,f) = self;
        a.insert_into(state);
        b.insert_into(state);
        c.insert_into(state);
        d.insert_into(state);
        e.insert_into(state);
        f.insert_into(state);
    }

    fn collect_produces<Col,Err>(mut collector: Col) -> Result<(),Err>
    where 
        Col: FnMut(Id) -> Result<(),Err>
    {
        A::collect_produces(&mut collector)?;
        B::collect_produces(&mut collector)?;
        C::collect_produces(&mut collector)?;
        D::collect_produces(&mut collector)?;
        E::collect_produces(&mut collector)?;
        F::collect_produces(&mut collector)
    }
}

impl<A, B, C, D, E, F, G> TransitionResult for (A, B, C, D, E, F, G) 
where 
    A: TransitionResult,
    B: TransitionResult,
    C: TransitionResult,
    D: TransitionResult,
    E: TransitionResult,
    F: TransitionResult,
    G: TransitionResult
{
    fn insert_into(self, state: &mut State) {
        let (a,b,c,d,e,f,g) = self;
        a.insert_into(state);
        b.insert_into(state);
        c.insert_into(state);
        d.insert_into(state);
        e.insert_into(state);
        f.insert_into(state);
        g.insert_into(state);
    }

    fn collect_produces<Col,Err>(mut collector: Col) -> Result<(),Err>
    where 
        Col: FnMut(Id) -> Result<(),Err>
    {
        A::collect_produces(&mut collector)?;
        B::collect_produces(&mut collector)?;
        C::collect_produces(&mut collector)?;
        D::collect_produces(&mut collector)?;
        E::collect_produces(&mut collector)?;
        F::collect_produces(&mut collector)?;
        G::collect_produces(&mut collector)
    }
}

impl<A, B, C, D, E, F, G, H> TransitionResult for (A, B, C, D, E, F, G, H) 
where 
    A: TransitionResult,
    B: TransitionResult,
    C: TransitionResult,
    D: TransitionResult,
    E: TransitionResult,
    F: TransitionResult,
    G: TransitionResult,
    H: TransitionResult
{
    fn insert_into(self, state: &mut State) {
        let (a,b,c,d,e,f,g,h) = self;
        a.insert_into(state);
        b.insert_into(state);
        c.insert_into(state);
        d.insert_into(state);
        e.insert_into(state);
        f.insert_into(state);
        g.insert_into(state);
        h.insert_into(state);
    }

    fn collect_produces<Col,Err>(mut collector: Col) -> Result<(),Err>
    where 
        Col: FnMut(Id) -> Result<(),Err>
    {
        A::collect_produces(&mut collector)?;
        B::collect_produces(&mut collector)?;
        C::collect_produces(&mut collector)?;
        D::collect_produces(&mut collector)?;
        E::collect_produces(&mut collector)?;
        F::collect_produces(&mut collector)?;
        G::collect_produces(&mut collector)?;
        H::collect_produces(&mut collector)
    }
}