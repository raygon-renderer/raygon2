use thermite::*;

use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct CompensatedFloat<S: Simd> {
    pub v: Vf32<S>,
    pub e: Vf32<S>,
}

impl<S: Simd> CompensatedFloat<S> {
    #[inline(always)]
    pub fn zero() -> Self {
        CompensatedFloat {
            v: S::Vf32::zero(),
            e: S::Vf32::zero(),
        }
    }

    #[inline(always)]
    pub fn value(self) -> Vf32<S> {
        self.v + self.e
    }

    #[inline(always)]
    pub fn product(a: Vf32<S>, b: Vf32<S>) -> Self {
        let v = a * b;
        let e = a.mul_sub(b, v);
        CompensatedFloat { v, e }
    }

    #[inline(always)]
    pub fn sum(a: Vf32<S>, b: Vf32<S>) -> Self {
        let v = a + b;
        let d = v - a;
        let e = (a - (v - d)) + (b - d);
        CompensatedFloat { v, e }
    }

    #[inline(always)]
    pub fn inner_product(ip: impl InnerProductHelper<S>) -> Self {
        ip.inner_product()
    }
}

/// Variadic recursive helper
///
/// Direct recursion for the same monomorphized function will deoptimize, so we have to use indirect recursion
pub trait InnerProductHelper<S: Simd> {
    fn inner_product(self) -> CompensatedFloat<S>;
}

impl<S: Simd> InnerProductHelper<S> for (Vf32<S>, Vf32<S>) {
    #[inline(always)]
    fn inner_product(self) -> CompensatedFloat<S> {
        CompensatedFloat::<S>::product(self.0, self.1)
    }
}

impl<S: Simd, REST> InnerProductHelper<S> for (Vf32<S>, Vf32<S>, REST)
where
    REST: InnerProductHelper<S>,
{
    #[inline(always)]
    fn inner_product(self) -> CompensatedFloat<S> {
        let (a, b, rest) = self;
        let tp = rest.inner_product();

        let ab = CompensatedFloat::<S>::product(a, b);
        let mut sum = CompensatedFloat::<S>::sum(ab.v, tp.v);

        sum.e = ab.e + (tp.e + sum.e);
        sum
    }
}

impl<S: Simd> Add<Vf32<S>> for CompensatedFloat<S> {
    type Output = CompensatedFloat<S>;

    #[inline(always)]
    fn add(mut self, rhs: Vf32<S>) -> CompensatedFloat<S> {
        let delta = rhs - self.e;
        let new_sum = self.v + delta;
        self.e = (new_sum - self.v) - delta;
        self.v = new_sum;
        self
    }
}
