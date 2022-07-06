//! General purpose context-related operations.

use crate::reference::Mutable;

/// Reborrow emulation trait.
pub trait Reborrow {
    type Output<'s>
    where
        Self: 's;

    fn reborrow(&mut self) -> Self::Output<'_>;
}

/// Coerce to `Other` type.
///
/// Coerce trait by default only supports only one coercion [permitted](https://doc.rust-lang.org/stable/reference/type-coercions.html?highlight=coercion#coercion-types)
/// by Rust language: `&mut T` -> `&T`.
/// Additionally, when invoked on `Store`s it can remove capability from context.
///
/// Technically it is possible to provide support for custom coercions impls,
/// but this is not supported at the moment.
pub trait Coerce<Other> {
    fn coerce(self) -> Other;
}

#[doc(hidden)]
pub trait UnifyOp<T>: Sized {
    type Output;
}

/// Trait permitting unification.
///
/// You shouldn't use this trait directly.
/// [`Unify`] type provides convenient shortcut.
/// The only time when you need this is to produce correct trait bounds
/// for contextual methods with wildcard contexts in traits.
pub trait Unified {
    type Output;
}

impl Unified for () {
    type Output = ();
}

impl<T0> Unified for (T0,) {
    type Output = T0;
}

impl<T0, T1> Unified for (T0, T1)
where
    T0: UnifyOp<T1>,
{
    type Output = <T0 as UnifyOp<T1>>::Output;
}

impl<T0, T1, T2> Unified for (T0, T1, T2)
where
    (T0, T1): Unified,
    Unify<(T0, T1)>: UnifyOp<T2>,
{
    type Output = <Unify<(T0, T1)> as UnifyOp<T2>>::Output;
}

impl<T0, T1, T2, T3> Unified for (T0, T1, T2, T3)
where
    (T0, T1, T2): Unified,
    Unify<(T0, T1, T2)>: UnifyOp<T3>,
{
    type Output = <Unify<(T0, T1, T2)> as UnifyOp<T3>>::Output;
}

/// Unify multiple `Store`s or `Handle`s into a single one.
///
/// Unification and coercion are two inverse operations.
/// The assumption is that it is always possible to undo unification via [`Coerce`] trait,
/// however this is not encoded in unification-related traits.
///
/// ```
/// # #![allow(non_camel_case_types)]
/// # use rust_context_emulation::prelude_input::*;
/// #
/// # struct my_vec(Vec<usize>);
/// #
/// # impl Capability for my_vec {
/// #     type Index = Index<0>;
/// #     type Inner = Vec<usize>;
/// #
/// #     fn as_ref(&self) -> &Self::Inner {
/// #         &self.0
/// #     }
/// #
/// #     fn as_mut(&mut self) -> &mut Self::Inner {
/// #         &mut self.0
/// #     }
/// # }
/// #
/// # struct my_box(Box<usize>);
/// #
/// # impl Capability for my_box {
/// #     type Index = Index<0>;
/// #     type Inner = Box<usize>;
/// #
/// #     fn as_ref(&self) -> &Self::Inner {
/// #         &self.0
/// #     }
/// #
/// #     fn as_mut(&mut self) -> &mut Self::Inner {
/// #         &mut self.0
/// #     }
/// # }
/// #
/// // This is equivalent to `MakeContext<(&'my_vec my_vec, &'my_box mut my_box)>`
/// type UnifiedContext<'my_vec, 'my_box> = Unify<(
///     MakeContext<(&'my_vec my_vec, &'my_box my_box)>,
///     MakeContext<(&'my_box mut my_box,)>,
/// )>;
/// ```
pub type Unify<T> = <T as Unified>::Output;

/// Trait permitting controlled removal of capabilities from context.
///
/// See [`LimitBy`] for details.
///
/// You shouldn't use this trait directly.
/// The only time when you need this is to produce correct trait bounds
/// for contextual methods with wildcard contexts in traits.
pub trait LimitByOp<Permissions> {
    type Output;
}

/// Remove unnecessary capabilities from context.
///
/// This is useful when e.g. you call other function, but satisfy some of its capability
/// requirements locally.
///
/// ```
/// # #![allow(non_camel_case_types)]
/// # use rust_context_emulation::prelude_input::*;
/// #
/// # struct my_vec(Vec<usize>);
/// #
/// # impl Capability for my_vec {
/// #     type Index = Index<0>;
/// #     type Inner = Vec<usize>;
/// #
/// #     fn as_ref(&self) -> &Self::Inner {
/// #         &self.0
/// #     }
/// #
/// #     fn as_mut(&mut self) -> &mut Self::Inner {
/// #         &mut self.0
/// #     }
/// # }
/// #
/// // Equivalent to `EmptyContext`.
/// type LimitedContext<'my_vec> = LimitBy<
///     MakeContext<(&'my_vec my_vec,)>,
///     HaveLocalReferenceTo<(&'my_vec my_vec,)>
/// >;
/// ```
pub type LimitBy<Target, Permissions> = <Target as LimitByOp<Permissions>>::Output;

#[doc(hidden)]
pub trait InverseOp {
    type Output;
}

impl InverseOp for () {
    type Output = Mutable;
}

#[doc(hidden)]
pub trait AsPermissionOp {
    type Output;
}
