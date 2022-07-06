#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]
#![allow(clippy::drop_ref)]

use rust_context_emulation::prelude_gat::*;

fn main() {
    let mut data_store = __data_store(vec![3]);
    let cx = EmptyStore::new().push(&mut data_store);

    do_work::default().cx_call((), cx);
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

#[derive(Default)]
struct do_work;

impl CxFnOnce<()> for do_work {
    type Output<'_0, '_1, '_2, '_3> = ();
    type Context<'_0, '_1, '_2, '_3> = Unify<(
        // Time for crazy type manipulation.
        // We want to call `compare_to` locally, but we also set `hidden_str` capability locally.
        // Simply unifying with `compare_to`'s context will request the capability,
        // even though we know it is satisfied locally.
        // A better way to handle situation is to remove `hidden_str` from `compare_to` context,
        // but also must keep the rest.
        // The following invocation achieves the goal.
        LimitBy<
            <compare_to as CxFnOnce<(&'static str,)>>::Context<'_0, '_1, '_2, '_3>,
            HaveLocalReferenceTo<(Select<'_0, '_1, '_2, '_3, Mutable, __hidden_str>,)>,
        >,
        MakeContext<(Select<'_0, '_1, '_2, '_3, Mutable, __data_store>,)>,
    )>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl CxFnMut<()> for do_work {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl CxFn<()> for do_work {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (): (),
        mut cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let data_store = cx.extract_mut::<__data_store>();
        data_store.push(42);

        let equals = {
            // Add hidden_str to context.
            let hidden_str = __hidden_str("treasure".to_string());
            let cx = cx.reborrow().push(&hidden_str);

            {
                let cx = cx.coerce();
                compare_to::default().cx_call(("aliens",), cx)
            }
        };

        assert!(!equals);
    }
}

#[derive(Default)]
struct compare_to;

impl CxFnOnce<(&str,)> for compare_to {
    type Output<'_0, '_1, '_2, '_3> = bool;
    type Context<'_0, '_1, '_2, '_3> =
        MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __hidden_str>,)>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (&str,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl CxFnMut<(&str,)> for compare_to {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (&str,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl CxFn<(&str,)> for compare_to {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (s,): (&str,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let hidden_str = cx.extract_ref::<__hidden_str>();

        hidden_str == s
    }
}
