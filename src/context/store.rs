//! Implementation of `Store` which is a storage of `Handle`s.

use crate::indexed_tuple::{Get, Put};
use crate::ops::*;
use crate::option::{None, Some};
use crate::tuple4::Tuple4;
use crate::Capability;

use crate::context::handle::{HandleMut, HandleRef};

/// Storage for handles.
#[derive(Copy, Clone)]
pub struct Store<Tuple>(Tuple);

impl<T> Store<T> {
    /// Extract shared reference to a capability.
    ///
    /// Using [`ExtractRef`] trait in non-generic code gets verbose,
    /// and this function provides slightly more concise alternative:
    ///
    /// ```
    /// # #![allow(non_camel_case_types)]
    /// # use rust_context_emulation::prelude_input::{Index, Capability};
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
    /// # fn main() {
    /// # use rust_context_emulation::prelude_input::*;
    /// # let my_vec_val = my_vec(Vec::new());
    /// # let cx = EmptyStore::new().push(&my_vec_val);
    /// let my_vec_ref = cx.extract_ref::<my_vec>();
    /// # }
    /// ```
    pub fn extract_ref<'a, C>(&self) -> &'a <C as Capability>::Inner
    where
        C: Capability + ?Sized + 'a,
        Self: ExtractRef<'a, C>,
    {
        ExtractRef::extract_ref(self).as_ref()
    }

    /// Extract mutable reference to a capability.
    ///
    /// Using [`ExtractMut`] trait in non-generic code gets verbose,
    /// and this function provides slightly more concise alternative:
    ///
    /// ```
    /// # #![allow(non_camel_case_types)]
    /// # use rust_context_emulation::prelude_input::{Index, Capability};
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
    /// # fn main() {
    /// # use rust_context_emulation::prelude_input::*;
    /// # let mut my_vec_val = my_vec(Vec::new());
    /// # let mut cx = EmptyStore::new().push(&mut my_vec_val);
    /// let my_vec_ref = cx.extract_mut::<my_vec>();
    /// # }
    /// ```
    pub fn extract_mut<'a, 's, C>(&'s mut self) -> &'s mut <C as Capability>::Inner
    where
        'a: 's,
        C: Capability + ?Sized + 'a,
        Self: ExtractMut<'a, C>,
    {
        ExtractMut::extract_mut(self).as_mut()
    }
}

/// Empty context type.
pub type EmptyStore = Store<Tuple4<None, None, None, None>>;

impl EmptyStore {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Store(Tuple4(None, None, None, None))
    }
}

impl<Tuple> Reborrow for Store<Tuple>
where
    Tuple: Reborrow,
{
    type Output<'s> = Store<<Tuple as Reborrow>::Output<'s>> where Self: 's;

    fn reborrow(&mut self) -> Self::Output<'_> {
        Store(self.0.reborrow())
    }
}

impl<'s, Tuple> Reborrow2Lifetime<'s> for Store<Tuple>
where
    Tuple: Reborrow2Lifetime<'s>,
{
    type Output = Store<<Tuple as Reborrow2Lifetime<'s>>::Output>;
}

impl<Tuple> Reborrow2 for Store<Tuple>
where
    Tuple: Reborrow2,
{
    fn reborrow(&mut self) -> <Self as Reborrow2Lifetime<'_>>::Output {
        Store(self.0.reborrow())
    }
}

/// Extract shared reference to a capability.
///
/// **Note** that resulting reference is not bound to lifetime of `self`.
/// This is OK since we are working with shared references.
pub trait ExtractRef<'a, T>
where
    T: ?Sized + 'a,
{
    fn extract_ref(&self) -> &'a T;
}

impl<'a, C, Tuple> ExtractRef<'a, C> for Store<Tuple>
where
    C: Capability + ?Sized + 'a,
    Tuple: Get<<C as Capability>::Index, Output = Some<HandleRef<'a, C>>>,
{
    fn extract_ref(&self) -> &'a C {
        self.0.get().0.as_ref()
    }
}

/// Extract mutable reference to a capability.
///
/// **Note** that resulting reference assumes lifetime of `self` and discards original lifetime.
/// Unfortunately this is required to establish uniqueness of mutable reference.
pub trait ExtractMut<'a, T>
where
    T: ?Sized + 'a,
{
    fn extract_mut<'s>(&'s mut self) -> &'s mut T
    where
        'a: 's;
}

impl<'a, C, Tuple> ExtractMut<'a, C> for Store<Tuple>
where
    C: Capability + ?Sized + 'a,
    Tuple: Get<<C as Capability>::Index, Output = Some<HandleMut<'a, C>>>,
{
    fn extract_mut<'s>(&'s mut self) -> &'s mut C
    where
        'a: 's,
    {
        self.0.get_mut().0.as_mut()
    }
}

/// Insert reference to a capability into `Store`.
///
/// It is expected that push will overwrite previously stored value.
pub trait Push<CRef> {
    type Output;

    fn push(self, t: CRef) -> Self::Output;
}

