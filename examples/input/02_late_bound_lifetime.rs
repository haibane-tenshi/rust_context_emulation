//! Illustrate desugaring of a function with late-bound capability lifetime.

#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]

use rust_context_emulation::prelude_input::*;

fn main() {
    let mut hidden_str = __hidden_str("treasure".to_string());
    let equals = {
        let cx = EmptyStore::new().push(&hidden_str);
        compare_to::default().cx_call(("treasure",), cx)
    };

    assert!(equals);

    {
        let cx = EmptyStore::new().push(&mut hidden_str);
        append::default().cx_call((), cx);
    }

    assert_eq!(hidden_str.as_ref(), "treasure is stolen!");
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

#[derive(Default)]
struct compare_to;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, (&str,)> for compare_to {
    type Output = bool;

    // At this point we don't know which lifetime corresponds to which capability
    // (it is part of capability definition); we have to use helper `Select` type which
    // will pick correct lifetime for capability and construct the reference.
    // We can't just wing it and use arbitrary one: compiler have to unify lifetimes at the end,
    // so if capability handle is bound by multiple lifetimes,
    // it *will* result in compilation error for any non-trivial case.
    type Context = MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __hidden_str>,)>;

    fn cx_call_once(mut self, args: (&str,), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, (&str,)> for compare_to {
    fn cx_call_mut(&mut self, args: (&str,), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, (&str,)> for compare_to {
    fn cx_call(&self, (s,): (&str,), cx: Self::Context) -> Self::Output {
        let hidden_str = cx.extract_ref::<__hidden_str>();

        hidden_str == s
    }
}

#[derive(Default)]
struct append;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, ()> for append {
    type Output = ();
    type Context = MakeContext<(Select<'_0, '_1, '_2, '_3, Mutable, __hidden_str>,)>;

    fn cx_call_once(mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, ()> for append {
    fn cx_call_mut(&mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, ()> for append {
    fn cx_call(&self, (): (), mut cx: Self::Context) -> Self::Output {
        let hidden_str = cx.extract_mut::<__hidden_str>();
        *hidden_str += " is stolen!";
    }
}
