//! Implementation of `Handle` which is a wrapper around references.

use crate::ops::*;
use crate::option;
use crate::reference::{Mutable, Shared};

pub struct Handle<T>(T);

pub type HandleRef<'a, T> = Handle<&'a T>;
pub type HandleMut<'a, T> = Handle<&'a mut T>;

impl<T> Handle<T> {
    pub fn new(t: T) -> Self {
        Handle(t)
    }
}

impl<'a, T> HandleRef<'a, T>
where
    T: ?Sized + 'a,
{
    #[allow(clippy::should_implement_trait)]
    pub fn as_ref(&self) -> &'a T {
        self.0
    }
}

impl<'a, T> HandleMut<'a, T>
where
    T: ?Sized + 'a,
{
    #[allow(clippy::should_implement_trait)]
    pub fn as_mut(&mut self) -> &mut T {
        self.0
    }
}

impl<'a, T> Reborrow for HandleRef<'a, T>
where
    T: ?Sized + 'a,
{
    type Output<'s> = Self where Self: 's;

    fn reborrow(&mut self) -> Self::Output<'_> {
        Handle(self.0)
    }
}

impl<'a, T> Reborrow for HandleMut<'a, T>
where
    T: ?Sized + 'a,
{
    type Output<'s> = HandleMut<'s, T> where Self: 's;

    fn reborrow(&mut self) -> Self::Output<'_> {
        Handle(&mut *self.0)
    }
}

impl<'a, T> Coerce<HandleRef<'a, T>> for HandleRef<'a, T> {
    fn coerce(self) -> HandleRef<'a, T> {
        self
    }
}

impl<'a, T> Coerce<HandleRef<'a, T>> for HandleMut<'a, T> {
    fn coerce(self) -> HandleRef<'a, T> {
        Handle(self.0)
    }
}

impl<'a, T> Coerce<HandleMut<'a, T>> for HandleMut<'a, T> {
    fn coerce(self) -> HandleMut<'a, T> {
        self
    }
}

impl<'a, T> UnifyOp<HandleRef<'a, T>> for HandleRef<'a, T> {
    type Output = HandleRef<'a, T>;
}

impl<'a, T> UnifyOp<HandleMut<'a, T>> for HandleRef<'a, T> {
    type Output = HandleMut<'a, T>;
}

impl<'a, T> UnifyOp<HandleRef<'a, T>> for HandleMut<'a, T> {
    type Output = HandleMut<'a, T>;
}

impl<'a, T> UnifyOp<HandleMut<'a, T>> for HandleMut<'a, T> {
    type Output = HandleMut<'a, T>;
}

impl<'a, T> AsPermissionOp for HandleRef<'a, T> {
    type Output = Shared;
}

impl<'a, T> AsPermissionOp for HandleMut<'a, T> {
    type Output = Mutable;
}

impl<'a, T> LimitByOp<()> for HandleRef<'a, T> {
    type Output = option::None;
}

impl<'a, T> LimitByOp<Shared> for HandleRef<'a, T> {
    type Output = option::Some<Self>;
}

impl<'a, T> LimitByOp<Mutable> for HandleRef<'a, T> {
    type Output = option::Some<Self>;
}

impl<'a, T> LimitByOp<()> for HandleMut<'a, T> {
    type Output = option::None;
}

impl<'a, T> LimitByOp<Shared> for HandleMut<'a, T> {
    type Output = option::Some<HandleRef<'a, T>>;
}

impl<'a, T> LimitByOp<Mutable> for HandleMut<'a, T> {
    type Output = option::Some<Self>;
}
