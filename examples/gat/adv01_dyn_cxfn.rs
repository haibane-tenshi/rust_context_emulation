//! Illustrate creation of `CxFn` pseudo-`dyn` object.

#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::unused_unit)]

use rust_context_emulation::prelude_gat::*;

fn main() {
    let mut counter = __counter(0);
    let mut f = DynCxFn::coerce_from(&plus_one);
    {
        let cx = EmptyStore::new().push(&mut counter);
        f.cx_call((), cx);
    }

    assert_eq!(counter.0, 1);

    f = DynCxFn::coerce_from(&minus_one);
    {
        let cx = EmptyStore::new().push(&mut counter);
        f.cx_call((), cx);
    }

    assert_eq!(counter.0, 0);
}

type FixedContext<'_0, '_1, '_2, '_3> =
    MakeContext<(Select<'_0, '_1, '_2, '_3, Mutable, __counter>,)>;

// We create a specific VTable for a class of functions.
// I tried to genericize the approach but hit a wall.
// Someone might have better luck than me.
struct CxFnVTable {
    pub cx_call: for<'_0, '_1, '_2, '_3> fn(
        *const (),                        // Self
        (),                               // Args
        FixedContext<'_0, '_1, '_2, '_3>, // Context
    ) -> (),
}

struct DynCxFn {
    vtable: CxFnVTable,
    this: *const (),
}

impl DynCxFn {
    pub fn coerce_from<F>(f: &F) -> Self
    where
        F: for<'_0, '_1, '_2, '_3> CxFn<
            (),
            Output<'_0, '_1, '_2, '_3> = (),
            Context<'_0, '_1, '_2, '_3> = FixedContext<'_0, '_1, '_2, '_3>,
        >,
    {
        let cx_call = |this: *const (), args: (), cx: FixedContext<'_, '_, '_, '_>| -> () {
            unsafe {
                let this = &*(this as *const F);
                this.cx_call(args, cx)
            }
        };

        let vtable = CxFnVTable { cx_call };

        DynCxFn {
            vtable,
            this: f as *const F as *const (),
        }
    }

    pub fn coerce_from2<F>(f: &F) -> Self
    where
        // Technically, this is a more general function:
        // it also accepts contextual functions which have "narrower" context than prescribed.
        // Unfortunately, it doesn't work: compiler fails to follow types through GAT.
        // Not sure, but there might be a way to work around this using approach
        // hinted by second issue.
        // Related issues:
        // https://github.com/rust-lang/rust/issues/93341
        // https://github.com/rust-lang/rust/issues/93342
        for<'_0, '_1, '_2, '_3> F: CxFn<(), Output<'_0, '_1, '_2, '_3> = ()>,
        for<'_0, '_1, '_2, '_3> FixedContext<'_0, '_1, '_2, '_3>:
            Coerce<<F as CxFnOnce<()>>::Context<'_0, '_1, '_2, '_3>>,
    {
        let cx_call = |this: *const (), args: (), cx: FixedContext<'_, '_, '_, '_>| -> () {
            unsafe {
                let this = &*(this as *const F);
                let cx = cx.coerce();
                this.cx_call(args, cx)
            }
        };

        let vtable = CxFnVTable { cx_call };

        DynCxFn {
            vtable,
            this: f as *const F as *const (),
        }
    }
}

impl CxFnOnce<()> for DynCxFn {
    type Output<'_0, '_1, '_2, '_3> = ();
    type Context<'_0, '_1, '_2, '_3> = FixedContext<'_0, '_1, '_2, '_3>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl CxFnMut<()> for DynCxFn {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl CxFn<()> for DynCxFn {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        (self.vtable.cx_call)(self.this, args, cx)
    }
}

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

impl CxFnOnce<()> for plus_one {
    type Output<'_0, '_1, '_2, '_3> = ();
    type Context<'_0, '_1, '_2, '_3> =
        MakeContext<(Select<'_0, '_1, '_2, '_3, Mutable, __counter>,)>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl CxFnMut<()> for plus_one {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl CxFn<()> for plus_one {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (): (),
        mut cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let counter = cx.extract_mut::<__counter>();
        *counter += 1;
    }
}

#[derive(Default)]
struct minus_one;

impl CxFnOnce<()> for minus_one {
    type Output<'_0, '_1, '_2, '_3> = ();
    type Context<'_0, '_1, '_2, '_3> =
        MakeContext<(Select<'_0, '_1, '_2, '_3, Mutable, __counter>,)>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl CxFnMut<()> for minus_one {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl CxFn<()> for minus_one {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (): (),
        mut cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let counter = cx.extract_mut::<__counter>();
        *counter -= 1;
    }
}
