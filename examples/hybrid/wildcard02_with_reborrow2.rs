//! Illustrate usage of wildcard contexts in generic context using `Reborrow2` trait.

#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]

use rust_context_emulation::prelude_hybrid::*;
use rust_context_emulation::{
    context::{handle::HandleRef, store::Store},
    ops::{Reborrow2, Reborrow2Lifetime, Unified},
    option::{None, Some},
    tuple4::Tuple4,
};
use std::marker::PhantomData;

fn main() {
    let hidden_str = __hidden_str("treasure".to_string());
    let mut data_store = __data_store(vec![3]);
    let counter = __counter(5);
    let mut cx = EmptyStore::new()
        .push(&hidden_str)
        .push(&mut data_store)
        .push(&counter);

    let value = {
        let cx = cx.reborrow().coerce();
        // Compiler gets confused about something here, so we need turbofish to help him out.
        generic_fn::<shared_ref>::default().cx_call((shared_ref,), cx)
    };

    assert_eq!(value, 13);

    let value = {
        let cx = cx.coerce();
        generic_fn::<mutable_ref>::default().cx_call((mutable_ref,), cx)
    };

    assert_eq!(value, 40);
    assert_eq!(data_store.as_ref(), &[3, 32]);
}

type UnifiedContext<'_0, '_1, '_2, '_3, 'm, F> = Unify<(
    <F as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context<'m>,
    MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __counter>,)>,
)>;

// `F` is a type and must be early bound.
// Also notice that any bound on `F` also go into fn type.
struct generic_fn<F>(PhantomData<fn(F)>)
where
    // This is the only meaningful bound here.
    // The rest is related to internal machinery.
    F: for<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, (), Output = usize>,

    // Welcome to hell.
    // We know the following type manipulation should be valid by convention,
    // but we need to convince compiler of that.
    // There are three parts to take care of: unification, reborrowing and coercion.

    // This is the unification bound:
    for<'_0, '_1, '_2, '_3, 'm> (
        <F as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context<'m>,
        Store<Tuple4<None, None, Some<HandleRef<'_2, __counter>>, None>>,
    ): Unified,

    // This bound is fragile and spills out a lot of guts.
    // Ideally, we would like to write something like this:
    //
    // for<'_0, '_1, '_2, '_3, 'm> (
    //     <F as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context<'m>,
    //     MakeContext<(Select<'_0, '_1, '_2, '_3, Mutable, __counter>,)>,
    // ): Unified,
    //
    // However, compiler for some reason doesn't expand definition of `MakeContext` in this context,
    // so it fails.
    // I still don't understand why it doesn't digest type shorthands in some places.

    // This is reborrowing bound:
    // for<'_0, '_1, '_2, '_3, 'm> UnifiedContext<'_0, '_1, '_2, '_3, 'm, F>: Reborrow2,
    //
    // Unfortunately, we need to expand it as well.
    for<'_0, '_1, '_2, '_3, 'm> <(
        <F as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context<'m>,
        Store<Tuple4<None, None, Some<HandleRef<'_2, __counter>>, None>>,
    ) as Unified>::Output: Reborrow2,

    // And this is two coercion bounds, one for each target:
    for<'_0, '_1, '_2, '_3, 'm, 'local> <UnifiedContext<'_0, '_1, '_2, '_3, 'm, F> as Reborrow2Lifetime<'local>>::Output:
        Coerce<<F as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context<'local>>,
    for<'_0, '_1, '_2, '_3, 'm, 'local> <UnifiedContext<'_0, '_1, '_2, '_3, 'm, F> as Reborrow2Lifetime<'local>>::Output:
        Coerce<MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __counter>,)>>;

