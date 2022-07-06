#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]

use rust_context_emulation::prelude_input::*;

fn main() {
    let data_store = __data_store(vec![6_usize]);
    let hidden_str = __hidden_str("aliens".to_string());
    let cx = EmptyStore::new().push(&data_store).push(&hidden_str);

    let equal = useful_work::default().cx_call((), cx);
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
struct useful_work;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, ()> for useful_work {
    type Output = bool;
    // Since we want to call two functions, we also need to satisfy context requirements for both.
    // This can be achieved by unifying their contexts.
    // `Unify` takes a tuple of contexts (implemented up to 4 atm) and squashes them together.
    type Context = Unify<(
        <first as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context,
        <hidden_len as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context,
    )>;

    fn cx_call_once(mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, ()> for useful_work {
    fn cx_call_mut(&mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, ()> for useful_work {
    fn cx_call(&self, (): (), mut cx: Self::Context) -> Self::Output {
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

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, ()> for first {
    type Output = Option<SelectT<'_0, '_1, '_2, '_3, Shared, usize, __data_store>>;
    type Context = MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __data_store>,)>;

    fn cx_call_once(mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, ()> for first {
    fn cx_call_mut(&mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, ()> for first {
    fn cx_call(&self, (): (), cx: Self::Context) -> Self::Output {
        let data_store = cx.extract_ref::<__data_store>();

        data_store.first()
    }
}

// Return length of hidden string.
#[derive(Default)]
struct hidden_len;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, ()> for hidden_len {
    type Output = usize;
    type Context = MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __hidden_str>,)>;

    fn cx_call_once(mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, ()> for hidden_len {
    fn cx_call_mut(&mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, ()> for hidden_len {
    fn cx_call(&self, (): (), cx: Self::Context) -> Self::Output {
        let hidden_str = cx.extract_ref::<__hidden_str>();

        hidden_str.len()
    }
}
