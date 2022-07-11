//! Illustrate usage of wildcard contexts in generic context using `Copy` instead of reborrow.
//!
//! In this case we abandon the idea of supporting mutable references
//! which renders all contexts `Copy`able.
//! This allows us to go away with reborrowing cruft and simplify bounds.

#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]
#![allow(clippy::clone_on_copy)]

use rust_context_emulation::prelude_gat::*;
use rust_context_emulation::{
    context::{handle::HandleRef, store::Store},
    ops::Unified,
    option::{None, Some},
    tuple4::Tuple4,
};
use std::marker::PhantomData;

fn main() {
    let hidden_str = __hidden_str("treasure".to_string());
    let data_store = __data_store(vec![3]);
    let counter = __counter(5);
    let cx = EmptyStore::new()
        .push(&hidden_str)
        .push(&data_store)
        .push(&counter);

    // This is not intended to work.
    // let _ = generic_fn::<mutable_ref>(PhantomData);

    let value = {
        let cx = cx.coerce();
        // Compiler gets confused about something here, so we need turbofish to help him out.
        generic_fn::<shared_ref>::default().cx_call((shared_ref,), cx)
    };

    assert_eq!(value, 13);
}

type UnifiedContext<'_0, '_1, '_2, '_3, F> = Unify<(
    <F as CxFnOnce<()>>::Context<'_0, '_1, '_2, '_3>,
    MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __counter>,)>,
)>;

// `F` is a type and must be early bound.
// Also notice that any bound on `F` also go into fn type.
struct generic_fn<F>(PhantomData<fn(F)>)
where
    // This is the only meaningful bound here.
    // The rest is related to internal machinery.
    F: for<'_0, '_1, '_2, '_3> CxFn<(), Output<'_0, '_1, '_2, '_3> = usize>,

    // Welcome to hell.
    // We know the following type manipulation should be valid by convention,
    // but we need to convince compiler of that.
    // There are three parts to take care of: unification, reborrowing and coercion.

    // This is the unification bound:
    for<'_0, '_1, '_2, '_3> (
        <F as CxFnOnce<()>>::Context<'_0, '_1, '_2, '_3>,
        Store<Tuple4<None, None, Some<HandleRef<'_2, __counter>>, None>>,
    ): Unified,

    // This bound is fragile and spills out a lot of guts.
    // Ideally, we would like to write something like this:
    //
    // for<'_0, '_1, '_2, '_3> (
    //     <F as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context,
    //     MakeContext<(Select<'_0, '_1, '_2, '_3, Mutable, __counter>,)>,
    // ): Unified,
    //
    // However, compiler for some reason doesn't expand definition of `MakeContext` in this context,
    // so it fails.
    // I still don't understand why it doesn't digest type shorthands in some places.

    // This is "reborrowing" bound.
    // Because we don't expect mutable references every context is trivially `Copy`able.
    for<'_0, '_1, '_2, '_3> UnifiedContext<'_0, '_1, '_2, '_3, F>: Copy,

    // And this is two coercion bounds, one for each target:
    for<'_0, '_1, '_2, '_3> UnifiedContext<'_0, '_1, '_2, '_3, F>:
        Coerce<<F as CxFnOnce<()>>::Context<'_0, '_1, '_2, '_3>>,
    for<'_0, '_1, '_2, '_3> UnifiedContext<'_0, '_1, '_2, '_3, F>:
        Coerce<MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __counter>,)>>;

