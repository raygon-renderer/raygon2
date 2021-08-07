use std::ops::*;

use crate::math::scalar::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    /// Creates a vector where all dimensions are the same value
    #[inline(always)]
    pub fn diag(xyz: f32) -> Self {
        Vector3 { x: xyz, y: xyz, z: xyz }
    }

    #[inline(always)]
    pub fn abs(&self) -> Self {
        Vector3 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    #[inline(always)]
    pub fn dot(&self, other: &Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    #[inline(always)]
    pub fn norm_squared(&self) -> f32 {
        self.dot(self)
    }

    #[inline(always)]
    pub fn norm(&self) -> f32 {
        self.norm_squared().sqrt()
    }

    #[inline(always)]
    pub fn cross(&self, other: &Self) -> Self {
        Vector3 {
            x: difference_of_products(self.y, other.z, self.z, other.y),
            y: difference_of_products(self.z, other.x, self.x, other.z),
            z: difference_of_products(self.x, other.y, self.y, other.x),
        }
    }

    #[inline(always)]
    pub fn face_forward(self, v: &Self) -> Self {
        self * Self::diag(self.dot(v).signum())
    }
}

impl Neg for Vector3 {
    type Output = Self;

    #[inline(always)]
    fn neg(mut self) -> Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Self;

    #[inline(always)]
    fn mul(mut self, rhs: Self) -> Self {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self
    }
}