// Now, we just need to replicate the insanity on every impl block...
impl<'_0, '_1, '_2, '_3, F> CxFnOnce<'_0, '_1, '_2, '_3, (F,)> for generic_fn<F>
where
    F: for<'_a0, '_a1, '_a2, '_a3> CxFn<'_a0, '_a1, '_a2, '_a3, (), Output = usize>,
    for<'_a0, '_a1, '_a2, '_a3, 'am> (
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>,
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3, 'am> <(
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>,
    ) as Unified>::Output: Reborrow2,
    for<'_a0, '_a1, '_a2, '_a3, 'am, 'local> <UnifiedContext<'_a0, '_a1, '_a2, '_a3, 'am, F> as Reborrow2Lifetime<'local>>::Output:
        Coerce<<F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'local>>,
    for<'_a0, '_a1, '_a2, '_a3, 'am, 'local> <UnifiedContext<'_a0, '_a1, '_a2, '_a3, 'am, F> as Reborrow2Lifetime<'local>>::Output:
        Coerce<MakeContext<(Select<'_a0, '_a1, '_a2, '_a3, Shared, __counter>,)>>,
{
    type Output = usize;
    type Context<'m> = UnifiedContext<'_0, '_1, '_2, '_3, 'm, F>;

    fn cx_call_once(mut self, args: (F,), cx: Self::Context<'_>) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3, F> CxFnMut<'_0, '_1, '_2, '_3, (F,)> for generic_fn<F>
where
    F: for<'_a0, '_a1, '_a2, '_a3> CxFn<'_a0, '_a1, '_a2, '_a3, (), Output = usize>,
    for<'_a0, '_a1, '_a2, '_a3, 'am> (
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>,
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3, 'am> <(
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>,
    ) as Unified>::Output: Reborrow2,
    for<'_a0, '_a1, '_a2, '_a3, 'am, 'local> <UnifiedContext<'_a0, '_a1, '_a2, '_a3, 'am, F> as Reborrow2Lifetime<'local>>::Output:
        Coerce<<F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'local>>,
    for<'_a0, '_a1, '_a2, '_a3, 'am, 'local> <UnifiedContext<'_a0, '_a1, '_a2, '_a3, 'am, F> as Reborrow2Lifetime<'local>>::Output:
        Coerce<MakeContext<(Select<'_a0, '_a1, '_a2, '_a3, Shared, __counter>,)>>,
{
    fn cx_call_mut(&mut self, args: (F,), cx: Self::Context<'_>) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3, F> CxFn<'_0, '_1, '_2, '_3, (F,)> for generic_fn<F>
where
    F: for<'_a0, '_a1, '_a2, '_a3> CxFn<'_a0, '_a1, '_a2, '_a3, (), Output = usize>,
    for<'_a0, '_a1, '_a2, '_a3, 'am> (
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>,
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3, 'am> <(
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>,
    ) as Unified>::Output: Reborrow2,
    for<'_a0, '_a1, '_a2, '_a3, 'am, 'local> <UnifiedContext<'_a0, '_a1, '_a2, '_a3, 'am, F> as Reborrow2Lifetime<'local>>::Output:
        Coerce<<F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'local>>,
    for<'_a0, '_a1, '_a2, '_a3, 'am, 'local> <UnifiedContext<'_a0, '_a1, '_a2, '_a3, 'am, F> as Reborrow2Lifetime<'local>>::Output:
        Coerce<MakeContext<(Select<'_a0, '_a1, '_a2, '_a3, Shared, __counter>,)>>,
{
    fn cx_call(&self, (f,): (F,), mut cx: Self::Context<'_>) -> Self::Output {
        let value = {
            let cx = cx.reborrow().coerce();
            f.cx_call((), cx)
        };

        let local_cx: MakeContext<(&__counter,)> = cx.reborrow().coerce();
        let counter = local_cx.extract_ref::<__counter>();

        *counter + value
    }
}

impl<F> Default for generic_fn<F>
where
    F: for<'_a0, '_a1, '_a2, '_a3> CxFn<'_a0, '_a1, '_a2, '_a3, (), Output = usize>,
    for<'_a0, '_a1, '_a2, '_a3, 'am> (
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>,
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3, 'am> <(
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'am>,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>,
    ) as Unified>::Output: Reborrow2,
    for<'_a0, '_a1, '_a2, '_a3, 'am, 'local> <UnifiedContext<'_a0, '_a1, '_a2, '_a3, 'am, F> as Reborrow2Lifetime<'local>>::Output:
        Coerce<<F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context<'local>>,
    for<'_a0, '_a1, '_a2, '_a3, 'am, 'local> <UnifiedContext<'_a0, '_a1, '_a2, '_a3, 'am, F> as Reborrow2Lifetime<'local>>::Output:
        Coerce<MakeContext<(Select<'_a0, '_a1, '_a2, '_a3, Shared, __counter>,)>>,
{
    fn default() -> Self {
        generic_fn::<F>(PhantomData)
    }
}

struct __hidden_str(String);

impl Capability for __hidden_str {
    type Index = Index<0>;
    type Inner = String;

    fn as_ref(&self) -> &Self::Inner {
        &self.0
    }

    fn as_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

struct __data_store(Vec<usize>);

impl Capability for __data_store {
    type Index = Index<1>;
    type Inner = Vec<usize>;

    fn as_ref(&self) -> &Self::Inner {
        &self.0
    }

    fn as_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

struct __counter(usize);

impl Capability for __counter {
    type Index = Index<2>;
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
    type Output = usize;
    type Context<'m> = MakeContext<(&'m mut __data_store,)>;

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
        let data_store = cx.extract_mut::<__data_store>();
        data_store.push(32);

        data_store.iter().copied().sum()
    }
}

#[derive(Default)]
struct shared_ref;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, ()> for shared_ref {
    type Output = usize;
    type Context<'m> = MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __hidden_str>,)>;

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
        let hidden_str = cx.extract_ref::<__hidden_str>();

        hidden_str.len()
    }
}
