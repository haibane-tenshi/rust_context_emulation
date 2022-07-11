//! Illustrate process of unification and coercion when multiple functions are called.

#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]

use rust_context_emulation::prelude_gat::*;

fn main() {
    let data_store = __data_store(vec![6_usize]);
    let hidden_str = __hidden_str("aliens".to_string());
    let cx = EmptyStore::new().push(&data_store).push(&hidden_str);

    let equal = call_two::default().cx_call((), cx);
    assert!(equal);
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

struct __hidden_str(String);

impl Capability for __hidden_str {
    type Index = Index<1>;
    type Inner = String;

    fn as_ref(&self) -> &Self::Inner {
        &self.0
    }

    fn as_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

// Do some questionable things.
#[derive(Default)]
struct call_two;

impl CxFnOnce<()> for call_two {
    type Output<'_0, '_1, '_2, '_3> = bool;
    // Since we want to call two functions, we also need to satisfy context requirements for both.
    // This can be achieved by unifying their contexts.
    // `Unify` takes a tuple of contexts (implemented up to 4 atm) and squashes them together.
    type Context<'_0, '_1, '_2, '_3> = Unify<(
        <first as CxFnOnce<()>>::Context<'_0, '_1, '_2, '_3>,
        <hidden_len as CxFnOnce<()>>::Context<'_0, '_1, '_2, '_3>,
    )>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl CxFnMut<()> for call_two {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl CxFn<()> for call_two {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (): (),
        mut cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let first_item = {
            // However, the call site requires a bit of care.
            // Action is two-fold:
            // 1. We reborrow the context.
            //    It is effectively a "soft-clone" for references.
            //    Contexts are taken by value, so without doing this we would lose local context
            //    and with it ability to call other contextual functions.
            // 2. We coerce the context.
            //    Coercion molds local context into what the other function expects.
            //    Most notably, it can remove unnecessary capabilities and
            //    downgrade mutable references to shared ones.
            let cx = cx.reborrow().coerce();
            first::default().cx_call((), cx)
        };

        let length = {
            let cx = cx.reborrow().coerce();
            hidden_len::default().cx_call((), cx)
        };

        first_item.map(|value| *value == length).unwrap_or_default()
    }
}

// Acquire reference to first element of `data_store`.
#[derive(Default)]
struct first;

impl CxFnOnce<()> for first {
    type Output<'_0, '_1, '_2, '_3> =
        Option<SelectT<'_0, '_1, '_2, '_3, Shared, usize, __data_store>>;

    type Context<'_0, '_1, '_2, '_3> =
        MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __data_store>,)>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl CxFnMut<()> for first {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl CxFn<()> for first {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (): (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let data_store = cx.extract_ref::<__data_store>();

        data_store.first()
    }
}

// Return length of hidden string.
#[derive(Default)]
struct hidden_len;

impl CxFnOnce<()> for hidden_len {
    type Output<'_0, '_1, '_2, '_3> = usize;
    type Context<'_0, '_1, '_2, '_3> =
        MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __hidden_str>,)>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl CxFnMut<()> for hidden_len {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl CxFn<()> for hidden_len {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (): (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let hidden_str = cx.extract_ref::<__hidden_str>();

        hidden_str.len()
    }
}
