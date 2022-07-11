//! Illustrate 2 different approaches to desugaring early-bound lifetimes.
//!
//! This is about the only place where there is semantic difference between flavors.
//! I'm still not sure which one is correct.

#![feature(generic_associated_types)]
#![feature(allocator_api)]
#![allow(non_camel_case_types)]
#![allow(clippy::clone_on_copy)]

use rust_context_emulation::prelude_input::*;
use rust_context_emulation::{
    context::handle::HandleRef,
    context::store::Store,
    ops::Unified,
    option::{None, Some},
    tuple4::Tuple4,
};
use std::alloc::{Allocator, Global};
use std::marker::PhantomData;

fn main() {
    let alloc: &__alloc = &__alloc_helper(Global);
    let my_vec = __my_vec(Vec::new_in(&Global));
    let cx = EmptyStore::new().push(alloc).push(&my_vec);

    // Both of those fail, which is expected.
    // The only difference is error message.

    // First flavor complains that it cannot properly mold context.
    // let _ = generic_fn_1::<early_flavor_1>(PhantomData);

    // Second flavor complains that function doesn't have correct shape.
    // let _ = generic_fn_1::<early_flavor_2>(PhantomData);

    // Both of those succeed, also as expected.
    generic_fn_2::<early_flavor_1>(PhantomData).cx_call((early_flavor_1::default(),), cx.clone());
    generic_fn_2::<early_flavor_2>(PhantomData).cx_call((early_flavor_2::default(),), cx.clone());
}

type LocalContext<'_0, '_1, '_2, '_3> = MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __alloc>,)>;

type UnifiedContext<'_0, '_1, '_2, '_3, F> = Unify<(
    <F as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context,
    LocalContext<'_0, '_1, '_2, '_3>,
)>;

