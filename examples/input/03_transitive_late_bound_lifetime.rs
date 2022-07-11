//! Illustrate construction of a function calling other functions
//! which rely on late-bound capability lifetimes.

#![feature(generic_associated_types)]
#![allow(non_camel_case_types)]

use rust_context_emulation::prelude_input::*;

fn main() {
    let data_store = __hidden_str("treasure".to_string());
    let mut cx = EmptyStore::new().push(&data_store);

    let equals = is_aliens::default().cx_call((), cx.reborrow());
    assert!(!equals);

    let equals = is_cake::default().cx_call((), cx.reborrow());
    assert!(!equals);

    let equals = is_pirates::default().cx_call((), cx.reborrow());
    assert!(!equals);

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

// Important: late-bound lifetime in argument is unrelated to capability lifetimes.
impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, (&str,)> for compare_to {
    type Output = bool;
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
struct is_aliens;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, ()> for is_aliens {
    type Output = bool;

    // Things get complicated when other late-bound lifetimes are involved.
    // We need to be careful to provide correct lifetime corresponding to how function is used.
    // In this case we know that `compare_to` is invoked on a static string slice.
    // This works just as well when target lifetime is local to this impl block.
    type Context = <compare_to as CxFnOnce<'_0, '_1, '_2, '_3, (&'static str,)>>::Context;

    fn cx_call_once(mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, ()> for is_aliens {
    fn cx_call_mut(&mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, ()> for is_aliens {
    fn cx_call(&self, (): (), cx: Self::Context) -> Self::Output {
        compare_to::default().cx_call(("aliens",), cx)
    }
}

#[derive(Default)]
struct is_cake;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, ()> for is_cake {
    type Output = bool;

    // However, here `compare_to` is invoked with reference to a local string.
    // There is no way for us to get hold of its lifetime `'local` at this point.
    // By current rulings around late-bound lifetimes there are only two possibilities:
    // * `'local` is entirely unrelated to capability lifetimes
    // * `'local` is identical to a specific capability lifetime
    // `compare_to` clearly belongs to the first camp.
    // This means it is sufficient to supply *any* lifetime to construct correct context.
    type Context = <compare_to as CxFnOnce<'_0, '_1, '_2, '_3, (&'static str,)>>::Context;

    fn cx_call_once(mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, ()> for is_cake {
    fn cx_call_mut(&mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, ()> for is_cake {
    fn cx_call(&self, (): (), cx: Self::Context) -> Self::Output {
        let cake = "cake".to_string();

        compare_to::default().cx_call((&cake,), cx)
    }
}

#[derive(Default)]
struct compare_with;

// Important: late-bound lifetime in argument is coupled to a capability lifetime.
impl<'_0, '_1, '_2, '_3>
    CxFnOnce<'_0, '_1, '_2, '_3, (SelectT<'_0, '_1, '_2, '_3, Shared, str, __hidden_str>,)>
    for compare_with
{
    type Output = bool;
    type Context = MakeContext<(Select<'_0, '_1, '_2, '_3, Shared, __hidden_str>,)>;

    fn cx_call_once(
        mut self,
        args: (SelectT<'_0, '_1, '_2, '_3, Shared, str, __hidden_str>,),
        cx: Self::Context,
    ) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3>
    CxFnMut<'_0, '_1, '_2, '_3, (SelectT<'_0, '_1, '_2, '_3, Shared, str, __hidden_str>,)>
    for compare_with
{
    fn cx_call_mut(
        &mut self,
        args: (SelectT<'_0, '_1, '_2, '_3, Shared, str, __hidden_str>,),
        cx: Self::Context,
    ) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3>
    CxFn<'_0, '_1, '_2, '_3, (SelectT<'_0, '_1, '_2, '_3, Shared, str, __hidden_str>,)>
    for compare_with
{
    fn cx_call(
        &self,
        (s,): (SelectT<'_0, '_1, '_2, '_3, Shared, str, __hidden_str>,),
        cx: Self::Context,
    ) -> Self::Output {
        let hidden_str = cx.extract_ref::<__hidden_str>();

        hidden_str == s
    }
}

#[derive(Default)]
struct is_pirates;

impl<'_0, '_1, '_2, '_3> CxFnOnce<'_0, '_1, '_2, '_3, ()> for is_pirates {
    type Output = bool;

    // We need to make sure to pick correct lifetime for &'_ str in CxFnOnce trait.
    //
    // Note how this is different from the previous situation.
    // Even though we invoke it on &'static str,
    // there is still lifetime dependency that needs to be satisfied.
    type Context = <compare_with as CxFnOnce<
        '_0,
        '_1,
        '_2,
        '_3,
        (SelectT<'_0, '_1, '_2, '_3, Shared, str, __hidden_str>,),
    >>::Context;

    fn cx_call_once(mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFnMut<'_0, '_1, '_2, '_3, ()> for is_pirates {
    fn cx_call_mut(&mut self, args: (), cx: Self::Context) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3> CxFn<'_0, '_1, '_2, '_3, ()> for is_pirates {
    fn cx_call(&self, (): (), cx: Self::Context) -> Self::Output {
        compare_with::default().cx_call(("pirates",), cx)
    }
}

#[derive(Default)]
struct is_other;

// When we pass the argument into `compare_with`
// it enforces lifetime relationship between this lifetime and lifetime of the relevant capability.
// There are two ways to resolve it: introduce dependency between those two lifetimes,
// making both early-bound, or propagate the relationship to keep it late-bound.
// This function chooses the latter.
// This is different from GAT variant because there we first set `str`'s lifetime,
// but then it gets immediately overriden by parameters on `Context`.
// We have to be more honest.
impl<'_0, '_1, '_2, '_3>
    CxFnOnce<'_0, '_1, '_2, '_3, (SelectT<'_0, '_1, '_2, '_3, Shared, str, __hidden_str>,)>
    for is_other
{
    type Output = Option<bool>;

    type Context = <compare_with as CxFnOnce<
        '_0,
        '_1,
        '_2,
        '_3,
        (SelectT<'_0, '_1, '_2, '_3, Shared, str, __hidden_str>,),
    >>::Context;

    fn cx_call_once(
        mut self,
        args: (SelectT<'_0, '_1, '_2, '_3, Shared, str, __hidden_str>,),
        cx: Self::Context,
    ) -> Self::Output {
        self.cx_call_mut(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3>
    CxFnMut<'_0, '_1, '_2, '_3, (SelectT<'_0, '_1, '_2, '_3, Shared, str, __hidden_str>,)>
    for is_other
{
    fn cx_call_mut(
        &mut self,
        args: (SelectT<'_0, '_1, '_2, '_3, Shared, str, __hidden_str>,),
        cx: Self::Context,
    ) -> Self::Output {
        self.cx_call(args, cx)
    }
}

impl<'_0, '_1, '_2, '_3>
    CxFn<'_0, '_1, '_2, '_3, (SelectT<'_0, '_1, '_2, '_3, Shared, str, __hidden_str>,)>
    for is_other
{
    fn cx_call(
        &self,
        (s,): (SelectT<'_0, '_1, '_2, '_3, Shared, str, __hidden_str>,),
        cx: Self::Context,
    ) -> Self::Output {
        (!s.is_empty()).then(|| compare_with::default().cx_call((s,), cx))
    }
}
