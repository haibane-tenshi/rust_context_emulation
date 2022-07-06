#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use rust_context_emulation::prelude_input::*;
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
    let _cx = EmptyStore::new()
        .push(&hidden_str)
        .push(&data_store)
        .push(&counter);

    // This doesn't work.
    // Beats me why.
    // The cause is coercion bounds on `generic_fn`, but error message makes no sense.
    // I suspect the cause is GAT on `Reborrow::Output`.
    // let f = generic_fn::<A>(PhantomData);
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

trait WildcardTrait {
    type wildcard_method: Default
        + for<'_0, '_1, '_2, '_3, 'a> CxFn<'_0, '_1, '_2, '_3, (&'a Self,), Output = usize>;
}

type GenericFnContext<'_0, '_1, '_2, '_3, 't, T> = Unify<(
    <<T as WildcardTrait>::wildcard_method as CxFnOnce<'_0, '_1, '_2, '_3, (&'t T,)>>::Context,
    MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __counter>,)>,
)>;

// `T` is a type and must be early bound.
// Also notice that any bound on `T` also go into fn type.
struct generic_fn<T>(PhantomData<T>)
where
    // This is the only meaningful bound here.
    // The rest is related to internal machinery.
    T: WildcardTrait,

    // Welcome to hell.
    // We know the following type manipulation should be valid by convention,
    // however compiler doesn't know about that, so we need to convince it.
    // There are three parts to take care of: unification, reborrowing and coercion.

    // This is the unification bound:
    for<'_0, '_1, '_2, '_3, 't> (
        <<T as WildcardTrait>::wildcard_method as CxFnOnce<'_0, '_1, '_2, '_3, (&'t T,)>>::Context,
        Store<Tuple4<None, None, Some<HandleRef<'_2, __counter>>, None>>,
    ): Unified,

    // This bound is fragile and spills out a lot of guts.
    // Ideally, we would like to write something like this:
    //
    // for<'_0, '_1, '_2, '_3, 't> (
    //     <<T as UsefulWork>::wildcard_method as CxFnOnce<'_0, '_1, '_2, '_3, (&'t T,)>>::Context,
    //     MakeContext<(Select<'_0, '_1, '_2, '_3, Mutable, __counter>,)>,
    // ): Unified,
    //
    // However, compiler for some reason doesn't expand definition of `MakeContext` in this context,
    // so it fails.
    // I still don't understand why it doesn't digest type shorthands in some places but is fine
    // in others.

    // This is reborrowing bound:
    for<'_0, '_1, '_2, '_3, 't> <(
        <<T as WildcardTrait>::wildcard_method as CxFnOnce<'_0, '_1, '_2, '_3, (&'t T,)>>::Context,
        Store<Tuple4<None, None, Some<HandleRef<'_2, __counter>>, None>>,
    ) as Unified>::Output: Reborrow,

    // And this is two coercion bounds, one for each target:
    for<'_0, '_1, '_2, '_3, 't, 's> <GenericFnContext<'_0, '_1, '_2, '_3, 't, T> as Reborrow>::Output<'s>:
        Coerce<
            <<T as WildcardTrait>::wildcard_method as CxFnOnce<'_0, '_1, '_2, '_3, (&'t T,)>>::Context,
        >,
    for<'_0, '_1, '_2, '_3, 't, 's> <GenericFnContext<'_0, '_1, '_2, '_3, 't, T> as Reborrow>::Output<'s>:
        Coerce<MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __counter>,)>>;

