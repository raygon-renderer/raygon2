use std::ops::*;

use thermite::*;

use crate::math::*;

use super::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Point3<S: Simd> {
    pub x: Vf32<S>,
    pub y: Vf32<S>,
    pub z: Vf32<S>,
}

#[dispatch]
impl<S: Simd> Point3<S> {
    #[inline(always)]
    pub fn coords(self) -> Vector3<S> {
        let Point3 { x, y, z } = self;
        Vector3 { x, y, z }
    }

    #[inline(always)]
    pub fn distance_squared(self, other: Self) -> Vf32<S> {
        (self - other).norm_squared()
    }

    #[inline(always)]
    pub fn distance(self, other: Self) -> Vf32<S> {
        (self - other).norm()
    }
}

impl<S: Simd> Sub<Point3<S>> for Point3<S> {
    type Output = Vector3<S>;

    #[inline(always)]
    fn sub(self, rhs: Point3<S>) -> Vector3<S> {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<S: Simd> Add<Vector3<S>> for Point3<S> {
    type Output = Point3<S>;

    #[inline(always)]
    fn add(mut self, rhs: Vector3<S>) -> Point3<S> {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}
