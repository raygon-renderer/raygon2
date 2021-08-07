use std::ops::*;

use thermite::*;

use crate::math::*;

use super::{Error3, Point3, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Ray3<S: Simd> {
    pub origin: Point3<S>,
    pub direction: Vector3<S>,
    pub tmax: Vf32<S>,
}

pub struct Ray3Error3<S: Simd> {
    pub pos: Error3<S>,
    pub dir: Error3<S>,
}

#[dispatch]
impl<S: Simd> Ray3<S> {
    pub fn at(&self, t: Vf32<S>) -> Point3<S> {
        self.origin + self.direction * Vector3::diag(t)
    }
}
