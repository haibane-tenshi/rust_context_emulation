//! Illustrate desugaring of function with early-bound capability lifetime.

#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]

use rust_context_emulation::prelude_gat::*;
use std::marker::PhantomData;

fn main() {
    let data_store = __data_store(3_usize);
    let cx = EmptyStore::new().push(&data_store);

    let other_data = 4_usize;
    hide::default().cx_call((&other_data,), cx);
}

struct __data_store(usize);

impl Capability for __data_store {
    type Index = Index<0>;
    type Inner = usize;

    fn as_ref(&self) -> &Self::Inner {
        &self.0
    }

    fn as_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

#[derive(Default)]
struct hide<'data_store>(PhantomData<fn(&'data_store ())>);

impl<'data_store> CxFnOnce<(&'data_store usize,)> for hide<'data_store> {
    type Output<'_0, '_1, '_2, '_3> = ();

    // Desired lifetime is already known, extra lifetimes can just be ignored.
    type Context<'_0, '_1, '_2, '_3> = MakeContext<(&'data_store __data_store,)>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (&'data_store usize,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl<'data_store> CxFnMut<(&'data_store usize,)> for hide<'data_store> {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (&'data_store usize,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl<'data_store> CxFn<(&'data_store usize,)> for hide<'data_store> {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (_,): (&'data_store usize,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let _data_store = cx.extract_ref::<__data_store>();

        // Do something useful?
    }
}

#[derive(Default)]
struct invoke_hide;

impl CxFnOnce<()> for invoke_hide {
    type Output<'_0, '_1, '_2, '_3> = ();
    // Properly satisfying early-bound lifetime with late-bound one is tricky.
    // We need to supply correct lifetime to `hide<'??>` and we don't know which one is which.
    // This is the purpose of hide_builder type.
    type Context<'_0, '_1, '_2, '_3> = <hide_builder<'_0, '_1, '_2, '_3> as CxFnOnce<(
        SelectT<'_0, '_1, '_2, '_3, Shared, usize, __data_store>,
    )>>::Context<'_0, '_1, '_2, '_3>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

// Produce monomorphization of `hide` parametrized with correct capability lifetimes.
//
// We can hack into machinery behind `Select` to produce correct monomorphization.
// `Select` is typically used with `Shared` or `Mutable` which implement `Applicator` -
// but we can use our own type.
// Unfortunately this is where we start to run into limitations of Rust type system.
// GAT in this case is used as a limited form of HKT, but it falls short on more complex examples.
// The only solution I know of is duplicating full `Select`/`Applicator` machinery to specialize it
// for the use case.
type hide_builder<'_0, '_1, '_2, '_3> =
    Select<'_0, '_1, '_2, '_3, hide_apply_data_store, __data_store>;

struct hide_apply_data_store;

impl Applicator for hide_apply_data_store {
    type Apply<'a, T: ?Sized + 'a> = hide<'a>;
}

impl CxFnMut<()> for invoke_hide {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl CxFn<()> for invoke_hide {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (): (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        hide::default().cx_call((&3,), cx)
    }
}
