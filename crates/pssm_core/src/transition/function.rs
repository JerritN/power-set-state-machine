use std::{collections::HashSet, ops::{Deref, DerefMut}};

use crate::{Id, State, Truth, transition::{TransitionError, InvalidTransitionError}};

/// A wrapper type for parameters passed to transition functions.
/// 
/// This type is used to distinguish parameters from inputs when calling transition functions.
pub struct Param<P>(pub P);

impl<P> Deref for Param<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P> DerefMut for Param<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub(crate) trait TransitionFunction<In,Param> {
    type Result: TransitionOutput;
    fn call(&self, input: In, params: Param) -> Self::Result;
}

pub(crate) trait TransitionFunctionMut<In,Param> {
    type Result: TransitionOutput;
    fn call(&mut self, input: In, params: Param) -> Self::Result;
}

pub(crate) trait TransitionFunctionOnce<In,Param> {
    type Result: TransitionOutput;
    fn call(self, input: In, params: Param) -> Self::Result;
}

macro_rules! impl_trans_fns {
    (
        // input params
        ($($I:ident),*),
        // parameter params
        ($($P:ident),*)
    ) => {
        impl<$($I,)* $($P,)* Res, Fun> TransitionFunction<($($I,)*),($($P,)*)> for Fun
        where
            $($I: TransitionInput,)*
            Res: TransitionOutput,
            Fun: Fn($($I,)* $(Param<$P>,)*) -> Res
        {
            type Result = Res;

            #[allow(non_snake_case)]
            fn call(&self, input: ($($I,)*), params: ($($P,)*)) -> Self::Result {
                let ($($I,)*) = input;
                let ($($P,)*) = params;
                self($($I,)* $(Param($P),)*)
            }
        }

        impl<$($I,)* $($P,)* Res, Fun> TransitionFunctionMut<($($I,)*),($($P,)*)> for Fun
        where
            $($I: TransitionInput,)*
            Res: TransitionOutput,
            Fun: FnMut($($I,)* $(Param<$P>,)*) -> Res
        {
            type Result = Res;

            #[allow(non_snake_case)]
            fn call(&mut self, input: ($($I,)*), params: ($($P,)*)) -> Self::Result {
                let ($($I,)*) = input;
                let ($($P,)*) = params;
                self($($I,)* $(Param($P),)*)
            }
        }

        impl<$($I,)* $($P,)* Res, Fun> TransitionFunctionOnce<($($I,)*),($($P,)*)> for Fun
        where
            $($I: TransitionInput,)*
            Res: TransitionOutput,
            Fun: FnOnce($($I,)* $(Param<$P>,)*) -> Res
        {
            type Result = Res;

            #[allow(non_snake_case)]
            fn call(self, input: ($($I,)*), params: ($($P,)*)) -> Self::Result {
                let ($($I,)*) = input;
                let ($($P,)*) = params;
                self($($I,)* $(Param($P),)*)
            }
        }
    };
}

impl_trans_fns!((),());
impl_trans_fns!((I1), ());
impl_trans_fns!((I1, I2), ());
impl_trans_fns!((I1, I2, I3), ());
impl_trans_fns!((I1, I2, I3, I4), ());
impl_trans_fns!((), (P1));
impl_trans_fns!((I1), (P1));
impl_trans_fns!((I1, I2), (P1));
impl_trans_fns!((I1, I2, I3), (P1));
impl_trans_fns!((I1, I2, I3, I4), (P1));
impl_trans_fns!((), (P1, P2));
impl_trans_fns!((I1), (P1, P2));
impl_trans_fns!((I1, I2), (P1, P2));
impl_trans_fns!((I1, I2, I3), (P1, P2));
impl_trans_fns!((I1, I2, I3, I4), (P1, P2));

/// A trait that represents a transition input.
/// 
/// A transition input is a piece of data that can be taken from the state when running a transition.
/// 
/// It is implemented for:
/// 
/// - `Truth` types
/// - `Option<Truth>` types
/// - Tuples of up to 8 `TransitionInput` types
pub(crate) trait TransitionInput: Sized {
    fn try_take_from(state: &mut State) -> Result<Self, TransitionError>;

    fn collect_required<C,E>(collector: &mut C) -> Result<(),E>
    where 
        C: FnMut(Id) -> Result<(),E>;

    fn required() -> Result<HashSet<Id>,InvalidTransitionError> {
        let mut ids = HashSet::new();
        Self::collect_required(&mut |id| { 
            if ids.insert(id) {
                Ok(())
            } else {
                Err(InvalidTransitionError::TruthRequiredMultipleTimes(id))
            }
        }).map(|_| ids)
    }
}

