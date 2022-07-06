//! GAT-based approach.

pub trait CxFnOnce<Args> {
    type Output<'_0, '_1, '_2, '_3>;
    type Context<'_0, '_1, '_2, '_3>;

    fn cx_call_once<'_0, '_1, '_2, '_3>(
        self,
        args: Args,
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3>;
}

pub trait CxFnMut<Args>: CxFnOnce<Args> {
    fn cx_call_mut<'_0, '_1, '_2, '_3>(
        &mut self,
        args: Args,
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3>;
}

pub trait CxFn<Args>: CxFnMut<Args> {
    fn cx_call<'_0, '_1, '_2, '_3>(
        &self,
        args: Args,
        cx: Self::Context<'_0, '_1, '_2, '_3>,
    ) -> Self::Output<'_0, '_1, '_2, '_3>;
}
