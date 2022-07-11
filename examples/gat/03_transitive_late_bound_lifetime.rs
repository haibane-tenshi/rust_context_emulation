//! Illustrate construction of a function calling other functions
//! which rely on late-bound capability lifetimes.

#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]

use rust_context_emulation::prelude_gat::*;

fn main() {
    let data_store = __hidden_str("treasure".to_string());
    let mut cx = EmptyStore::new().push(&data_store);

    let equals = is_aliens::default().cx_call((), cx.reborrow());
    assert!(!equals);

    let equals = is_cake::default().cx_call((), cx.reborrow());
    assert!(!equals);

    let equals = is_cake::default().cx_call((), cx.reborrow());
    assert!(equals);

    let equals = compare_to::default().cx_call(("treasure",), cx.reborrow());
    assert!(equals);
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

// Important: late-bound lifetime in trait is unrelated to capability lifetimes.
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

#[derive(Default)]
struct is_aliens;

impl CxFnOnce<()> for is_aliens {
    type Output<'_0, '_1, '_2, '_3> = bool;

    // Things get complicated when other late-bound lifetimes are involved.
    // We need to be careful to provide correct lifetime corresponding to how function is used.
    // In this case we know that `compare_to` is invoked on a static string slice.
    // This works just as well when target lifetime is local to this impl block.
    type Context<'_0, '_1, '_2, '_3> =
        <compare_to as CxFnOnce<(&'static str,)>>::Context<'_0, '_1, '_2, '_3>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl CxFnMut<()> for is_aliens {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl CxFn<()> for is_aliens {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (): (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        compare_to::default().cx_call(("aliens",), cx)
    }
}

#[derive(Default)]
struct is_cake;

impl CxFnOnce<()> for is_cake {
    type Output<'_0, '_1, '_2, '_3> = bool;

    // However, here `compare_to` is invoked with reference to a local string.
    // There is no way for us to get hold of its lifetime 'a at this point.
    // By current rulings around late-bound lifetimes there are only two possibilities:
    // * 'a is entirely unrelated to capability lifetimes
    // * 'a is identical to a specific capability lifetime
    // `compare_to` clearly belongs to the first camp.
    // This means it is sufficient to supply *any* lifetime to construct correct context.
    type Context<'_0, '_1, '_2, '_3> =
        <compare_to as CxFnOnce<(&'static str,)>>::Context<'_0, '_1, '_2, '_3>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl CxFnMut<()> for is_cake {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl CxFn<()> for is_cake {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (): (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        let cake = "cake".to_string();

        compare_to::default().cx_call((&cake,), cx)
    }
}

#[derive(Default)]
struct compare_with;

// Important: late-bound lifetime in trait is coupled to a capability lifetime.
impl<'s> CxFnOnce<(&'s str,)> for compare_with {
    type Output<'_0, '_1, '_2, '_3> = bool;
    type Context<'_0, '_1, '_2, '_3> = MakeContext<(&'s __hidden_str,)>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (&'s str,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl<'s> CxFnMut<(&'s str,)> for compare_with {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (&'s str,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl<'s> CxFn<(&'s str,)> for compare_with {
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
struct is_pirates;

impl CxFnOnce<()> for is_pirates {
    type Output<'_0, '_1, '_2, '_3> = bool;

    // We need to make sure to pick correct lifetime for &'_ str in CxFnOnce trait.
    //
    // Note how this is different from the previous situation.
    // Even though we invoke it on &'static str,
    // there is still lifetime dependency that needs to be satisfied.
    type Context<'_0, '_1, '_2, '_3> = <compare_with as CxFnOnce<(
        SelectT<'_0, '_1, '_2, '_3, Shared, str, __hidden_str>,
    )>>::Context<'_0, '_1, '_2, '_3>;
    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl CxFnMut<()> for is_pirates {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl CxFn<()> for is_pirates {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (): (),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        compare_with::default().cx_call(("pirates",), cx)
    }
}

#[derive(Default)]
struct is_other;

impl<'a> CxFnOnce<(&'a str,)> for is_other {
    type Output<'_0, '_1, '_2, '_3> = Option<bool>;

    // But if the target lifetime is part of impl block, our life is this much easier.
    type Context<'_0, '_1, '_2, '_3> =
        <compare_with as CxFnOnce<(&'a str,)>>::Context<'_0, '_1, '_2, '_3>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        mut self,
        args: (&'a str,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call_mut(args, cx)
    }
}

impl<'a> CxFnMut<(&'a str,)> for is_other {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: (&'a str,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        self.cx_call(args, cx)
    }
}

impl<'a> CxFn<(&'a str,)> for is_other {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        (s,): (&'a str,),
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3> {
        (!s.is_empty()).then(|| compare_with::default().cx_call((s,), cx))
    }
}