impl<T> TransitionInput for T 
where 
    T: Truth + 'static
{
    fn try_take_from(state: &mut State) -> Result<Self, TransitionError> {
        state.remove(&T::id())
            .ok_or_else(|| TransitionError::MissingTruth(T::id()))
            .map(|val| *val.downcast::<T>().expect("Invalid type stored for a truth in the state"))
    }

    fn collect_required<C,E>(collector: &mut C) -> Result<(),E>
    where 
        C: FnMut(Id) -> Result<(),E>
    {
        collector(T::id())
    }
}

impl<T> TransitionInput for Option<T> 
where 
    T: TransitionInput + 'static
{
    fn try_take_from(state: &mut State) -> Result<Self, TransitionError> {
        match T::try_take_from(state) {
            Ok(val) => Ok(Some(val)),
            Err(TransitionError::MissingTruth(_)) => Ok(None)
        }
    }

    fn collect_required<C,E>(_: &mut C) -> Result<(),E>
    where 
        C: FnMut(Id) -> Result<(),E>
    {
        Ok(())
    }
}

macro_rules! impl_trans_in {
    ($($T:ident),*) => {
        impl<$($T,)*> TransitionInput for ($($T,)*) 
        where 
            $($T: TransitionInput,)*
        {
            #[allow(unused)]
            fn try_take_from(state: &mut State) -> Result<Self, TransitionError> {
                Ok(($(<$T>::try_take_from(state)?,)*))
            }

            #[allow(unused)]
            fn collect_required<C,E>(collector: &mut C) -> Result<(),E>
            where 
                C: FnMut(Id) -> Result<(),E>
            {
                $(
                    <$T>::collect_required(collector)?;
                )*
                Ok(())
            }
        }
    }
}

impl_trans_in!();
impl_trans_in!(A1);
impl_trans_in!(A1, A2);
impl_trans_in!(A1, A2, A3);
impl_trans_in!(A1, A2, A3, A4);
impl_trans_in!(A1, A2, A3, A4, A5);
impl_trans_in!(A1, A2, A3, A4, A5, A6);
impl_trans_in!(A1, A2, A3, A4, A5, A6, A7);
impl_trans_in!(A1, A2, A3, A4, A5, A6, A7, A8);

/// A trait that represents a transition result.
/// 
/// A transition result is a piece of data that can be returned from a transition.
/// 
/// It is implemented for:
/// 
/// - `Truth` types
/// - `Option<Truth>` types
/// - Tuples of up to 8 `TransitionOutput` types
pub(crate) trait TransitionOutput {

    fn insert_into(self, state: &mut State);

    fn collect_produces<C,E>(collector: C) -> Result<(),E>
    where 
        C: FnMut(Id) -> Result<(),E>;

    fn produces() -> Result<HashSet<Id>,InvalidTransitionError> {
        let mut ids = HashSet::new();
        Self::collect_produces(|id| {
            if ids.insert(id) {
                Ok(())
            } else {
                Err(InvalidTransitionError::TruthProducedMultipleTimes(id))
            }
        }).map(|_| ids)
    }
}

impl<T: Truth + 'static> TransitionOutput for T {
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

impl<A> TransitionOutput for Option<A>
where 
    A: TransitionOutput
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

macro_rules! impl_trans_out {
    ($($T:ident),*) => {
        impl<$($T,)*> TransitionOutput for ($($T,)*) 
        where 
            $($T: TransitionOutput,)*
        {
            #[allow(non_snake_case, unused)]
            fn insert_into(self, state: &mut State) {
                let ($($T,)*) = self;
                $(
                    $T.insert_into(state);
                )*
            }

            #[allow(unused)]
            fn collect_produces<C,E>(mut collector: C) -> Result<(),E>
            where 
                C: FnMut(Id) -> Result<(),E>
            {
                $(
                    $T::collect_produces(&mut collector)?;
                )*
                Ok(())
            }
        }
    }
}

impl_trans_out!();
impl_trans_out!(A1);
impl_trans_out!(A1, A2);
impl_trans_out!(A1, A2, A3);
impl_trans_out!(A1, A2, A3, A4);
impl_trans_out!(A1, A2, A3, A4, A5);
impl_trans_out!(A1, A2, A3, A4, A5, A6);
impl_trans_out!(A1, A2, A3, A4, A5, A6, A7);
impl_trans_out!(A1, A2, A3, A4, A5, A6, A7, A8);