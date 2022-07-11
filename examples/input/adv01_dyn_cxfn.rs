//! Illustrate creation of `CxFn` trait object.

#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::unused_unit)]

use rust_context_emulation::prelude_input::*;

fn main() {
    let mut counter = __counter(0);
    let mut f: &dyn for<'_0, '_1, '_2, '_3> CxFn<
        '_0,
        '_1,
        '_2,
        '_3,
        (),
        Output = (),
        Context = FixedContext<'_0, '_1, '_2, '_3>,
    > = &plus_one;
    {
        let cx = EmptyStore::new().push(&mut counter);
        f.cx_call((), cx);
    }

    assert_eq!(counter.0, 1);

    f = &minus_one;
    {
        let cx = EmptyStore::new().push(&mut counter);
        f.cx_call((), cx);
    }

    assert_eq!(counter.0, 0);
}

type FixedContext<'_0, '_1, '_2, '_3> =
    MakeContext<(Select<'_0, '_1, '_2, '_3, Mutable, __counter>,)>;

struct __counter(u32);

impl Capability for __counter {
    type Index = Index<1>;
    type Inner = u32;

    fn as_ref(&self) -> &Self::Inner {
        &self.0
    }

    fn as_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

#[derive(Default)]
struct plus_one;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, ()> for plus_one {
    type Output = ();
    type Context = FixedContext<'_0, '_1, '_2, '_3>;

    fn cx_call_once(mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, ()> for plus_one {
    fn cx_call_mut(&mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, ()> for plus_one {
    fn cx_call(&self, (): (), mut cx: Self::Context) -> Self::Output {
        let counter = cx.extract_mut::<__counter>();
        *counter += 1;
    }
}

#[derive(Default)]
struct minus_one;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, ()> for minus_one {
    type Output = ();
    type Context = FixedContext<'_0, '_1, '_2, '_3>;

    fn cx_call_once(mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, ()> for minus_one {
    fn cx_call_mut(&mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, ()> for minus_one {
    fn cx_call(&self, (): (), mut cx: Self::Context) -> Self::Output {
        let counter = cx.extract_mut::<__counter>();
        *counter -= 1;
    }
}
