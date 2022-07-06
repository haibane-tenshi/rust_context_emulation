//! Input-lifetime-based approach.

pub trait CxFnOnce<'_0, '_1, '_2, '_3, Args> {
    type Output;
    type Context;

    fn cx_call_once(self, args: Args, cx: Self::Context) -> Self::Output;
}

pub trait CxFnMut<'_0, '_1, '_2, '_3, Args>: CxFnOnce<'_0, '_1, '_2, '_3, Args> {
    fn cx_call_mut(&mut self, args: Args, cx: Self::Context) -> Self::Output;
}

pub trait CxFn<'_0, '_1, '_2, '_3, Args>: CxFnMut<'_0, '_1, '_2, '_3, Args> {
    fn cx_call(&self, args: Args, cx: Self::Context) -> Self::Output;
}
