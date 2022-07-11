//! Illustrate usage of generic types in capability.

#![feature(generic_associated_types)]
#![feature(allocator_api)]
#![allow(non_camel_case_types)]

use rust_context_emulation::prelude_gat::*;
use std::alloc::{Allocator, Global};
use std::marker::PhantomData;

fn main() {
    let alloc = __alloc(Global);
    let mut cx = EmptyStore::new().push(&alloc);

    let mut v = vec_new::default().cx_call((), cx.reborrow());
    // `Vec` remembers its allocator, so calling other (even potentially allocating) methods
    // can be done normally.
    v.push(3_u32);

    assert_eq!(&v, &[3_u32]);

    let alloc: &__alloc<dyn Allocator> = &alloc;
    let mut cx = cx.push(alloc);

    let mut v2 = vec_new::default().cx_call((), cx.reborrow());
    v2.push(32_u32);

    assert_eq!(v, [32_u32]);

    // Doesn't compile, since allocator type is different.
    // v = v2;
}

struct __alloc<A>(A)
where
    A: Allocator + ?Sized;

impl<A> Capability for __alloc<A>
where
    A: Allocator + ?Sized,
{
    type Index = Index<0>;
    type Inner = A;

    fn as_ref(&self) -> &Self::Inner {
        &self.0
    }

    fn as_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

// Note how generic from capability by necessity leaks into function type.
struct vec_new<'alloc, T, A>(PhantomData<T>, PhantomData<&'alloc A>)
where
    A: Allocator + ?Sized;

impl<'alloc, T, A> CxFnOnce<()> for vec_new<'alloc, T, A>
where
    A: Allocator + ?Sized,
{
    type Output<'_0, '_1, '_2, '_3> = Vec<T, &'alloc A>;
    type Context<'_0, '_1, '_2, '_3> = MakeContext<(&'alloc __alloc<A>,)>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl<'alloc, T, A> CxFnMut<()> for vec_new<'alloc, T, A>
where
    A: Allocator + ?Sized,
{
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl<'alloc, T, A> CxFn<()> for vec_new<'alloc, T, A>
where
    A: Allocator + ?Sized,
{
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (): (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let alloc = cx.extract_ref::<__alloc<A>>();

        Vec::new_in(alloc)
    }
}

impl<'alloc, T, A> Default for vec_new<'alloc, T, A>
where
    A: Allocator + ?Sized,
{
    fn default() -> Self {
        vec_new(PhantomData, PhantomData)
    }
}
