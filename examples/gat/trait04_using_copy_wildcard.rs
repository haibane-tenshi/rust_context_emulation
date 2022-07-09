#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
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

    // In this example we abandon support for mutable references altogether.
    // This allows us to go away with reborrowing and actually make things work.

    let value = {
        let cx = cx.clone().coerce();
        generic_fn::default().cx_call((&A,), cx)
    };

    assert_eq!(value, 8);

    let value = {
        let cx = cx.clone().coerce();
        generic_fn::default().cx_call((&B,), cx)
    };

    assert_eq!(value, 13);
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

trait UsefulWork {
    type wildcard_method: Default
        + for<'_0, '_1, '_2, '_3, 'a> CxFn<(&'a Self,), Output<'_0, '_1, '_2, '_3> = usize>;
}

type GenericFnContext<'_0, '_1, '_2, '_3, 't, T> = Unify<(
    <<T as UsefulWork>::wildcard_method as CxFnOnce<(&'t T,)>>::Context<'_0, '_1, '_2, '_3>,
    MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __counter>,)>,
)>;

// `T` is a type and must be early bound.
// Also notice that any bound on `T` also go into fn type.
#[rustfmt::skip]
struct generic_fn<T>(PhantomData<T>)
where
    // This is the only meaningful bound here.
    // The rest is related to internal machinery.
    T: UsefulWork,

    // Welcome to hell.
    // We know the following type manipulation should be valid by convention,
    // however compiler doesn't know about that, so we need to convince it.
    // There are three parts to take care of: unification, reborrowing and coercion.

    // This is the unification bound:
    for<'_0, '_1, '_2, '_3, 't> (
        <<T as UsefulWork>::wildcard_method as CxFnOnce<(&'t T,)>>::Context<'_0, '_1, '_2, '_3>,
        Store<Tuple4<None, None, Some<HandleRef<'_2, __counter>>, None>>,
    ): Unified,

    // This is "reborrowing" bound.
    // We don't intend to support mutable references which makes context `Copy`.
    for<'_0, '_1, '_2, '_3, 't> GenericFnContext<'_0, '_1, '_2, '_3, 't, T>: Copy,

    // And this is two coercion bounds, one for each target.
    // They apply directly to unified context, bypassing reborrowing cruft thanks to `Copy`.
    for<'_0, '_1, '_2, '_3, 't, 's> GenericFnContext<'_0, '_1, '_2, '_3, 't, T>: Coerce<
        <<T as UsefulWork>::wildcard_method as CxFnOnce<(&'t T,)>>::Context<'_0, '_1, '_2, '_3>,
    >,
    for<'_0, '_1, '_2, '_3, 't, 's> GenericFnContext<'_0, '_1, '_2, '_3, 't, T>:
        Coerce<MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __counter>,)>>;

// Now, we just need to replicate the insanity on every impl block...
impl<'t, T> CxFnOnce<(&'t T,)> for generic_fn<T>
where
    T: UsefulWork,
    for<'_0, '_1, '_2, '_3, 'at> (
        <<T as UsefulWork>::wildcard_method as CxFnOnce<(&'at T,)>>::Context<'_0, '_1, '_2, '_3>,
        Store<Tuple4<None, None, Some<HandleRef<'_2, __counter>>, None>>,
    ): Unified,
    for<'_0, '_1, '_2, '_3, 'at> GenericFnContext<'_0, '_1, '_2, '_3, 'at, T>: Copy
        + Coerce<
            <<T as UsefulWork>::wildcard_method as CxFnOnce<(&'at T,)>>::Context<
                '_0,
                '_1,
                '_2,
                '_3,
            >,
        > + Coerce<MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __counter>,)>>,
{
    type Output<'_0, '_1, '_2, '_3> = usize;
    type Context<'_0, '_1, '_2, '_3> = GenericFnContext<'_0, '_1, '_2, '_3, 't, T>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (&'t T,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl<'t, T> CxFnMut<(&'t T,)> for generic_fn<T>
