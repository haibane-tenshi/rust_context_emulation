//! Illustrate construction and implementation of traits containing contextual methods.

#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]

use rust_context_emulation::prelude_input::*;

fn main() {
    let hidden_str = __hidden_str("treasure".to_string());
    let mut data_store = __data_store(vec![3]);
    let mut cx = EmptyStore::new().push(&hidden_str).push(&mut data_store);

    {
        let cx = cx.reborrow().coerce();
        <A as MyPush>::concrete_push::default().cx_call((4,), cx);
    }

    {
        let cx = cx.reborrow().coerce();
        <A as MyPush>::wildcard_push::default().cx_call((&A, 5), cx);
    }

    assert_eq!(&data_store.0, &[3, 4, 5 + 8]);
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

struct __data_store(Vec<u32>);

impl Capability for __data_store {
    type Index = Index<1>;
    type Inner = Vec<u32>;

    fn as_ref(&self) -> &Self::Inner {
        &self.0
    }

    fn as_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

// This is trait representation.
// Instead of writing function declarations we specify only their types.
// `Default` trait is used to spawn values of those types at will.
trait MyPush {
    // This method is specified with concrete context.
    // All implementors have to follow the suit.
    type concrete_push: Default
        + for<'_0, '_1, '_2, '_3> CxFn<
            '_0,
            '_1,
            '_2,
            '_3,
            (u32,),
            Output = (),
            Context = MakeContext<(Select<'_0, '_1, '_2, '_3, Mutable, __data_store>,)>,
        >;

    // This method is specified with wildcard context.
    type wildcard_push: Default
        + for<'_0, '_1, '_2, '_3, 'a> CxFn<'_0, '_1, '_2, '_3, (&'a Self, u32), Output = ()>;
}

struct A;

// This is how trait implementation looks like.
// We have to define a new type for each "method" and assign it in.
impl MyPush for A {
    type concrete_push = a_concrete_push;
    type wildcard_push = a_wildcard_push;
}

#[derive(Default)]
struct a_concrete_push;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, (u32,)> for a_concrete_push {
    type Output = ();
    type Context = MakeContext<(Select<'_0, '_1, '_2, '_3, Mutable, __data_store>,)>;

    fn cx_call_once(mut self, args: (u32,), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, (u32,)> for a_concrete_push {
    fn cx_call_mut(&mut self, args: (u32,), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, (u32,)> for a_concrete_push {
    fn cx_call(&self, (val,): (u32,), mut cx: Self::Context) -> Self::Output {
        let data_store = cx.extract_mut::<__data_store>();
        data_store.push(val);
    }
}

#[derive(Default)]
struct a_wildcard_push;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, (&A, u32)> for a_wildcard_push {
    type Output = ();
    type Context = MakeContext<(
        Select<'_0, '_1, '_2, '_3, Shared, __hidden_str>,
        Select<'_0, '_1, '_2, '_3, Mutable, __data_store>,
    )>;

    fn cx_call_once(mut self, args: (&A, u32), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, (&A, u32)> for a_wildcard_push {
    fn cx_call_mut(&mut self, args: (&A, u32), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, (&A, u32)> for a_wildcard_push {
    fn cx_call(&self, (_, val): (&A, u32), mut cx: Self::Context) -> Self::Output {
        let hidden_str = cx.extract_ref::<__hidden_str>();
        let value = val + hidden_str.len() as u32;

        let data_store = cx.extract_mut::<__data_store>();
        data_store.push(value);
    }
}
