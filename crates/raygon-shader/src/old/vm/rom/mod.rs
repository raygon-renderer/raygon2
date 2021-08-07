use raygon_core::slice::SliceExt;

pub mod curve;
use curve::Curve;

use super::instr::CurveIndex;

#[derive(DeepSizeOf)]
pub struct ROM {
    pub scalar: Vec<f32>,
    pub curves: Vec<Curve>,
}

impl ROM {
    #[inline(always)]
    pub fn get_curve(&self, index: CurveIndex) -> &Curve {
        unsafe { &*self.curves.get_unchecked_debug_checked(index.into()) }
    }
}