impl<'a, C, Tuple> Push<&'a C> for Store<Tuple>
where
    C: Capability + ?Sized + 'a,
    Tuple: Put<Some<HandleRef<'a, C>>, <C as Capability>::Index>,
{
    type Output = Store<<Tuple as Put<Some<HandleRef<'a, C>>, <C as Capability>::Index>>::Output>;

    fn push(self, t: &'a C) -> Self::Output {
        let (tuple, _) = self.0.put(Some(HandleRef::new(t)));
        Store(tuple)
    }
}

impl<'a, C, Tuple> Push<&'a mut C> for Store<Tuple>
where
    C: Capability + ?Sized + 'a,
    Tuple: Put<Some<HandleMut<'a, C>>, <C as Capability>::Index>,
{
    type Output = Store<<Tuple as Put<Some<HandleMut<'a, C>>, <C as Capability>::Index>>::Output>;

    fn push(self, t: &'a mut C) -> Self::Output {
        let (tuple, _) = self.0.put(Some(HandleMut::new(t)));
        Store(tuple)
    }
}

impl<TupleA, TupleB> Coerce<Store<TupleB>> for Store<TupleA>
where
    TupleA: Coerce<TupleB>,
{
    fn coerce(self) -> Store<TupleB> {
        Store(self.0.coerce())
    }
}

impl<TupleA, TupleB> UnifyOp<Store<TupleB>> for Store<TupleA>
where
    TupleA: UnifyOp<TupleB>,
{
    type Output = Store<Unify<(TupleA, TupleB)>>;
}

#[doc(hidden)]
pub trait MakeContextHelper {
    type Output;
}

impl MakeContextHelper for () {
    type Output = EmptyStore;
}

impl<T0> MakeContextHelper for (T0,)
where
    EmptyStore: Push<T0>,
{
    type Output = <EmptyStore as Push<T0>>::Output;
}

impl<T0, T1> MakeContextHelper for (T0, T1)
where
    (T0,): MakeContextHelper,
    (T1,): MakeContextHelper,
    MakeContext<(T0,)>: UnifyOp<MakeContext<(T1,)>>,
{
    type Output = Unify<(MakeContext<(T0,)>, MakeContext<(T1,)>)>;
}

impl<T0, T1, T2> MakeContextHelper for (T0, T1, T2)
where
    (T0, T1): MakeContextHelper,
    (T2,): MakeContextHelper,
    MakeContext<(T0, T1)>: UnifyOp<MakeContext<(T2,)>>,
{
    type Output = Unify<(MakeContext<(T0, T1)>, MakeContext<(T2,)>)>;
}

impl<T0, T1, T2, T3> MakeContextHelper for (T0, T1, T2, T3)
where
    (T0, T1, T2): MakeContextHelper,
    (T3,): MakeContextHelper,
    MakeContext<(T0, T1, T2)>: UnifyOp<MakeContext<(T3,)>>,
{
    type Output = Unify<(MakeContext<(T0, T1, T2)>, MakeContext<(T3,)>)>;
}

/// Construct `Store` type holding specified capabilities.
///
/// `T` is expected to be a tuple of references.
/// Currently is implemented only up to size of 4.
/// Behavior is unspecified when same capability is included multiple times.
/// Use [`Unify`](crate::ops::Unify) for such cases instead.
///
/// ```
/// # #![allow(non_camel_case_types)]
/// # use rust_context_emulation::prelude_input::{Index, Capability};
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
/// # use rust_context_emulation::prelude_input::MakeContext;
/// #
/// type C0 = MakeContext<()>; // Same thing as `EmptyContext`
/// type C1<'my_vec> = MakeContext<(&'my_vec my_vec,)>;
/// ```
pub type MakeContext<T> = <T as MakeContextHelper>::Output;

/// Construct set of "permissions"
/// which can be applied to context to remove unnecessary capabilities.
///
/// `T` is expected to be a tuple of references.
/// Currently is implemented up to size of 4.
/// Behavior is unspecified when same capability is included multiple times.
///
/// ```
/// # #![allow(non_camel_case_types)]
/// # use rust_context_emulation::prelude_input::{Index, Capability};
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
/// # use rust_context_emulation::prelude_input::HaveLocalReferenceTo;
/// #
/// type Permissions<'my_vec> = HaveLocalReferenceTo<(&'my_vec my_vec,)>;
/// ```
///
/// Refer to [`LimitBy`](crate::ops::LimitBy) for usage.
pub type HaveLocalReferenceTo<T> =
    <<MakeContext<T> as AsPermissionOp>::Output as InverseOp>::Output;

impl<Tuple> AsPermissionOp for Store<Tuple>
where
    Tuple: AsPermissionOp,
{
    type Output = Store<<Tuple as AsPermissionOp>::Output>;
}

impl<Tuple> InverseOp for Store<Tuple>
where
    Tuple: InverseOp,
{
    type Output = Store<<Tuple as InverseOp>::Output>;
}

impl<TupleA, TupleB> LimitByOp<Store<TupleB>> for Store<TupleA>
where
    TupleA: LimitByOp<TupleB>,
{
    type Output = Store<LimitBy<TupleA, TupleB>>;
}
