#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]
#![allow(clippy::drop_ref)]

use rust_context_emulation::prelude_input::*;

fn main() {
    let mut data_store = __data_store(vec![5]);
    let mut cx = EmptyStore::new().push(&mut data_store);

    {
        let cx = cx.reborrow().coerce();
        access_mutably::default().cx_call((), cx);
    }

    {
        let cx = cx.reborrow().coerce();
        access_immutably::default().cx_call((), cx);
    }
}

struct __data_store(Vec<u32>);

impl Capability for __data_store {
    type Index = Index<0>;
    type Inner = Vec<u32>;

    fn as_ref(&self) -> &Self::Inner {
        &self.0
    }

    fn as_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

#[derive(Default)]
struct access_mutably;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, ()> for access_mutably {
    type Output = ();
    type Context = Unify<(
        <push as CxFnOnce<'_0, '_1, '_2, '_3, (u32,)>>::Context,
        <len as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context,
        MakeContext<(Select<'_0, '_1, '_2, '_3, Mutable, __data_store>,)>, // Those capabilities are required locally
    )>;

    fn cx_call_once(mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, ()> for access_mutably {
    fn cx_call_mut(&mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, ()> for access_mutably {
    fn cx_call(&self, (): (), mut cx: Self::Context) -> Self::Output {
        {
            let cx = cx.reborrow().coerce();
            push::default().cx_call((3,), cx);
        }

        let data_store = cx.extract_mut::<__data_store>();
        let value = data_store.pop().unwrap();
        assert_eq!(value, 3);

        {
            let cx = cx.reborrow().coerce();
            push::default().cx_call((4,), cx);
        }

        // We cannot keep mutable references across function calls.
        // This is just how Rust works.
        // Have to recreate the reference every time we need to access it.
        let data_store = cx.extract_mut::<__data_store>();
        let value = data_store.pop().unwrap();
        assert_eq!(value, 4);
        assert_eq!(*data_store, [5]);
    }
}

#[derive(Default)]
struct access_immutably;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, ()> for access_immutably {
    type Output = ();
    type Context = Unify<(
        <len as CxFnOnce<'_0, '_1, '_2, '_3, ()>>::Context,
        MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __data_store>,)>,
    )>;

    fn cx_call_once(mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, ()> for access_immutably {
    fn cx_call_mut(&mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, ()> for access_immutably {
    fn cx_call(&self, (): (), mut cx: Self::Context) -> Self::Output {
        let data_store = cx.extract_ref::<__data_store>();

        let length = {
            let cx = cx.reborrow().coerce();
            len::default().cx_call((), cx)
        };

        // As a consolation, we *can* keep immutable references across function calls.
        assert_eq!(length, data_store.len())
    }
}

// Push an element into `data_store`.
#[derive(Default)]
struct push;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, (u32,)> for push {
    type Output = ();
    type Context = MakeContext<(Select<'_0, '_1, '_2, '_3, Mutable, __data_store>,)>;

    fn cx_call_once(mut self, args: (u32,), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, (u32,)> for push {
    fn cx_call_mut(&mut self, args: (u32,), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, (u32,)> for push {
    fn cx_call(&self, (value,): (u32,), mut cx: Self::Context) -> Self::Output {
        let data_store = cx.extract_mut::<__data_store>();
        data_store.push(value);
    }
}

#[derive(Default)]
struct len;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, ()> for len {
    type Output = usize;
    type Context = MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __data_store>,)>;

    fn cx_call_once(mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, ()> for len {
    fn cx_call_mut(&mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, ()> for len {
    fn cx_call(&self, _: (), cx: Self::Context) -> Self::Output {
        let data_store = cx.extract_ref::<__data_store>();

        data_store.len()
    }
}
