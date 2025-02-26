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

impl TransitionParam for () {
    fn take_from(_: &mut State) -> Self {
        ()
    }

    fn collect_ids(_: &mut dyn FnMut(Id)) {}
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