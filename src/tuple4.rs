//! Implementation of indexation and context ops on 4-tuple.

use super::indexed_tuple::*;
use crate::ops::*;

/// Indexed 4-tuple.
#[derive(Copy, Clone)]
pub struct Tuple4<T0, T1, T2, T3>(pub T0, pub T1, pub T2, pub T3);

impl<T0, T1, T2, T3> Get<Index<0>> for Tuple4<T0, T1, T2, T3> {
    type Output = T0;

    fn get(&self) -> &Self::Output {
        &self.0
    }
    fn get_mut(&mut self) -> &mut Self::Output {
        &mut self.0
    }
}

impl<T0, T1, T2, T3> Get<Index<1>> for Tuple4<T0, T1, T2, T3> {
    type Output = T1;

    fn get(&self) -> &Self::Output {
        &self.1
    }
    fn get_mut(&mut self) -> &mut Self::Output {
        &mut self.1
    }
}

impl<T0, T1, T2, T3> Get<Index<2>> for Tuple4<T0, T1, T2, T3> {
    type Output = T2;

    fn get(&self) -> &Self::Output {
        &self.2
    }
    fn get_mut(&mut self) -> &mut Self::Output {
        &mut self.2
    }
}

impl<T0, T1, T2, T3> Get<Index<3>> for Tuple4<T0, T1, T2, T3> {
    type Output = T3;

    fn get(&self) -> &Self::Output {
        &self.3
    }
    fn get_mut(&mut self) -> &mut Self::Output {
        &mut self.3
    }
}

impl<T, T0, T1, T2, T3> Put<T, Index<0>> for Tuple4<T0, T1, T2, T3> {
    type Output = Tuple4<T, T1, T2, T3>;
    type Previous = T0;

    fn put(self, t: T) -> (Self::Output, Self::Previous) {
        let Tuple4(_0, _1, _2, _3) = self;
        let r = Tuple4(t, _1, _2, _3);

        (r, _0)
    }
}

impl<T, T0, T1, T2, T3> Put<T, Index<1>> for Tuple4<T0, T1, T2, T3> {
    type Output = Tuple4<T0, T, T2, T3>;
    type Previous = T1;

    fn put(self, t: T) -> (Self::Output, Self::Previous) {
        let Tuple4(_0, _1, _2, _3) = self;
        let r = Tuple4(_0, t, _2, _3);

        (r, _1)
    }
}

impl<T, T0, T1, T2, T3> Put<T, Index<2>> for Tuple4<T0, T1, T2, T3> {
    type Output = Tuple4<T0, T1, T, T3>;
    type Previous = T2;

    fn put(self, t: T) -> (Self::Output, Self::Previous) {
        let Tuple4(_0, _1, _2, _3) = self;
        let r = Tuple4(_0, _1, t, _3);

        (r, _2)
    }
}

impl<T, T0, T1, T2, T3> Put<T, Index<3>> for Tuple4<T0, T1, T2, T3> {
    type Output = Tuple4<T0, T1, T2, T>;
    type Previous = T3;

    fn put(self, t: T) -> (Self::Output, Self::Previous) {
        let Tuple4(_0, _1, _2, _3) = self;
        let r = Tuple4(_0, _1, _2, t);

        (r, _3)
    }
}

impl<T0, T1, T2, T3> Reborrow for Tuple4<T0, T1, T2, T3>
where
    T0: Reborrow,
    T1: Reborrow,
    T2: Reborrow,
    T3: Reborrow,
{
    type Output<'s> = Tuple4<
        <T0 as Reborrow>::Output<'s>,
        <T1 as Reborrow>::Output<'s>,
        <T2 as Reborrow>::Output<'s>,
        <T3 as Reborrow>::Output<'s>,
    >
    where Self: 's;

    fn reborrow(&mut self) -> Self::Output<'_> {
        Tuple4(
            self.0.reborrow(),
            self.1.reborrow(),
            self.2.reborrow(),
            self.3.reborrow(),
        )
    }
}

