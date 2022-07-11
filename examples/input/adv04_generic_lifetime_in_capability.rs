//! Illustrate usage of generic lifetimes in capability.

#![feature(generic_associated_types)]
#![feature(allocator_api)]
#![allow(non_camel_case_types)]

use rust_context_emulation::prelude_input::*;
use std::alloc::{Allocator, Global};
use std::marker::PhantomData;

fn main() {
    let alloc: &__alloc = &__alloc_helper(Global);
    let mut my_vec = __my_vec(Vec::new_in(&Global));
    let cx = EmptyStore::new().push(alloc).push(&mut my_vec);

    allocate_into::default().cx_call((), cx);

    assert_eq!(my_vec.as_ref(), &[3]);
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

// Because `my_vec` embeds `'alloc` we need to introduce bound between lifetimes on those
// capabilities.
// By necessity it means both have to be early-bound.
struct allocate_into<'alloc, 'my_vec>(PhantomData<fn(&'alloc (), &'my_vec ())>)
where
    'alloc: 'my_vec;

impl<'_0, '_1, '_2, '_3, 'alloc, 'my_vec> CxFnOnce<'_0, '_1, '_2, '_3, ()>
    for allocate_into<'alloc, 'my_vec>
{
    type Output = ();
    type Context = MakeContext<(&'alloc __alloc, &'my_vec mut __my_vec<'alloc>)>;

    fn cx_call_once(mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3, 'alloc, 'my_vec> CxFnMut<'_0, '_1, '_2, '_3, ()>
    for allocate_into<'alloc, 'my_vec>
{
    fn cx_call_mut(&mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3, 'alloc, 'my_vec> CxFn<'_0, '_1, '_2, '_3, ()>
    for allocate_into<'alloc, 'my_vec>
{
    fn cx_call(&self, (): (), mut cx: Self::Context) -> Self::Output {
        let alloc = cx.extract_ref::<__alloc>();
        let my_vec = cx.extract_mut::<__my_vec>();

        let v = {
            let mut v = Vec::new_in(alloc);
            v.push(3);
            v
        };

        *my_vec = v;
    }
}

impl<'alloc, 'my_vec> Default for allocate_into<'alloc, 'my_vec> {
    fn default() -> Self {
        allocate_into(PhantomData)
    }
}