// Now, we just need to replicate the insanity on every impl block...
impl<'_0, '_1, '_2, '_3, 't, T> CxFnOnce<'_0, '_1, '_2, '_3, (&'t T,)> for generic_fn<T>
where
    T: WildcardTrait,
    for<'_a0, '_a1, '_a2, '_a3, 'at> (
        <<T as WildcardTrait>::wildcard_method as CxFnOnce<'_a0, '_a1, '_a2, '_a3, (&'at T,)>>::Context,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3, 'at> <(
        <<T as WildcardTrait>::wildcard_method as CxFnOnce<'_a0, '_a1, '_a2, '_a3, (&'at T,)>>::Context,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>
    ) as Unified>::Output: Reborrow,
    for<'_a0, '_a1, '_a2, '_a3, 'at, 's> <GenericFnContext<'_a0, '_a1, '_a2, '_a3, 'at, T> as Reborrow>::Output<'s>: Coerce<
        <<T as WildcardTrait>::wildcard_method as CxFnOnce<'_a0, '_a1, '_a2, '_a3, (&'at T,)>>::Context,
    >,
    for<'_a0, '_a1, '_a2, '_a3, 'at, 's> <GenericFnContext<'_a0, '_a1, '_a2, '_a3, 'at, T> as Reborrow>::Output<'s>:
    Coerce<MakeContext<(Select<'_a0, '_a1, '_a2, '_a3, Shared, __counter>,)>>
{
    type Output = usize;
    type Context = GenericFnContext<'_0, '_1, '_2, '_3, 't, T>;

    fn cx_call_once(
        mut self,
        args: (&'t T,),
        cx: Self::Context,
    ) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3, 't, T> CxFnMut<'_0, '_1, '_2, '_3, (&'t T,)> for generic_fn<T>
where
    T: WildcardTrait,
    for<'_a0, '_a1, '_a2, '_a3, 'at> (
        <<T as WildcardTrait>::wildcard_method as CxFnOnce<'_a0, '_a1, '_a2, '_a3, (&'at T,)>>::Context,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3, 'at> <(
        <<T as WildcardTrait>::wildcard_method as CxFnOnce<'_a0, '_a1, '_a2, '_a3, (&'at T,)>>::Context,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>
    ) as Unified>::Output: Reborrow,
    for<'_a0, '_a1, '_a2, '_a3, 'at, 's> <GenericFnContext<'_a0, '_a1, '_a2, '_a3, 'at, T> as Reborrow>::Output<'s>: Coerce<
        <<T as WildcardTrait>::wildcard_method as CxFnOnce<'_a0, '_a1, '_a2, '_a3, (&'at T,)>>::Context,
    >,
    for<'_a0, '_a1, '_a2, '_a3, 'at, 's> <GenericFnContext<'_a0, '_a1, '_a2, '_a3, 'at, T> as Reborrow>::Output<'s>:
    Coerce<MakeContext<(Select<'_a0, '_a1, '_a2, '_a3, Shared, __counter>,)>>
{
    fn cx_call_mut(&mut self, args: (&'t T, ), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3, 't, T> CxFn<'_0, '_1, '_2, '_3, (&'t T,)> for generic_fn<T>
where
    T: WildcardTrait,
    for<'_a0, '_a1, '_a2, '_a3, 'at> (
        <<T as WildcardTrait>::wildcard_method as CxFnOnce<'_a0, '_a1, '_a2, '_a3, (&'at T,)>>::Context,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3, 'at> <(
        <<T as WildcardTrait>::wildcard_method as CxFnOnce<'_a0, '_a1, '_a2, '_a3, (&'at T,)>>::Context,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>
    ) as Unified>::Output: Reborrow,
    for<'_a0, '_a1, '_a2, '_a3, 'at, 's> <GenericFnContext<'_a0, '_a1, '_a2, '_a3, 'at, T> as Reborrow>::Output<'s>: Coerce<
        <<T as WildcardTrait>::wildcard_method as CxFnOnce<'_a0, '_a1, '_a2, '_a3, (&'at T,)>>::Context,
    >,
    for<'_a0, '_a1, '_a2, '_a3, 'at, 's> <GenericFnContext<'_a0, '_a1, '_a2, '_a3, 'at, T> as Reborrow>::Output<'s>:
    Coerce<MakeContext<(Select<'_a0, '_a1, '_a2, '_a3, Shared, __counter>,)>>
{
    fn cx_call(&self, (t,): (&'t T, ), mut cx: Self::Context) -> Self::Output {
        let __local_cx: MakeContext<(&__counter,)> = cx.reborrow().coerce();
        let counter = __local_cx.extract_ref::<__counter>();

        let value = {
            let cx = cx.reborrow().coerce();
            <T as WildcardTrait>::wildcard_method::default().cx_call((t, ), cx)
        };

        *counter + value
    }
}

impl<T> Default for generic_fn<T>
where
    T: WildcardTrait,
    for<'_a0, '_a1, '_a2, '_a3, 'at> (
        <<T as WildcardTrait>::wildcard_method as CxFnOnce<'_a0, '_a1, '_a2, '_a3, (&'at T,)>>::Context,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>
    ): Unified,
    for<'_a0, '_a1, '_a2, '_a3, 'at> <(
        <<T as WildcardTrait>::wildcard_method as CxFnOnce<'_a0, '_a1, '_a2, '_a3, (&'at T,)>>::Context,
        Store<Tuple4<None, None, Some<HandleRef<'_a2, __counter>>, None>>
    ) as Unified>::Output: Reborrow,
    for<'_a0, '_a1, '_a2, '_a3, 'at, 's> <GenericFnContext<'_a0, '_a1, '_a2, '_a3, 'at, T> as Reborrow>::Output<'s>: Coerce<
        <<T as WildcardTrait>::wildcard_method as CxFnOnce<'_a0, '_a1, '_a2, '_a3, (&'at T,)>>::Context,
    >,
    for<'_a0, '_a1, '_a2, '_a3, 'at, 's> <GenericFnContext<'_a0, '_a1, '_a2, '_a3, 'at, T> as Reborrow>::Output<'s>:
    Coerce<MakeContext<(Select<'_a0, '_a1, '_a2, '_a3, Shared, __counter>,)>>
{
    fn default() -> Self {
        generic_fn::<T>(PhantomData)
    }
}

struct A;

impl WildcardTrait for A {
    type wildcard_method = a_wildcard_method;
}

#[derive(Default)]
struct a_wildcard_method;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, (&A,)> for a_wildcard_method {
    type Output = usize;
    type Context = MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __data_store>,)>;

    fn cx_call_once(mut self, args: (&A,), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, (&A,)> for a_wildcard_method {
    fn cx_call_mut(&mut self, args: (&A,), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, (&A,)> for a_wildcard_method {
    fn cx_call(&self, (_,): (&A,), cx: Self::Context) -> Self::Output {
        let data_store = cx.extract_ref::<__data_store>();
        data_store.first().copied().unwrap_or_default()
    }
}

struct B;

impl WildcardTrait for B {
    type wildcard_method = b_wildcard_method;
}

#[derive(Default)]
struct b_wildcard_method;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, (&B,)> for b_wildcard_method {
    type Output = usize;
    type Context = MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __hidden_str>,)>;

    fn cx_call_once(mut self, args: (&B,), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, (&B,)> for b_wildcard_method {
    fn cx_call_mut(&mut self, args: (&B,), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, (&B,)> for b_wildcard_method {
    fn cx_call(&self, (_,): (&B,), cx: Self::Context) -> Self::Output {
        let hidden_str = cx.extract_ref::<__hidden_str>();

        hidden_str.len()
    }
}
