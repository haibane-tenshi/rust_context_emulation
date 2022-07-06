//! Type-level `Option`.
//!
//! It implements most context-related operations to make integration smoother.

use crate::ops::*;

#[derive(Copy, Clone)]
pub struct None;

#[derive(Copy, Clone)]
pub struct Some<T>(pub T);

pub trait OrF<Other> {
    type Output;
}

pub type Or<A, B> = <A as OrF<B>>::Output;

impl OrF<None> for None {
    type Output = None;
}

impl<T> OrF<Some<T>> for None {
    type Output = Some<T>;
}

impl<T> OrF<None> for Some<T> {
    type Output = Self;
}

impl<T, U> OrF<Some<U>> for Some<T> {
    type Output = Self;
}

pub trait Mapper<T> {
    type Apply;
}

pub trait MapOp<M> {
    type Output;
}

impl<M> MapOp<M> for None {
    type Output = None;
}

impl<M, T> MapOp<M> for Some<T>
where
    M: Mapper<T>,
{
    type Output = Some<<M as Mapper<T>>::Apply>;
}

pub type Map<Op, M> = <Op as MapOp<M>>::Output;

impl Reborrow for None {
    type Output<'s> = None;

    fn reborrow(&mut self) -> Self::Output<'_> {
        None
    }
}

impl<T> Reborrow for Some<T>
where
    T: Reborrow,
{
    type Output<'s> = Some<<T as Reborrow>::Output<'s>> where Self: 's;

    fn reborrow(&mut self) -> Self::Output<'_> {
        Some(self.0.reborrow())
    }
}

impl UnifyOp<None> for None {
    type Output = Self;
}

impl<T> UnifyOp<Some<T>> for None {
    type Output = Some<T>;
}

impl<T> UnifyOp<None> for Some<T> {
    type Output = Self;
}

impl<T, U> UnifyOp<Some<U>> for Some<T>
where
    T: UnifyOp<U>,
{
    type Output = Some<Unify<(T, U)>>;
}

impl Coerce<None> for None {
    fn coerce(self) -> None {
        None
    }
}

impl<T> Coerce<None> for Some<T> {
    fn coerce(self) -> None {
        None
    }
}

impl<T, U> Coerce<Some<U>> for Some<T>
where
    T: Coerce<U>,
{
    fn coerce(self) -> Some<U> {
        Some(self.0.coerce())
    }
}

impl LimitByOp<None> for None {
    type Output = Self;
}

impl<T> LimitByOp<Some<T>> for None {
    type Output = Self;
}

impl<T> LimitByOp<None> for Some<T> {
    type Output = Self;
}

impl<T, U> LimitByOp<Some<U>> for Some<T>
where
    T: LimitByOp<U>,
{
    type Output = LimitBy<T, U>;
}

impl AsPermissionOp for None {
    type Output = None;
}

impl<T> AsPermissionOp for Some<T>
where
    T: AsPermissionOp,
{
    type Output = Some<<T as AsPermissionOp>::Output>;
}

impl InverseOp for None {
    type Output = None;
}

impl<T> InverseOp for Some<T>
where
    T: InverseOp,
{
    type Output = Some<<T as InverseOp>::Output>;
}
