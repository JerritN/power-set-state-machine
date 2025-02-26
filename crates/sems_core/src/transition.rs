use std::{any::Any, marker::PhantomData};

use crate::{truth::{Deconstructable, Requestable}, Id};

pub struct Transition<Marker>
{
    run: Box<dyn Fn(Vec<Box<dyn Any>>) -> Vec<(Id,Box<dyn Any>)>>,
    _phantom: PhantomData<Marker>
}

impl<Marker> Transition<Marker>
{
    pub fn new<F>(run: F) -> Self
    where
        F: Fn(Vec<Box<dyn Any>>) -> Vec<(Id,Box<dyn Any>)> + 'static
    {
        Transition {
            run: Box::new(run),
            _phantom: PhantomData
        }
    }

    pub fn run(&self, args: Vec<Box<dyn Any>>) -> Vec<(Id,Box<dyn Any>)>
    {
        (self.run)(args)
    }
}

pub trait IntoTransition<Marker>
{
    fn into_transition(self) -> Transition<Marker>;
}

impl<A,R,F> IntoTransition<(A,R)> for F
where 
    A: Requestable + 'static,
    R: Deconstructable,
    F: Fn(A) -> R + 'static
{
    fn into_transition(self) -> Transition<(A,R)> {
        Transition::new(move |args| {
            let mut iter = args.into_iter();
            let a = A::take_from(&mut iter);
            let res = self(a);
            res.deconstruct()
        })
    }
}