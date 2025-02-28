use crate::{Id, State, Truth};

pub trait TransitionParam {
    fn take_from(state: &mut State) -> Self;

    fn collect_ids(collector: &mut dyn FnMut(Id));

    fn ids() -> Vec<Id> {
        let mut ids = Vec::new();
        Self::collect_ids(&mut |id| ids.push(id));
        ids
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

    fn collect_ids(collector: &mut dyn FnMut(Id)) {
        collector(T::id());
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

    fn collect_ids(_: &mut dyn FnMut(Id)) {}
}

impl TransitionParam for () {
    fn take_from(_: &mut State) -> Self {
        ()
    }

    fn collect_ids(_: &mut dyn FnMut(Id)) {}
}

impl<A,B> TransitionParam for (A, B) 
where 
    A: TransitionParam,
    B: TransitionParam
{
    fn take_from(state: &mut State) -> Self {
        (A::take_from(state), B::take_from(state))
    }

    fn collect_ids(collector: &mut dyn FnMut(Id)) {
        A::collect_ids(collector);
        B::collect_ids(collector);
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

    fn collect_ids(collector: &mut dyn FnMut(Id)) {
        A::collect_ids(collector);
        B::collect_ids(collector);
        C::collect_ids(collector);
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

    fn collect_ids(collector: &mut dyn FnMut(Id)) {
        A::collect_ids(collector);
        B::collect_ids(collector);
        C::collect_ids(collector);
        D::collect_ids(collector);
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

    fn collect_ids(collector: &mut dyn FnMut(Id)) {
        A::collect_ids(collector);
        B::collect_ids(collector);
        C::collect_ids(collector);
        D::collect_ids(collector);
        E::collect_ids(collector);
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

    fn collect_ids(collector: &mut dyn FnMut(Id)) {
        A::collect_ids(collector);
        B::collect_ids(collector);
        C::collect_ids(collector);
        D::collect_ids(collector);
        E::collect_ids(collector);
        F::collect_ids(collector);
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

    fn collect_ids(collector: &mut dyn FnMut(Id)) {
        A::collect_ids(collector);
        B::collect_ids(collector);
        C::collect_ids(collector);
        D::collect_ids(collector);
        E::collect_ids(collector);
        F::collect_ids(collector);
        G::collect_ids(collector);
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

    fn collect_ids(collector: &mut dyn FnMut(Id)) {
        A::collect_ids(collector);
        B::collect_ids(collector);
        C::collect_ids(collector);
        D::collect_ids(collector);
        E::collect_ids(collector);
        F::collect_ids(collector);
        G::collect_ids(collector);
        H::collect_ids(collector);
    }
}