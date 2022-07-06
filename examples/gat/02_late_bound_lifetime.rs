#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]

use rust_context_emulation::prelude_gat::*;

fn main() {
    let data_store = __hidden_str("treasure".to_string());
    let mut cx = EmptyStore::new().push(&data_store);

    let emptied = is_empty::default().cx_call((), cx.reborrow());
    assert!(!emptied);

    let first = first_char::default().cx_call((), cx.reborrow());
    assert_eq!(first, Some("t"));

    let pirates = "pirates".to_string();
    let equals = compare_to::default().cx_call((&pirates,), cx.reborrow());
    assert!(!equals);

    let equals = compare_to::default().cx_call(("treasure",), cx.reborrow());
    assert!(equals);

    let answer = shortest::default().cx_call((&pirates,), cx.reborrow());
    assert_eq!(answer, "pirates");
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
struct is_empty;

impl CxFnOnce<()> for is_empty {
    type Output<'_0, '_1, '_2, '_3> = bool;

    // Late-bound lifetime which is present only in context.
    //
    // At this point we don't know which lifetime corresponds to which capability
    // (it is part of capability definition); we have to use helper `Select` type which
    // will pick correct lifetime for capability and construct the reference.
    // We can't just wing it and use arbitrary one: compiler have to unify lifetimes at the end,
    // so if capability handle is bound by multiple lifetimes,
    // it *will* result in compilation error for any non-trivial case.
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

impl CxFnMut<()> for is_empty {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl CxFn<()> for is_empty {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (): (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let hidden_str = cx.extract_ref::<__hidden_str>();

        hidden_str.is_empty()
    }
}

#[derive(Default)]
struct compare_to;

impl<'s> CxFnOnce<(&'s str,)> for compare_to {
    type Output<'_0, '_1, '_2, '_3> = bool;

    // Late-bound lifetime, shared by argument and capability.
    // Strictly speaking, it is not required for this function and
    // I'm not certain as to how it is practically useful.
    //
    // Since lifetime appears in trait, this is functionally identical to
    // "normal" late-bound lifetime.
    type Context<'_0, '_1, '_2, '_3> = MakeContext<(&'s __hidden_str,)>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (&'s str,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl<'s> CxFnMut<(&'s str,)> for compare_to {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (&'s str,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl<'s> CxFn<(&'s str,)> for compare_to {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (s,): (&'s str,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let hidden_str = cx.extract_ref::<__hidden_str>();

        hidden_str == s
    }
}

#[derive(Default)]
struct first_char;

impl CxFnOnce<()> for first_char {
    // Late-bound lifetime which is shared by Context and Output.
    //
    // SelectT does the same thing as Select, except constructs reference with arbitrary type T.
    // In this case we create a &'_ str reference into capability.
    type Output<'_0, '_1, '_2, '_3> =
        Option<SelectT<'_0, '_1, '_2, '_3, Shared, str, __hidden_str>>;

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

impl CxFnMut<()> for first_char {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl CxFn<()> for first_char {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (): (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let hidden_str = cx.extract_ref::<__hidden_str>();

        (!hidden_str.is_empty()).then(|| &hidden_str[..1])
    }
}

#[derive(Default)]
struct shortest;

impl<'s> CxFnOnce<(&'s str,)> for shortest {
    // Late-bound lifetime, shared by argument, capability and output.
    type Output<'_0, '_1, '_2, '_3> = &'s str;
    type Context<'_0, '_1, '_2, '_3> = MakeContext<(&'s __hidden_str,)>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (&'s str,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl<'s> CxFnMut<(&'s str,)> for shortest {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (&'s str,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl<'s> CxFn<(&'s str,)> for shortest {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (s,): (&'s str,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let hidden_str = cx.extract_ref::<__hidden_str>();

        if hidden_str.len() < s.len() {
            hidden_str
        } else {
            s
        }
    }
}
