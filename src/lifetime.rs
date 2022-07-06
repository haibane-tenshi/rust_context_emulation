//! Lifetime manipulation utilities.

use crate::indexed_tuple::{Index, Indexed};
use crate::Capability;
use std::marker::PhantomData;

/// Apply lifetime to a type.
///
/// This is a limited form of type constructor.
/// It is good enough for in-library use,
/// but you will likely have to reimplement it for use outside.
pub trait Applicator {
    type Apply<'a, T: ?Sized + 'a>;
}

#[doc(hidden)]
pub trait Choose<R, T, I>
where
    R: Applicator,
    T: ?Sized,
    I: Indexed,
{
    type Output;
}

#[doc(hidden)]
pub struct LifetimeStorage<'_0, '_1, '_2, '_3>(
    PhantomData<&'_0 ()>,
    PhantomData<&'_1 ()>,
    PhantomData<&'_2 ()>,
    PhantomData<&'_3 ()>,
);

impl<'_0, '_1, '_2, '_3, R, T> Choose<R, T, Index<0>> for LifetimeStorage<'_0, '_1, '_2, '_3>
where
    R: Applicator,
    T: ?Sized + '_0,
{
    type Output = <R as Applicator>::Apply<'_0, T>;
}
impl<'_0, '_1, '_2, '_3, R, T> Choose<R, T, Index<1>> for LifetimeStorage<'_0, '_1, '_2, '_3>
where
    R: Applicator,
    T: ?Sized + '_1,
{
    type Output = <R as Applicator>::Apply<'_1, T>;
}
impl<'_0, '_1, '_2, '_3, R, T> Choose<R, T, Index<2>> for LifetimeStorage<'_0, '_1, '_2, '_3>
where
    R: Applicator,
    T: ?Sized + '_2,
{
    type Output = <R as Applicator>::Apply<'_2, T>;
}
impl<'_0, '_1, '_2, '_3, R, T> Choose<R, T, Index<3>> for LifetimeStorage<'_0, '_1, '_2, '_3>
where
    R: Applicator,
    T: ?Sized + '_3,
{
    type Output = <R as Applicator>::Apply<'_3, T>;
}

#[doc(hidden)]
pub trait Selection<R, C, T>
where
    R: Applicator,
    C: Capability + ?Sized,
    T: ?Sized,
{
    type Output;
}

impl<'_0, '_1, '_2, '_3, R, C, T> Selection<R, C, T> for LifetimeStorage<'_0, '_1, '_2, '_3>
where
    R: Applicator,
    T: ?Sized,
    C: Capability + ?Sized,
    Self: Choose<R, T, <C as Capability>::Index>,
{
    type Output = <Self as Choose<R, T, <C as Capability>::Index>>::Output;
}

/// Select appropriate lifetime and construct reference to capability with it.
///
/// Requirements: `R: Applicator`, `C: Capability + ?Sized`.
/// Commonly, `R` will be either [`Shared`](crate::reference::Shared) or
/// [`Mutable`](crate::reference::Mutable), but any [`Applicator`] is accepted.
///
/// ```
///# #![allow(non_camel_case_types)]
/// # use rust_context_emulation::prelude_input::*;
/// #
/// struct my_vec(Vec<usize>);
///
/// impl Capability for my_vec {
///     type Index = Index<0>;
///
///     // ...
/// #     type Inner = Vec<usize>;
/// #
/// #     fn as_ref(&self) -> &Self::Inner {
/// #         &self.0
/// #     }
/// #
/// #     fn as_mut(&mut self) -> &mut Self::Inner {
/// #         &mut self.0
/// #     }
/// }
///
/// // The following invocation is equivalent to `&'_0 my_vec`.
/// type Ref<'_0, '_1, '_2, '_3> = Select<'_0, '_1, '_2, '_3, Shared, my_vec>;
/// ```
///
/// `Select` is used to associate lifetime slot to a specific capability.
/// It is useful in situations where different pieces of code need to agree on this fact,
/// e.g. between trait implementors and users.
pub type Select<'_0, '_1, '_2, '_3, R, C> =
    <LifetimeStorage<'_0, '_1, '_2, '_3> as Selection<R, C, C>>::Output;

/// Select appropriate lifetime and construct reference to `T` with it.
///
/// Requirements: `R: Applicator`, `C: Capability + ?Sized`.
/// Commonly, `R` will be either [`Shared`](crate::reference::Shared) or
/// [`Mutable`](crate::reference::Mutable), but any [`Applicator`] is accepted.
///
/// ```
///# #![allow(non_camel_case_types)]
/// # use rust_context_emulation::prelude_input::*;
/// #
/// struct my_vec(Vec<usize>);
///
/// impl Capability for my_vec {
///     type Index = Index<0>;
///
///     // ...
/// #     type Inner = Vec<usize>;
/// #
/// #     fn as_ref(&self) -> &Self::Inner {
/// #         &self.0
/// #     }
/// #
/// #     fn as_mut(&mut self) -> &mut Self::Inner {
/// #         &mut self.0
/// #     }
/// }
///
/// // The following invocation is equivalent to `&'_0 mut str`.
/// type Ref<'_0, '_1, '_2, '_3> = SelectT<'_0, '_1, '_2, '_3, Mutable, str, my_vec>;
/// ```
pub type SelectT<'_0, '_1, '_2, '_3, R, T, C> =
    <LifetimeStorage<'_0, '_1, '_2, '_3> as Selection<R, C, T>>::Output;
