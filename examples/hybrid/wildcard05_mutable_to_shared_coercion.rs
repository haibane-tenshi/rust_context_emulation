//! Illustrate usage of wildcard contexts in generic context using `Reborrow2` trait.

#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]

use rust_context_emulation::ops::{Reborrow2, Reborrow2Lifetime, Unified};
use rust_context_emulation::prelude_hybrid::*;
use std::marker::PhantomData;

fn main() {
    let mut counter = __counter(0);
    let mut cx = EmptyStore::new().push(&mut counter);

    {
        let cx = cx.reborrow().coerce();
        call_two::<shared_ref, shared_ref>::default()
            .cx_call((shared_ref::default(), shared_ref::default()), cx);
    }
    {
        let cx = cx.reborrow().coerce();
        call_two::<mutable_ref, mutable_ref>::default()
            .cx_call((mutable_ref::default(), mutable_ref::default()), cx);
    }

    // But both of those fail:
    // let _ = generic_fn::<mutable_ref, shared_ref>(PhantomData);
    // let _ = generic_fn::<shared_ref, mutable_ref>(PhantomData);
    // Unfortunately, we cannot spell proper coercion/unification bounds in this case still.

    assert_eq!(counter.0, 2);
}

type UnifiedContext<'_0, '_1, '_2, '_3, 'm, F, G> = Unify<(
    <F as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context<'m>,
    <G as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context<'m>,
)>;

// `F` is a type and must be early bound.
// Also notice that any bound on `F` also go into fn type.
struct call_two<F, G>(PhantomData<fn(F, G)>)
where
    F: for<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, (), Output = ()>,
    G: for<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, (), Output = ()>,

    // Welcome to hell.
    // We know the following type manipulation should be valid by convention,
    // but we need to convince compiler of that.
    // There are three parts to take care of: unification, reborrowing and coercion.

    // This is the unification bound:
    for<'_0, '_1, '_2, '_3, 'm> (
        <F as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context<'m>,
        <G as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context<'m>,
    ): Unified,

    // Roborrowing bound:
    for<'_0, '_1, '_2, '_3, 'm> <(
        <F as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context<'m>,
        <G as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context<'m>,
    ) as Unified>::Output: Reborrow2,

    // And this is two coercion bounds, one for each target:
    for<'_0, '_1, '_2, '_3, 'm, 'local> <UnifiedContext<'_0, '_1, '_2, '_3, 'm, F, G> as Reborrow2Lifetime<'local>>::Output:
        Coerce<<F as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context<'local>>,
    for<'_0, '_1, '_2, '_3, 'm, 'local> <UnifiedContext<'_0, '_1, '_2, '_3, 'm, F, G> as Reborrow2Lifetime<'local>>::Output:
        Coerce<<G as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context<'local>>;