impl<'s, T0, T1, T2, T3> Reborrow2Lifetime<'s> for Tuple4<T0, T1, T2, T3>
where
    T0: Reborrow2Lifetime<'s>,
    T1: Reborrow2Lifetime<'s>,
    T2: Reborrow2Lifetime<'s>,
    T3: Reborrow2Lifetime<'s>,
{
    type Output = Tuple4<
        <T0 as Reborrow2Lifetime<'s>>::Output,
        <T1 as Reborrow2Lifetime<'s>>::Output,
        <T2 as Reborrow2Lifetime<'s>>::Output,
        <T3 as Reborrow2Lifetime<'s>>::Output,
    >;
}

impl<T0, T1, T2, T3> Reborrow2 for Tuple4<T0, T1, T2, T3>
where
    T0: Reborrow2,
    T1: Reborrow2,
    T2: Reborrow2,
    T3: Reborrow2,
{
    fn reborrow(&mut self) -> <Self as Reborrow2Lifetime<'_>>::Output {
        Tuple4(
            self.0.reborrow(),
            self.1.reborrow(),
            self.2.reborrow(),
            self.3.reborrow(),
        )
    }
}

impl<AT0, AT1, AT2, AT3, BT0, BT1, BT2, BT3> UnifyOp<Tuple4<BT0, BT1, BT2, BT3>>
    for Tuple4<AT0, AT1, AT2, AT3>
where
    AT0: UnifyOp<BT0>,
    AT1: UnifyOp<BT1>,
    AT2: UnifyOp<BT2>,
    AT3: UnifyOp<BT3>,
{
    type Output =
        Tuple4<Unify<(AT0, BT0)>, Unify<(AT1, BT1)>, Unify<(AT2, BT2)>, Unify<(AT3, BT3)>>;
}

impl<AT0, AT1, AT2, AT3, BT0, BT1, BT2, BT3> LimitByOp<Tuple4<BT0, BT1, BT2, BT3>>
    for Tuple4<AT0, AT1, AT2, AT3>
where
    AT0: LimitByOp<BT0>,
    AT1: LimitByOp<BT1>,
    AT2: LimitByOp<BT2>,
    AT3: LimitByOp<BT3>,
{
    type Output =
        Tuple4<LimitBy<AT0, BT0>, LimitBy<AT1, BT1>, LimitBy<AT2, BT2>, LimitBy<AT3, BT3>>;
}

impl<AT0, AT1, AT2, AT3, BT0, BT1, BT2, BT3> Coerce<Tuple4<BT0, BT1, BT2, BT3>>
    for Tuple4<AT0, AT1, AT2, AT3>
where
    AT0: Coerce<BT0>,
    AT1: Coerce<BT1>,
    AT2: Coerce<BT2>,
    AT3: Coerce<BT3>,
{
    fn coerce(self) -> Tuple4<BT0, BT1, BT2, BT3> {
        Tuple4(
            self.0.coerce(),
            self.1.coerce(),
            self.2.coerce(),
            self.3.coerce(),
        )
    }
}

impl<T0, T1, T2, T3> AsPermissionOp for Tuple4<T0, T1, T2, T3>
where
    T0: AsPermissionOp,
    T1: AsPermissionOp,
    T2: AsPermissionOp,
    T3: AsPermissionOp,
{
    type Output = Tuple4<
        <T0 as AsPermissionOp>::Output,
        <T1 as AsPermissionOp>::Output,
        <T2 as AsPermissionOp>::Output,
        <T3 as AsPermissionOp>::Output,
    >;
}

impl<T0, T1, T2, T3> InverseOp for Tuple4<T0, T1, T2, T3>
where
    T0: InverseOp,
    T1: InverseOp,
    T2: InverseOp,
    T3: InverseOp,
{
    type Output = Tuple4<
        <T0 as InverseOp>::Output,
        <T1 as InverseOp>::Output,
        <T2 as InverseOp>::Output,
        <T3 as InverseOp>::Output,
    >;
}
