#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]

use std::marker::PhantomData;

use rust_context_emulation::prelude_input::*;

fn main() {
    let data_store = __data_store(vec![3_usize]);
    let cx = EmptyStore::new().push(&data_store);

    let first_item = first::default().cx_call((), cx);

    assert_eq!(first_item, Some(&3));
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

// Because we want the lifetime appear in return type it must be part of the function type.
#[derive(Default)]
struct first<'data_store>(PhantomData<&'data_store ()>);

impl<'_0, '_1, '_2, '_3, 'data_store> CxFnOnce<'_0, '_1, '_2, '_3, ()> for first<'data_store> {
    type Output = Option<&'data_store usize>;

    // Desired lifetime is already known, so extra lifetimes can just be ignored.
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
