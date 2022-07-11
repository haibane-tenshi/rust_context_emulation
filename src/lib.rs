//! This crate is an experimental emulation of Rust contexts.
//!
//! For high level overview of the approach refer to my blog post.
//!
//! For usage examples refer to examples folder.
//!
//! It is recommended to start with input-lifetime-based emulation.

#![feature(generic_associated_types)]
#![allow(clippy::just_underscores_and_digits)]
#![allow(clippy::missing_safety_doc)]

pub mod context;
pub mod cxfn_trait;
pub mod indexed_tuple;
pub mod lifetime;
pub mod ops;
pub mod option;
pub mod reference;
pub mod tuple4;

use indexed_tuple::Indexed;

/// Trait for marking capability wrapper type.
///
/// Capabilities are expressed as a newtype wrapper around the payload.
///
/// ```ignore
/// capability my_vec: Vec<usize>;
/// ```
///
/// ...is represented as
///
/// ```
/// # #![allow(non_camel_case_types)]
/// # use rust_context_emulation::prelude_input::{Index, Capability};
/// #
/// struct my_vec(Vec<usize>);
///
/// impl Capability for my_vec {
///     type Index = Index<0>;
///     type Inner = Vec<usize>;
///
///     fn as_ref(&self) -> &Self::Inner {
///         &self.0
///     }
///
///     fn as_mut(&mut self) -> &mut Self::Inner {
///         &mut self.0
///     }
/// }
/// ```
///
/// **Note** that every capability in your program must have a unique index.
/// Index denotes the slot inside [`Store`](crate::context::store::Store)
/// where capability will be stored.
/// Colliding indices will result in confusing compiler errors.
pub trait Capability {
    type Index: Indexed;
    type Inner: ?Sized;

    fn as_ref(&self) -> &Self::Inner;
    fn as_mut(&mut self) -> &mut Self::Inner;
}

/// Prelude for GAT-based approach.
pub mod prelude_gat {
    pub use crate::context::store::{EmptyStore, HaveLocalReferenceTo, MakeContext, Push};
    pub use crate::cxfn_trait::gat::{CxFn, CxFnMut, CxFnOnce};
    pub use crate::indexed_tuple::{Index, Indexed};
    pub use crate::lifetime::{Applicator, Select, SelectT};
    pub use crate::ops::{Coerce, LimitBy, Reborrow, Unify};
    pub use crate::reference::{Mutable, Shared};
    pub use crate::Capability;
}

/// Prelude for input-lifetime-based approach.
pub mod prelude_input {
    pub use crate::context::store::{EmptyStore, HaveLocalReferenceTo, MakeContext, Push};
    pub use crate::cxfn_trait::input::{CxFn, CxFnMut, CxFnOnce};
    pub use crate::indexed_tuple::{Index, Indexed};
    pub use crate::lifetime::{Applicator, Select, SelectT};
    pub use crate::ops::{Coerce, LimitBy, Reborrow, Unify};
    pub use crate::reference::{Mutable, Shared};
    pub use crate::Capability;
}

/// Prelude for input-lifetime-based approach.
pub mod prelude_hybrid {
    pub use crate::context::store::{EmptyStore, HaveLocalReferenceTo, MakeContext, Push};
    pub use crate::cxfn_trait::hybrid::{CxFn, CxFnMut, CxFnOnce};
    pub use crate::indexed_tuple::{Index, Indexed};
    pub use crate::lifetime::{Applicator, Select, SelectT};
    pub use crate::ops::{Coerce, LimitBy, Reborrow2, Reborrow2Lifetime, Unify};
    pub use crate::reference::{Mutable, Shared};
    pub use crate::Capability;
}
