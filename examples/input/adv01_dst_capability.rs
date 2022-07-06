#![feature(generic_associated_types)]
#![feature(allocator_api)]
#![allow(non_camel_case_types)]

use rust_context_emulation::prelude_input::*;
use std::alloc::{Allocator, Global};
use std::marker::PhantomData;

fn main() {
    let alloc: &__alloc = &__alloc_helper(Global);
    let cx = EmptyStore::new().push(alloc);

    let mut v = vec_new::default().cx_call((), cx);
    // `Vec` remembers its allocator, so calling other (even potentially allocating) methods
    // can be done normally.
    v.push(3_u32);

    assert_eq!(&v, &[3_u32]);
}

// This machinery is required to construct "custom" DSTs.
// For more details refer to the Nomicon pages:
// https://doc.rust-lang.org/nomicon/exotic-sizes.html#dynamically-sized-types-dsts
struct __alloc_helper<T: ?Sized>(T);
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

#[derive(Default)]
struct vec_new<'alloc, T>(PhantomData<&'alloc ()>, PhantomData<T>);

impl<'_0, '_1, '_2, '_3, 'alloc, T> CxFnOnce<'_0, '_1, '_2, '_3, ()> for vec_new<'alloc, T> {
    type Output = Vec<T, &'alloc dyn Allocator>;
    type Context = MakeContext<(&'alloc __alloc,)>;

    fn cx_call_once(mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3, 'alloc, T> CxFnMut<'_0, '_1, '_2, '_3, ()> for vec_new<'alloc, T> {
    fn cx_call_mut(&mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3, 'alloc, T> CxFn<'_0, '_1, '_2, '_3, ()> for vec_new<'alloc, T> {
    fn cx_call(&self, (): (), cx: Self::Context) -> Self::Output {
        let alloc = cx.extract_ref::<__alloc>();

        Vec::new_in(alloc)
    }
}
