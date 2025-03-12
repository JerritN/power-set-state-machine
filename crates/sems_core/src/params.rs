use std::{collections::HashSet};

use crate::{Id, State, Truth};

pub trait TransitionParam {
    fn take_from(state: &mut State) -> Self;

    fn collect_required<C,E>(collector: &mut C) -> Result<(),E>
    where 
        C: FnMut(Id) -> Result<(),E>;

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

    fn collect_required<C,E>(collector: &mut C) -> Result<(),E>
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