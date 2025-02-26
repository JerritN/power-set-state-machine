use std::any::Any;

use crate::Id;

pub trait Truth {
    fn id() -> Id;
}

pub trait Deconstructable {
    fn collect_deconstructed(self, collector: &mut dyn FnMut(Id, Box<dyn Any>));
    fn deconstruct(self) -> Vec<(Id, Box<dyn Any>)> where Self: Sized {
        let mut collector = Vec::new();
        Self::collect_deconstructed(self, &mut |id, val| collector.push((id, val)));
        collector
    }
}

pub trait Requestable {
    fn take_from<I>(iter: &mut I) -> Self
    where
        I: Iterator<Item = Box<dyn Any>>;

    fn collect_ids(collector: &mut dyn FnMut(Id));

    fn ids() -> Vec<Id> {
        let mut ids = Vec::new();
        Self::collect_ids(&mut |id| ids.push(id));
        ids
    }
}

impl<T: Truth + 'static> Deconstructable for T {
    fn collect_deconstructed(self, collector: &mut dyn FnMut(Id, Box<dyn Any>)) {
        collector(T::id(), Box::new(self));
    }
}

impl<T: Truth + 'static> Requestable for T {
    fn take_from<I>(iter: &mut I) -> Self
    where
        I: Iterator<Item = Box<dyn Any>>,
    {
        let val = iter.next().expect("Expected a value in iter");
        *val.downcast::<T>().expect("Next value in iter should be of type T")
    }
    
    fn collect_ids(collector: &mut dyn FnMut(Id)) {
        collector(T::id());
    }
}

impl Deconstructable for () {
    fn collect_deconstructed(self, _: &mut dyn FnMut(Id, Box<dyn Any>)) {}
}


impl Requestable for () {
    fn take_from<I>(_: &mut I) -> Self {()}

    fn collect_ids(_: &mut dyn FnMut(Id)) {}
}

impl<A, B> Deconstructable for (A, B) 
where 
    A: Deconstructable,
    B: Deconstructable
{
    fn collect_deconstructed(self, collector: &mut dyn FnMut(Id, Box<dyn Any>)) {
        self.0.collect_deconstructed(collector);
        self.1.collect_deconstructed(collector);
    }
}

impl<A, B> Requestable for (A, B) 
where 
    A: Requestable,
    B: Requestable
{
    fn take_from<I>(iter: &mut I) -> Self
    where
        I: Iterator<Item = Box<dyn Any>>,
    {
        (A::take_from(iter), B::take_from(iter))
    }

    fn collect_ids(collector: &mut dyn FnMut(Id)) {
        A::collect_ids(collector);
        B::collect_ids(collector);
    }
}

impl<A, B, C> Deconstructable for (A, B, C) 
where 
    A: Deconstructable,
    B: Deconstructable,
    C: Deconstructable
{
    fn collect_deconstructed(self, collector: &mut dyn FnMut(Id, Box<dyn Any>)) {
        self.0.collect_deconstructed(collector);
        self.1.collect_deconstructed(collector);
        self.2.collect_deconstructed(collector);
    }
}

impl<A, B, C> Requestable for (A, B, C) 
where 
    A: Requestable,
    B: Requestable,
    C: Requestable
{
    fn take_from<I>(iter: &mut I) -> Self
    where
        I: Iterator<Item = Box<dyn Any>>,
    {
        (A::take_from(iter), B::take_from(iter), C::take_from(iter))
    }

    fn collect_ids(collector: &mut dyn FnMut(Id)) {
        A::collect_ids(collector);
        B::collect_ids(collector);
        C::collect_ids(collector);
    }
}