struct generic_fn_1<F>(PhantomData<fn(F)>)
where
    for<'_0, '_1, '_2, '_3> F: CxFn<'_0, '_1, '_2, '_3, (), Output = ()>,
    for<'_a0, '_a1, '_a2, '_a3> (
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context,
        Store<Tuple4<Some<HandleRef<'_a0, __alloc>>, None, None, None>>,
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3> UnifiedContext<'_a0, '_a1, '_a2, '_a3, F>: Copy
        + Coerce<LocalContext<'_a0, '_a1, '_a2, '_a3>>
        + Coerce<<F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context>;

impl<'_0, '_1, '_2, '_3, F> CxFnOnce<'_0, '_1, '_2, '_3, (F,)> for generic_fn_1<F>
where
    for<'_a0, '_a1, '_a2, '_a3> F: CxFn<'_a0, '_a1, '_a2, '_a3, (), Output = ()>,
    for<'_a0, '_a1, '_a2, '_a3> (
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context,
        Store<Tuple4<Some<HandleRef<'_a0, __alloc>>, None, None, None>>,
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3> UnifiedContext<'_a0, '_a1, '_a2, '_a3, F>: Copy
        + Coerce<LocalContext<'_a0, '_a1, '_a2, '_a3>>
        + Coerce<<F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context>,
{
    type Output = ();
    type Context = UnifiedContext<'_0, '_1, '_2, '_3, F>;

    fn cx_call_once(mut self, args: (F,), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3, F> CxFnMut<'_0, '_1, '_2, '_3, (F,)> for generic_fn_1<F>
where
    for<'_a0, '_a1, '_a2, '_a3> F: CxFn<'_a0, '_a1, '_a2, '_a3, (), Output = ()>,
    for<'_a0, '_a1, '_a2, '_a3> (
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context,
        Store<Tuple4<Some<HandleRef<'_a0, __alloc>>, None, None, None>>,
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3> UnifiedContext<'_a0, '_a1, '_a2, '_a3, F>: Copy
        + Coerce<LocalContext<'_a0, '_a1, '_a2, '_a3>>
        + Coerce<<F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context>,
{
    fn cx_call_mut(&mut self, args: (F,), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3, F> CxFn<'_0, '_1, '_2, '_3, (F,)> for generic_fn_1<F>
where
    for<'_a0, '_a1, '_a2, '_a3> F: CxFn<'_a0, '_a1, '_a2, '_a3, (), Output = ()>,
    for<'_a0, '_a1, '_a2, '_a3> (
        <F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context,
        Store<Tuple4<Some<HandleRef<'_a0, __alloc>>, None, None, None>>,
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3> UnifiedContext<'_a0, '_a1, '_a2, '_a3, F>: Copy
        + Coerce<LocalContext<'_a0, '_a1, '_a2, '_a3>>
        + Coerce<<F as CxFnOnce<'_a0, '_a1, '_a2, '_a3, ()>>::Context>,
{
    fn cx_call(&self, (f,): (F,), cx: Self::Context) -> Self::Output {
        {
            let cx = cx.clone().coerce();
            f.cx_call((), cx)
        }

        let local_cx: MakeContext<(&__alloc,)> = cx.coerce();
        let _alloc = local_cx.extract_ref::<__alloc>();
    }
}

struct generic_fn_2<'alloc, 'my_vec, F>(PhantomData<fn(F, &'alloc (), &'my_vec ())>)
where
    'alloc: 'my_vec,
    for<'_2, '_3> F: CxFn<'alloc, 'my_vec, '_2, '_3, (), Output = ()>,
    for<'_a2, '_a3> (
        <F as CxFnOnce<'alloc, 'my_vec, '_a2, '_a3, ()>>::Context,
        Store<Tuple4<Some<HandleRef<'alloc, __alloc>>, None, None, None>>,
    ): Unified,
    for<'_a2, '_a3> UnifiedContext<'alloc, 'my_vec, '_a2, '_a3, F>: Copy
        + Coerce<LocalContext<'alloc, 'my_vec, '_a2, '_a3>>
        + Coerce<<F as CxFnOnce<'alloc, 'my_vec, '_a2, '_a3, ()>>::Context>;

impl<'alloc, 'my_vec, '_2, '_3, F> CxFnOnce<'alloc, 'my_vec, '_2, '_3, (F,)>
    for generic_fn_2<'alloc, 'my_vec, F>
where
    for<'_a2, '_a3> F: CxFn<'alloc, 'my_vec, '_a2, '_a3, (), Output = ()>,
    for<'_a2, '_a3> (
        <F as CxFnOnce<'alloc, 'my_vec, '_a2, '_a3, ()>>::Context,
        Store<Tuple4<Some<HandleRef<'alloc, __alloc>>, None, None, None>>,
    ): Unified,
    for<'_a2, '_a3> UnifiedContext<'alloc, 'my_vec, '_a2, '_a3, F>: Copy
        + Coerce<LocalContext<'alloc, 'my_vec, '_a2, '_a3>>
        + Coerce<<F as CxFnOnce<'alloc, 'my_vec, '_a2, '_a3, ()>>::Context>,
{
    type Output = ();
    type Context = UnifiedContext<'alloc, 'my_vec, '_2, '_3, F>;

    fn cx_call_once(mut self, args: (F,), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'alloc, 'my_vec, '_2, '_3, F> CxFnMut<'alloc, 'my_vec, '_2, '_3, (F,)>
    for generic_fn_2<'alloc, 'my_vec, F>
where
    for<'_a2, '_a3> F: CxFn<'alloc, 'my_vec, '_a2, '_a3, (), Output = ()>,
    for<'_a2, '_a3> (
        <F as CxFnOnce<'alloc, 'my_vec, '_a2, '_a3, ()>>::Context,
        Store<Tuple4<Some<HandleRef<'alloc, __alloc>>, None, None, None>>,
    ): Unified,
    for<'_a2, '_a3> UnifiedContext<'alloc, 'my_vec, '_a2, '_a3, F>: Copy
        + Coerce<LocalContext<'alloc, 'my_vec, '_a2, '_a3>>
        + Coerce<<F as CxFnOnce<'alloc, 'my_vec, '_a2, '_a3, ()>>::Context>,
{
    fn cx_call_mut(&mut self, args: (F,), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'alloc, 'my_vec, '_2, '_3, F> CxFn<'alloc, 'my_vec, '_2, '_3, (F,)>
    for generic_fn_2<'alloc, 'my_vec, F>
where
    for<'_a2, '_a3> F: CxFn<'alloc, 'my_vec, '_a2, '_a3, (), Output = ()>,
    for<'_a2, '_a3> (
        <F as CxFnOnce<'alloc, 'my_vec, '_a2, '_a3, ()>>::Context,
        Store<Tuple4<Some<HandleRef<'alloc, __alloc>>, None, None, None>>,
    ): Unified,
    for<'_a2, '_a3> UnifiedContext<'alloc, 'my_vec, '_a2, '_a3, F>: Copy
        + Coerce<LocalContext<'alloc, 'my_vec, '_a2, '_a3>>
        + Coerce<<F as CxFnOnce<'alloc, 'my_vec, '_a2, '_a3, ()>>::Context>,
{
    fn cx_call(&self, (f,): (F,), cx: Self::Context) -> Self::Output {
        {
            let cx = cx.clone().coerce();
            f.cx_call((), cx)
        }

        let local_cx: MakeContext<(&__alloc,)> = cx.coerce();
        let _alloc = local_cx.extract_ref::<__alloc>();
    }
}

// Because `my_vec` embeds `'alloc` we need to introduce bound between lifetimes on those
// capabilities.
// By necessity it means both have to be early-bound.
struct early_flavor_1<'alloc, 'my_vec>(PhantomData<fn(&'alloc (), &'my_vec ())>)
where
    'alloc: 'my_vec;

impl<'_0, '_1, '_2, '_3, 'alloc, 'my_vec> CxFnOnce<'_0, '_1, '_2, '_3, ()>
    for early_flavor_1<'alloc, 'my_vec>
{
    type Output = ();
    type Context = MakeContext<(&'alloc __alloc, &'my_vec __my_vec<'alloc>)>;

    fn cx_call_once(mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3, 'alloc, 'my_vec> CxFnMut<'_0, '_1, '_2, '_3, ()>
    for early_flavor_1<'alloc, 'my_vec>
{
    fn cx_call_mut(&mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3, 'alloc, 'my_vec> CxFn<'_0, '_1, '_2, '_3, ()>
    for early_flavor_1<'alloc, 'my_vec>
{
    fn cx_call(&self, (): (), _cx: Self::Context) -> Self::Output {
        // We have trouble working with mutable references,
        // so let's pretend that we allocate into `my_vec` here.
        // This is a quite realistic use case,
        // so even with shared references it is interesting to see how it pans out.

        println!("first flavor")
    }
}

impl<'alloc, 'my_vec> Default for early_flavor_1<'alloc, 'my_vec> {
    fn default() -> Self {
        early_flavor_1(PhantomData)
    }
}

// Because `my_vec` embeds `'alloc` we need to introduce bound between lifetimes on those
// capabilities.
// By necessity it means both have to be early-bound.
struct early_flavor_2<'alloc, 'my_vec>(PhantomData<fn(&'alloc (), &'my_vec ())>)
where
    'alloc: 'my_vec;

impl<'alloc, 'my_vec, '_2, '_3> CxFnOnce<'alloc, 'my_vec, '_2, '_3, ()>
    for early_flavor_2<'alloc, 'my_vec>
{
    type Output = ();
    type Context = MakeContext<(&'alloc __alloc, &'my_vec __my_vec<'alloc>)>;

    fn cx_call_once(mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'alloc, 'my_vec, '_2, '_3> CxFnMut<'alloc, 'my_vec, '_2, '_3, ()>
    for early_flavor_2<'alloc, 'my_vec>
{
    fn cx_call_mut(&mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'alloc, 'my_vec, '_2, '_3> CxFn<'alloc, 'my_vec, '_2, '_3, ()>
    for early_flavor_2<'alloc, 'my_vec>
{
    fn cx_call(&self, (): (), _cx: Self::Context) -> Self::Output {
        // We have trouble working with mutable references,
        // so let's pretend that we allocate into `my_vec` here.
        // This is a quite realistic use case,
        // so even with shared references it is interesting to see how it pans out.

        println!("second flavor")
    }
}

impl<'alloc, 'my_vec> Default for early_flavor_2<'alloc, 'my_vec> {
    fn default() -> Self {
        early_flavor_2(PhantomData)
    }
}

struct __alloc_helper<A>(A)
where
    A: Allocator + ?Sized;

type __alloc = __alloc_helper<dyn Allocator>;

impl Capability for __alloc {
    type Index = Index<0>;
    type Inner = dyn Allocator;

    fn as_ref(&self) -> &Self::Inner {
        &self.0
    }

    fn as_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

struct __my_vec<'alloc>(Vec<u64, &'alloc dyn Allocator>);

impl<'alloc> Capability for __my_vec<'alloc> {
    type Index = Index<1>;
    type Inner = Vec<u64, &'alloc dyn Allocator>;

    fn as_ref(&self) -> &Self::Inner {
        &self.0
    }

    fn as_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}