// Now, we just need to replicate the insanity on every impl block...
impl<F> CxFnOnce<(F,)> for generic_fn<F>
where
    F: for<'_a0, '_a1, '_a2, '_a3> CxFn<(), Output<'_a0, '_a1, '_a2, '_a3> = usize>,
    for<'_a0, '_a1, '_a2, '_a3> (
        <F as CxFnOnce<()>>::Context<'_a0, '_a1, '_a2, '_a3>,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>,
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3, 'local> UnifiedContext<'_a0, '_a1, '_a2, '_a3, F>: Copy
        + Coerce<<F as CxFnOnce<()>>::Context<'_a0, '_a1, '_a2, '_a3>>
        + Coerce<MakeContext<(Select<'_a0, '_a1, '_a2, '_a3, Shared, __counter>,)>>,
{
    type Output<'_0, '_1, '_2, '_3> = usize;
    type Context<'_0, '_1, '_2, '_3> = UnifiedContext<'_0, '_1, '_2, '_3, F>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (F,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl<F> CxFnMut<(F,)> for generic_fn<F>
where
    F: for<'_a0, '_a1, '_a2, '_a3> CxFn<(), Output<'_a0, '_a1, '_a2, '_a3> = usize>,
    for<'_a0, '_a1, '_a2, '_a3> (
        <F as CxFnOnce<()>>::Context<'_a0, '_a1, '_a2, '_a3>,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>,
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3, 'local> UnifiedContext<'_a0, '_a1, '_a2, '_a3, F>: Copy
        + Coerce<<F as CxFnOnce<()>>::Context<'_a0, '_a1, '_a2, '_a3>>
        + Coerce<MakeContext<(Select<'_a0, '_a1, '_a2, '_a3, Shared, __counter>,)>>,
{
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (F,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl<F> CxFn<(F,)> for generic_fn<F>
where
    F: for<'_a0, '_a1, '_a2, '_a3> CxFn<(), Output<'_a0, '_a1, '_a2, '_a3> = usize>,
    for<'_a0, '_a1, '_a2, '_a3> (
        <F as CxFnOnce<()>>::Context<'_a0, '_a1, '_a2, '_a3>,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>,
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3, 'local> UnifiedContext<'_a0, '_a1, '_a2, '_a3, F>: Copy
        + Coerce<<F as CxFnOnce<()>>::Context<'_a0, '_a1, '_a2, '_a3>>
        + Coerce<MakeContext<(Select<'_a0, '_a1, '_a2, '_a3, Shared, __counter>,)>>,
{
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (f,): (F,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let value = {
            let cx = cx.clone().coerce();
            f.cx_call((), cx)
        };

        let local_cx: MakeContext<(&__counter,)> = cx.clone().coerce();
        let counter = local_cx.extract_ref::<__counter>();

        *counter + value
    }
}

impl<F> Default for generic_fn<F>
where
    F: for<'_a0, '_a1, '_a2, '_a3> CxFn<(), Output<'_a0, '_a1, '_a2, '_a3> = usize>,
    for<'_a0, '_a1, '_a2, '_a3> (
        <F as CxFnOnce<()>>::Context<'_a0, '_a1, '_a2, '_a3>,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>,
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3, 'local> UnifiedContext<'_a0, '_a1, '_a2, '_a3, F>: Copy
        + Coerce<<F as CxFnOnce<()>>::Context<'_a0, '_a1, '_a2, '_a3>>
        + Coerce<MakeContext<(Select<'_a0, '_a1, '_a2, '_a3, Shared, __counter>,)>>,
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

impl CxFnOnce<()> for mutable_ref {
    type Output<'_0, '_1, '_2, '_3> = usize;
    type Context<'_0, '_1, '_2, '_3> =
        MakeContext<(Select<'_0, '_1, '_2, '_3, Mutable, __data_store>,)>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl CxFnMut<()> for mutable_ref {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl CxFn<()> for mutable_ref {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (): (),
        mut cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let data_store = cx.extract_mut::<__data_store>();
        data_store.push(32);

        data_store.iter().copied().sum()
    }
}

#[derive(Default)]
struct shared_ref;

impl CxFnOnce<()> for shared_ref {
    type Output<'_0, '_1, '_2, '_3> = usize;
    type Context<'_0, '_1, '_2, '_3> =
        MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __hidden_str>,)>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl CxFnMut<()> for shared_ref {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl CxFn<()> for shared_ref {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (): (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let hidden_str = cx.extract_ref::<__hidden_str>();

        hidden_str.len()
    }
}
