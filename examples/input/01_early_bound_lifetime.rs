//! Illustrate desugaring of a function with early-bound capability lifetime.
//!
//! There are actually two possible flavors.
//! I *think* the first one is correct...
//! but proving that requires to check every single edge case (and there is a lot of them).
//! It is also possible that both have use just in different situations.

#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]

use std::marker::PhantomData;

use rust_context_emulation::prelude_input::*;

fn main() {
    let data_store = __data_store(vec![3_usize]);
    let cx = EmptyStore::new().push(&data_store);

    let first_item = first::default().cx_call((), cx);
    assert_eq!(first_item, Some(&3));

    let first_item = first_alternative::default().cx_call((), cx);
    assert_eq!(first_item, Some(&3));
}

// Let's pretend this is necessary.
// I tried to come up with simple example where it is actually required, but failed.
// All more-or-less realistic cases involve advanced features.
// Take a look at advanced examples for real situations where early-bound lifetimes are needed.
#[derive(Default)]
struct first<'data_store>(PhantomData<fn(&'data_store ())>);

// This flavor assumes that lifetimes inside `CxFn*` exist only for the case when
// you need late-bound lifetimes, i.e. there is no relationship between those.
// I *think* this is correct, but I could be very wrong.
// All other examples assume this model too, in part for simplicity.
impl<'_0, '_1, '_2, '_3, 'data_store> CxFnOnce<'_0, '_1, '_2, '_3, ()> for first<'data_store> {
    type Output = Option<&'data_store usize>;
    type Context = MakeContext<(&'data_store __data_store,)>;

    fn cx_call_once(mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3, 'data_store> CxFnMut<'_0, '_1, '_2, '_3, ()> for first<'data_store> {
    fn cx_call_mut(&mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3, 'data_store> CxFn<'_0, '_1, '_2, '_3, ()> for first<'data_store> {
    fn cx_call(&self, (): (), cx: Self::Context) -> Self::Output {
        let data_store = cx.extract_ref::<__data_store>();

        data_store.first()
    }
}

#[derive(Default)]
struct first_alternative<'data_store>(PhantomData<fn(&'data_store ())>);

// In this flavor we strongly encode relationship between capability lifetime declared
// on the function with the one declared on the trait.
// Unfortunately it means we need to manually assign lifetime slots to capabilities,
// there is no way to construct machinery similar to `Select`.
// Also pure-GAT desugaring doesn't have this option,
// so if this style of handling proves to be more correct,
// it can potentially render GAT approach unusable.
// To the best of my knowledge there is no significant difference in behavior or expressivity.
impl<'data_store, '_1, '_2, '_3> CxFnOnce<'data_store, '_1, '_2, '_3, ()>
    for first_alternative<'data_store>
{
    // Desired lifetime is already known, so extra lifetimes can just be ignored.
    type Output = Option<&'data_store usize>;
    type Context = MakeContext<(&'data_store __data_store,)>;

    fn cx_call_once(mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'data_store, '_1, '_2, '_3> CxFnMut<'data_store, '_1, '_2, '_3, ()>
    for first_alternative<'data_store>
{
    fn cx_call_mut(&mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'data_store, '_1, '_2, '_3> CxFn<'data_store, '_1, '_2, '_3, ()>
    for first_alternative<'data_store>
{
    fn cx_call(&self, (): (), cx: Self::Context) -> Self::Output {
        let data_store = cx.extract_ref::<__data_store>();

        data_store.first()
    }
}

struct __data_store(Vec<usize>);

impl Capability for __data_store {
    type Index = Index<0>;
    type Inner = Vec<usize>;

    fn as_ref(&self) -> &Self::Inner {
        &self.0
    }

    fn as_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}
