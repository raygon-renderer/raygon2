use thermite::*;

pub mod cf;
pub use cf::CompensatedFloat;

pub use products::*;
#[dispatch]
mod products {
    use super::*;

    #[inline(always)]
    pub fn difference_of_products<S: Simd>(a: Vf32<S>, b: Vf32<S>, c: Vf32<S>, d: Vf32<S>) -> Vf32<S> {
        difference_of_products2::<S>(a, b, c, d, c * d)
    }

    #[inline(always)]
    pub fn difference_of_products2<S: Simd>(a: Vf32<S>, b: Vf32<S>, c: Vf32<S>, d: Vf32<S>, cd: Vf32<S>) -> Vf32<S> {
        a.mul_sub(b, cd) + c.nmul_add(d, cd)
    }

    #[inline(always)]
    pub fn sum_of_products<S: Simd>(a: Vf32<S>, b: Vf32<S>, c: Vf32<S>, d: Vf32<S>) -> Vf32<S> {
        sum_of_products2::<S>(a, b, c, d, c * d)
    }

    #[inline(always)]
    pub fn sum_of_products2<S: Simd>(a: Vf32<S>, b: Vf32<S>, c: Vf32<S>, d: Vf32<S>, cd: Vf32<S>) -> Vf32<S> {
        a.mul_add(b, cd) + c.mul_sub(d, cd)
    }
}
