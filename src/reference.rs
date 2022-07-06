//! Reference *kind* representation and operations defined on it.

use crate::lifetime::Applicator;
use crate::ops::{Coerce, InverseOp, LimitByOp, UnifyOp};
use crate::option::{None, Some};

pub trait ReferenceKind {
    type Ref<'a, T: ?Sized + 'a>;
}

pub struct Mutable;
pub struct Shared;

impl ReferenceKind for Mutable {
    type Ref<'a, T: ?Sized + 'a> = &'a mut T;
}

impl ReferenceKind for Shared {
    type Ref<'a, T: ?Sized + 'a> = &'a T;
}

impl UnifyOp<Shared> for Shared {
    type Output = Shared;
}

impl UnifyOp<Mutable> for Shared {
    type Output = Mutable;
}

impl UnifyOp<Shared> for Mutable {
    type Output = Mutable;
}

impl UnifyOp<Mutable> for Mutable {
    type Output = Mutable;
}

impl Coerce<Shared> for Shared {
    fn coerce(self) -> Shared {
        Shared
    }
}

impl Coerce<Shared> for Mutable {
    fn coerce(self) -> Shared {
        Shared
    }
}

impl Coerce<Mutable> for Mutable {
    fn coerce(self) -> Mutable {
        Mutable
    }
}

impl LimitByOp<()> for Shared {
    type Output = None;
}

impl LimitByOp<Shared> for Shared {
    type Output = Some<Self>;
}

impl LimitByOp<Mutable> for Shared {
    type Output = Some<Self>;
}

impl LimitByOp<()> for Mutable {
    type Output = None;
}

impl LimitByOp<Shared> for Mutable {
    type Output = Some<Shared>;
}

impl LimitByOp<Mutable> for Mutable {
    type Output = Some<Self>;
}

impl InverseOp for Shared {
    type Output = Shared;
}

impl InverseOp for Mutable {
    type Output = ();
}

impl Applicator for Shared {
    type Apply<'a, T: ?Sized + 'a> = &'a T;
}

impl Applicator for Mutable {
    type Apply<'a, T: ?Sized + 'a> = &'a mut T;
}
