#![feature(generic_associated_types)]
#![feature(allocator_api)]
#![allow(non_camel_case_types)]

use rust_context_emulation::prelude_gat::*;
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

// This machinery is required to construct custom DSTs.
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

impl<'alloc, T> CxFnOnce<()> for vec_new<'alloc, T> {
    type Output<'_0, '_1, '_2, '_3> = Vec<T, &'alloc dyn Allocator>;
    type Context<'_0, '_1, '_2, '_3> = MakeContext<(&'alloc __alloc,)>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl<'alloc, T> CxFnMut<()> for vec_new<'alloc, T> {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl<'alloc, T> CxFn<()> for vec_new<'alloc, T> {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (): (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let alloc = cx.extract_ref::<__alloc>();

        Vec::new_in(alloc)
    }
}
