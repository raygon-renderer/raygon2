use thermite::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
#[repr(u8)]
pub enum CompareMode {
    LessThan,
    LessThanEqual,
    Equal,
    ApproxEqual,
    GreaterThan,
    GreaterThanEqual,
}

impl CompareMode {
    const EPSILON: f32 = 1e-5;

    #[inline(always)]
    pub fn compare<S: Simd>(self, a: Vf32<S>, b: Vf32<S>) -> Vf32<S> {
        let eps = Vf32::<S>::splat(CompareMode::EPSILON);

        let res = match self {
            CompareMode::LessThan => a.lt(b),
            CompareMode::LessThanEqual => a.le(b),
            CompareMode::Equal => a.eq(b),
            CompareMode::ApproxEqual => (a - b).abs().le(eps),
            CompareMode::GreaterThan => a.gt(b),
            CompareMode::GreaterThanEqual => a.ge(b),
        };

        res.select(Vf32::<S>::one(), Vf32::<S>::zero())
    }
}