// Now, we just need to replicate the insanity on every impl block...
impl<'_0, '_1, '_2, '_3, F, G> CxFnOnce<'_0, '_1, '_2, '_3, (F, G)> for call_two<F, G>
where
    F: for<'_a0, '_a1, '_a2, '_a3> CxFn<'_a0, '_a1, '_a2, '_a3, (), Output = ()>,
    G: for<'_a0, '_a1, '_a2, '_a3> CxFn<'_a0, '_a1, '_a2, '_a3, (), Output = ()>,
    for<'_a0, '_a1, '_a2, '_a3, 'am> (
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
        <G as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3, 'am> <(
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
        <G as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
    ) as Unified>::Output: Reborrow2,
    for<'_a0, '_a1, '_a2, '_a3, 'am, 'local> <UnifiedContext<'_a0, '_a1, '_a2, '_a3, 'am, F, G> as Reborrow2Lifetime<'local>>::Output:
        Coerce<<F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'local>>,
    for<'_a0, '_a1, '_a2, '_a3, 'am, 'local> <UnifiedContext<'_a0, '_a1, '_a2, '_a3, 'am, F, G> as Reborrow2Lifetime<'local>>::Output:
        Coerce<<G as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'local>>,
{
    type Output = ();
    type Context<'m> = UnifiedContext<'_0, '_1, '_2, '_3, 'm, F, G>;

    fn cx_call_once(mut self, args: (F, G), cx: Self::Context<'_>) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3, F, G> CxFnMut<'_0, '_1, '_2, '_3, (F, G)> for call_two<F, G>
where
    F: for<'_a0, '_a1, '_a2, '_a3> CxFn<'_a0, '_a1, '_a2, '_a3, (), Output = ()>,
    G: for<'_a0, '_a1, '_a2, '_a3> CxFn<'_a0, '_a1, '_a2, '_a3, (), Output = ()>,
    for<'_a0, '_a1, '_a2, '_a3, 'am> (
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
        <G as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3, 'am> <(
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
        <G as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
    ) as Unified>::Output: Reborrow2,
    for<'_a0, '_a1, '_a2, '_a3, 'am, 'local> <UnifiedContext<'_a0, '_a1, '_a2, '_a3, 'am, F, G> as Reborrow2Lifetime<'local>>::Output:
        Coerce<<F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'local>>,
    for<'_a0, '_a1, '_a2, '_a3, 'am, 'local> <UnifiedContext<'_a0, '_a1, '_a2, '_a3, 'am, F, G> as Reborrow2Lifetime<'local>>::Output:
        Coerce<<G as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'local>>,
{
    fn cx_call_mut(&mut self, args: (F, G), cx: Self::Context<'_>) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3, F, G> CxFn<'_0, '_1, '_2, '_3, (F, G)> for call_two<F, G>
where
    F: for<'_a0, '_a1, '_a2, '_a3> CxFn<'_a0, '_a1, '_a2, '_a3, (), Output = ()>,
    G: for<'_a0, '_a1, '_a2, '_a3> CxFn<'_a0, '_a1, '_a2, '_a3, (), Output = ()>,
    for<'_a0, '_a1, '_a2, '_a3, 'am> (
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
        <G as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3, 'am> <(
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
        <G as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
    ) as Unified>::Output: Reborrow2,
    for<'_a0, '_a1, '_a2, '_a3, 'am, 'local> <UnifiedContext<'_a0, '_a1, '_a2, '_a3, 'am, F, G> as Reborrow2Lifetime<'local>>::Output:
        Coerce<<F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'local>>,
    for<'_a0, '_a1, '_a2, '_a3, 'am, 'local> <UnifiedContext<'_a0, '_a1, '_a2, '_a3, 'am, F, G> as Reborrow2Lifetime<'local>>::Output:
        Coerce<<G as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'local>>,
{
    fn cx_call(&self, (f, g): (F, G), mut cx: Self::Context<'_>) -> Self::Output {
        {
            let cx = cx.reborrow().coerce();
            f.cx_call((), cx)
        }

        {
            let cx = cx.reborrow().coerce();
            g.cx_call((), cx)
        }
    }
}

impl<F, G> Default for call_two<F, G>
where
    F: for<'_a0, '_a1, '_a2, '_a3> CxFn<'_a0, '_a1, '_a2, '_a3, (), Output = ()>,
    G: for<'_a0, '_a1, '_a2, '_a3> CxFn<'_a0, '_a1, '_a2, '_a3, (), Output = ()>,
    for<'_a0, '_a1, '_a2, '_a3, 'am> (
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
        <G as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3, 'am> <(
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
        <G as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
    ) as Unified>::Output: Reborrow2,
    for<'_a0, '_a1, '_a2, '_a3, 'am, 'local> <UnifiedContext<'_a0, '_a1, '_a2, '_a3, 'am, F, G> as Reborrow2Lifetime<'local>>::Output:
        Coerce<<F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'local>>,
    for<'_a0, '_a1, '_a2, '_a3, 'am, 'local> <UnifiedContext<'_a0, '_a1, '_a2, '_a3, 'am, F, G> as Reborrow2Lifetime<'local>>::Output:
        Coerce<<G as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'local>>,
{
    fn default() -> Self {
        call_two::<F, G>(PhantomData)
    }
}

struct __counter(usize);

impl Capability for __counter {
    type Index = Index<0>;
    type Inner = usize;

    fn as_ref(&self) -> &Self::Inner {
        &self.0
    }

    fn as_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

#[derive(Default)]
struct mutable_ref;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, ()> for mutable_ref {
    type Output = ();
    type Context<'m> = MakeContext<(&'m mut __counter,)>;

    fn cx_call_once(mut self, args: (), cx: Self::Context<'_>) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, ()> for mutable_ref {
    fn cx_call_mut(&mut self, args: (), cx: Self::Context<'_>) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, ()> for mutable_ref {
    fn cx_call(&self, (): (), mut cx: Self::Context<'_>) -> Self::Output {
        let counter = cx.extract_mut::<__counter>();
        *counter += 1;
    }
}

#[derive(Default)]
struct shared_ref;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, ()> for shared_ref {
    type Output = ();
    type Context<'m> = MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __counter>,)>;

    fn cx_call_once(mut self, args: (), cx: Self::Context<'_>) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, ()> for shared_ref {
    fn cx_call_mut(&mut self, args: (), cx: Self::Context<'_>) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, ()> for shared_ref {
    fn cx_call(&self, (): (), cx: Self::Context<'_>) -> Self::Output {
        let _counter = cx.extract_ref::<__counter>();
    }
}
