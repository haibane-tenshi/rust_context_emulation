//! Traits for indexed access into tuples.

mod private {
    pub trait Sealed {}

    impl<const N: usize> Sealed for super::Index<N> {}
}

/// Type-level `usize`.
///
/// Purpose of this type is to avoid manipulating `const`s directly.
/// Currently it requires a pile of nightly features (including incomplete ones).
/// We have no use for number itself, so we can afford to wrap it.
pub struct Index<const N: usize>;

pub trait Indexed: private::Sealed {}
impl<const N: usize> Indexed for Index<N> {}

/// Get tuple value in specified position.
pub trait Get<N: Indexed> {
    type Output;

    fn get(&self) -> &Self::Output;
    fn get_mut(&mut self) -> &mut Self::Output;
}

/// Put value into tuple at specified position.
pub trait Put<T, N: Indexed> {
    /// Resulting tuple type.
    type Output;

    /// Type of previously stored value.
    type Previous;

    fn put(self, t: T) -> (Self::Output, Self::Previous);
}