where
    T: UsefulWork,
    for<'_0, '_1, '_2, '_3, 'at> (
        <<T as UsefulWork>::wildcard_method as CxFnOnce<(&'at T,)>>::Context<'_0, '_1, '_2, '_3>,
        Store<Tuple4<None, None, Some<HandleRef<'_2, __counter>>, None>>,
    ): Unified,
    for<'_0, '_1, '_2, '_3, 'at> GenericFnContext<'_0, '_1, '_2, '_3, 'at, T>: Copy
        + Coerce<
            <<T as UsefulWork>::wildcard_method as CxFnOnce<(&'at T,)>>::Context<
                '_0,
                '_1,
                '_2,
                '_3,
            >,
        > + Coerce<MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __counter>,)>>,
{
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (&'t T,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl<'t, T> CxFn<(&'t T,)> for generic_fn<T>
where
    T: UsefulWork,
    for<'_0, '_1, '_2, '_3, 'at> (
        <<T as UsefulWork>::wildcard_method as CxFnOnce<(&'at T,)>>::Context<'_0, '_1, '_2, '_3>,
        Store<Tuple4<None, None, Some<HandleRef<'_2, __counter>>, None>>,
    ): Unified,
    for<'_0, '_1, '_2, '_3, 'at> GenericFnContext<'_0, '_1, '_2, '_3, 'at, T>: Copy
        + Coerce<
            <<T as UsefulWork>::wildcard_method as CxFnOnce<(&'at T,)>>::Context<
                '_0,
                '_1,
                '_2,
                '_3,
            >,
        > + Coerce<MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __counter>,)>>,
{
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (t,): (&'t T,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let __local_cx: MakeContext<(&__counter,)> = cx.clone().coerce();
        let counter = __local_cx.extract_ref::<__counter>();

        let value = {
            let cx = cx.clone().coerce();
            <T as UsefulWork>::wildcard_method::default().cx_call((t,), cx)
        };

        *counter + value
    }
}

impl<T> Default for generic_fn<T>
where
    T: UsefulWork,
    for<'_0, '_1, '_2, '_3, 'at> (
        <<T as UsefulWork>::wildcard_method as CxFnOnce<(&'at T,)>>::Context<'_0, '_1, '_2, '_3>,
        Store<Tuple4<None, None, Some<HandleRef<'_2, __counter>>, None>>,
    ): Unified,
    for<'_0, '_1, '_2, '_3, 'at> GenericFnContext<'_0, '_1, '_2, '_3, 'at, T>: Copy
        + Coerce<
            <<T as UsefulWork>::wildcard_method as CxFnOnce<(&'at T,)>>::Context<
                '_0,
                '_1,
                '_2,
                '_3,
            >,
        > + Coerce<MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __counter>,)>>,
{
    fn default() -> Self {
        generic_fn::<T>(PhantomData)
    }
}

struct A;

impl UsefulWork for A {
    type wildcard_method = a_wildcard_method;
}

#[derive(Default)]
struct a_wildcard_method;

impl CxFnOnce<(&A,)> for a_wildcard_method {
    type Output<'_0, '_1, '_2, '_3> = usize;
    type Context<'_0, '_1, '_2, '_3> =
        MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __data_store>,)>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (&A,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl CxFnMut<(&A,)> for a_wildcard_method {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (&A,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl CxFn<(&A,)> for a_wildcard_method {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (_,): (&A,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let data_store = cx.extract_ref::<__data_store>();

        data_store.first().copied().unwrap_or_default()
    }
}

struct B;

impl UsefulWork for B {
    type wildcard_method = b_wildcard_method;
}

#[derive(Default)]
struct b_wildcard_method;

impl CxFnOnce<(&B,)> for b_wildcard_method {
    type Output<'_0, '_1, '_2, '_3> = usize;
    type Context<'_0, '_1, '_2, '_3> =
        MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __hidden_str>,)>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (&B,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl CxFnMut<(&B,)> for b_wildcard_method {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (&B,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl CxFn<(&B,)> for b_wildcard_method {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (_,): (&B,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let hidden_str = cx.extract_ref::<__hidden_str>();

        hidden_str.len()
    }
}
