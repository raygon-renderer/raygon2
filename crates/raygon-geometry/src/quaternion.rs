use std::ops::*;

use thermite::*;

use crate::math::*;

use super::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Quaternion<S: Simd> {
    pub x: Vf32<S>,
    pub y: Vf32<S>,
    pub z: Vf32<S>,
    pub w: Vf32<S>,
}

#[dispatch]
impl<S: Simd> Quaternion<S> {
    #[inline(always)]
    pub fn identity() -> Self {
        let zero = Vf32::<S>::zero();
        let one = Vf32::<S>::one();
        Quaternion {
            x: zero,
            y: zero,
            z: zero,
            w: one,
        }
    }

    #[inline(always)]
    pub fn dot(&self, other: &Self) -> Vf32<S> {
        sum_of_products::<S>(self.x, other.x, self.y, other.y) + sum_of_products::<S>(self.z, other.z, self.w, other.w)
    }

    #[inline(always)]
    pub fn normalize(mut self) -> Self {
        let inv_norm = self.dot(&self).invsqrt();
        self.x *= inv_norm;
        self.y *= inv_norm;
        self.z *= inv_norm;
        self.w *= inv_norm;
        self
    }

    #[inline(always)]
    pub fn lerp(mut self, other: &Self, t: Vf32<S>) -> Self {
        self.x = t.lerp(self.x, other.x);
        self.y = t.lerp(self.y, other.y);
        self.z = t.lerp(self.z, other.z);
        self.w = t.lerp(self.w, other.w);
        self
    }

    #[inline]
    pub fn slerp(&self, other: &Self, t: Vf32<S>) -> Self {
        let cos_theta = self.dot(other);

        let theta_small = cos_theta.gt(Vf32::<S>::splat(0.9995));

        let linear = self.lerp(other, t);

        // avoid a lot of work if it's all small
        if theta_small.all() {
            return linear.normalize();
        }

        let theta = cos_theta.clamp(Vf32::<S>::neg_one(), Vf32::<S>::one()).acos();

        let mut qperp: Quaternion<S> = Quaternion {
            x: self.x.nmul_add(cos_theta, other.x),
            y: self.y.nmul_add(cos_theta, other.y),
            z: self.z.nmul_add(cos_theta, other.z),
            w: self.w.nmul_add(cos_theta, other.w),
        };

        // avoid having to normalize linear by shoving it into qperp here
        qperp.x = theta_small.select(linear.x, qperp.x);
        qperp.y = theta_small.select(linear.y, qperp.y);
        qperp.z = theta_small.select(linear.z, qperp.z);
        qperp.w = theta_small.select(linear.w, qperp.w);

        qperp = qperp.normalize();

        let (s, c) = (theta * t).sin_cos();

        let mut res: Quaternion<S> = Quaternion {
            x: sum_of_products::<S>(self.x, c, qperp.x, s),
            y: sum_of_products::<S>(self.y, c, qperp.y, s),
            z: sum_of_products::<S>(self.z, c, qperp.z, s),
            w: sum_of_products::<S>(self.w, c, qperp.w, s),
        };

        // get back normalized from qperp
        res.x = theta_small.select(qperp.x, res.x);
        res.y = theta_small.select(qperp.y, res.y);
        res.z = theta_small.select(qperp.z, res.z);
        res.w = theta_small.select(qperp.w, res.w);

        res
    }
}
