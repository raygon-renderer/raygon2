use std::ops::*;

use thermite::*;

use crate::math::soa::*;

#[derive(Debug, Clone, Copy)]
pub struct Vector3<S: Simd> {
    pub x: Vf32<S>,
    pub y: Vf32<S>,
    pub z: Vf32<S>,
}

impl<S: Simd> Vector3<S> {
    /// Creates a vector where all dimensions are the same value
    #[inline(always)]
    pub fn diag(xyz: Vf32<S>) -> Self {
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
    pub fn dot(&self, other: &Self) -> Vf32<S> {
        self.x.mul_add(other.x, self.y.mul_add(other.y, self.z * other.z))
        //self.x.mul_add(other.x, sum_of_products::<S>(self.y, other.y, self.z, other.z))
    }

    #[inline(always)]
    pub fn norm_squared(&self) -> Vf32<S> {
        self.dot(self)
    }

    #[inline(always)]
    pub fn norm(&self) -> Vf32<S> {
        let ns = self.norm_squared();
        ns * ns.invsqrt() // x / sqrt(x) = sqrt(x)
    }

    #[inline(always)]
    pub fn normalize(self) -> Self {
        self.normalize_len_mask().0
    }

    #[inline(always)]
    pub fn normalize_len(self) -> (Self, Vf32<S>) {
        let (v, norm, ..) = self.normalize_len_mask();
        (v, norm)
    }

    /// Returns the normalized vector, the norm before normalization, and a mask
    /// signifying if the vector length was zero before normalization. Since norm/dimenions
    /// may just be NaN after, this is faster than checking then.
    #[inline(always)]
    pub fn normalize_len_mask(self) -> (Self, Vf32<S>, Mask<S, Vf32<S>>) {
        let norm_squared = self.norm_squared();
        let inv_norm = norm_squared.invsqrt();
        let norm = inv_norm * norm_squared; // x/sqrt(x) = sqrt(x)
        (self * Self::diag(inv_norm), norm, norm_squared.eq(Vf32::<S>::zero()))
    }

    #[inline(always)]
    pub fn cross(&self, other: &Self) -> Self {
        Vector3 {
            x: difference_of_products::<S>(self.y, other.z, self.z, other.y),
            y: difference_of_products::<S>(self.z, other.x, self.x, other.z),
            z: difference_of_products::<S>(self.x, other.y, self.y, other.x),
        }
    }

    #[inline(always)]
    pub fn face_forward(self, v: &Self) -> Self {
        self * Self::diag(self.dot(v).signum())
    }
}

impl<S: Simd> Neg for Vector3<S> {
    type Output = Self;

    #[inline(always)]
    fn neg(mut self) -> Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
}

impl<S: Simd> Mul<Vector3<S>> for Vector3<S> {
    type Output = Self;

    #[inline(always)]
    fn mul(mut self, rhs: Self) -> Self {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self
    }
}
