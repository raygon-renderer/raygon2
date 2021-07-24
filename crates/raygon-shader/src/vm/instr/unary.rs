use thermite::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
#[repr(u8)]
pub enum UnaryOp {
    Saturate,
    Neg,
    Abs,
    Sqrt,
    Square,
    Sign,
    Ln,
    Sin,
    Cos,
    Tan,
    ArcSin,
    ArcCos,
    ArcTan,
    Trunc,
    Fract,
    Round,
    Floor,
    Ceil,
    ToDegrees,
    ToRadians,
    Invert,
    Heavyside,
}

impl UnaryOp {
    #[inline(always)]
    #[rustfmt::skip]
    pub fn eval<S: Simd>(self, x: Vf32<S>) -> Vf32<S> {
        use std::f32::consts::PI;

        let zero = Vf32::<S>::zero();
        let one = Vf32::<S>::one();

        let res = match self {
            UnaryOp::Saturate => return x.clamp(zero, one),
            UnaryOp::Neg => return -x,
            UnaryOp::Abs => return x.abs(),
            UnaryOp::Sqrt => return x.is_negative().select(zero, x.sqrt()),
            UnaryOp::Square => return x * x,
            UnaryOp::Sign => return x.signum(),
            UnaryOp::Ln => return x.is_negative().select(zero, x.ln()),
            UnaryOp::Sin => x.sin(),
            UnaryOp::Cos => x.cos(),
            UnaryOp::Tan => x.tan(),
            UnaryOp::ArcSin => x.asin(),
            UnaryOp::ArcCos => x.acos(),
            UnaryOp::ArcTan => x.atan(),
            UnaryOp::Trunc => return x.trunc(),
            UnaryOp::Fract => return x.fract(),
            UnaryOp::Round => return x.round(),
            UnaryOp::Floor => return x.floor(),
            UnaryOp::Ceil => return x.ceil(),
            UnaryOp::ToDegrees => return x * Vf32::<S>::splat(180.0 / PI),
            UnaryOp::ToRadians => return x * Vf32::<S>::splat(PI / 180.0),
            UnaryOp::Invert => return one - x,
            UnaryOp::Heavyside => return x.is_negative().select(zero, one) ,
        };

        res.is_normal().select(res, zero)
    }
}
