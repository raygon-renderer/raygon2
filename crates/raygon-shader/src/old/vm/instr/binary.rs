use thermite::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
#[repr(u8)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Powf,
    Min,
    Max,
    ArcTan2,
    Hypot,
}

impl BinaryOp {
    #[inline(always)]
    pub fn eval<S: Simd>(self, a: Vf32<S>, b: Vf32<S>) -> Vf32<S> {
        let zero = Vf32::<S>::zero();

        let res = match self {
            BinaryOp::Add => return a + b,
            BinaryOp::Sub => return a - b,
            BinaryOp::Mul => return a * b,
            BinaryOp::Min => return a.min(b),
            BinaryOp::Max => return a.max(b),
            // these require checking for not-normal values
            BinaryOp::Div => a / b,
            BinaryOp::Rem => a % b,
            BinaryOp::Powf => a.powf(b),
            BinaryOp::ArcTan2 => a.atan2(b),
            BinaryOp::Hypot => a.hypot(b),
        };

        res.is_normal().select(res, zero)
    }
}
