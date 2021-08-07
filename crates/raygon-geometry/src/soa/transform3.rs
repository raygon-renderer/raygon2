use thermite::*;

use std::ops::*;

use crate::math::soa::*;

use super::{Matrix4, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Transform3<S: Simd> {
    pub forward: Matrix4<S>,
    pub inverse: Matrix4<S>,
}

impl<S: Simd> Deref for Transform3<S> {
    type Target = Matrix4<S>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.forward
    }
}

#[dispatch]
impl<S: Simd> Transform3<S> {
    #[inline(always)]
    pub fn identity() -> Self {
        Transform3 {
            forward: Matrix4::identity(),
            inverse: Matrix4::identity(),
        }
    }

    #[inline]
    pub fn from_forward(forward: Matrix4<S>) -> (Self, Mask<S, Vf32<S>>) {
        let (inverse, mask) = forward.inverse();
        (Transform3 { forward, inverse }, mask)
    }

    #[inline]
    pub fn scale(axis: &Vector3<S>) -> (Self, Mask<S, Vf32<S>>) {
        let one = Vf32::<S>::one();
        let zero = Vf32::<S>::zero();

        let flattened = axis.x * axis.y * axis.z;
        let mask = flattened.eq(zero);

        let forward = Matrix4([
            [axis.x, zero, zero, zero],
            [zero, axis.y, zero, zero],
            [zero, zero, axis.z, zero],
            [zero, zero, zero, one],
        ]);

        let inverse = Matrix4([
            [one / axis.x, zero, zero, zero],
            [zero, one / axis.y, zero, zero],
            [zero, zero, one / axis.z, zero],
            [zero, zero, zero, one],
        ]);

        (Transform3 { forward, inverse }, mask)
    }

    #[inline]
    pub fn rotate(axis_angle: Vector3<S>) -> Self {
        let one = Vf32::<S>::one();
        let zero = Vf32::<S>::zero();

        let (axis, angle, mask) = axis_angle.normalize_len_mask();

        let (s, c) = angle.sin_cos();

        let Vector3 { mut x, mut y, mut z } = axis;

        // set axis to +X if no axis was found. The angle is zero anyway
        x = mask.select(x, one);
        y = mask.select(y, zero);
        z = mask.select(z, zero);

        // precompute these
        let cyy = c.nmul_add(y, y);
        let czz = c.nmul_add(z, z);
        let xs = x * s;
        let ys = y * s;
        let zs = z * s;

        //let m00 = x * x + (1.0 - x * x) * cos_theta;
        let m00 = sum_of_products::<S>(x, x, x.nmul_add(x, one), c);

        //let m01 = x * (y - y * cos_theta) - z * sin_theta;
        let m01 = difference_of_products2::<S>(x, cyy, z, s, zs);

        //let m02 = x * (z - z * cos_theta) + y * sin_theta;
        let m02 = sum_of_products2::<S>(x, czz, y, s, ys);

        //let m10 = x * (y - y * cos_theta) + z * sin_theta;
        let m10 = sum_of_products2::<S>(x, cyy, z, s, zs);

        //let m11 = y * y + (1.0 - y * y) * cos_theta;
        let m11 = sum_of_products::<S>(y, y, y.nmul_add(y, one), c);

        //let m12 = y * (z - z * cos_theta) - x * sin_theta;
        let m12 = difference_of_products2::<S>(y, czz, x, s, xs);

        //let m20 = x * (z - z * cos_theta) - y * sin_theta;
        let m20 = difference_of_products2::<S>(x, czz, y, s, ys);

        //let m21 = y * (z - z * cos_theta) + x * sin_theta;
        let m21 = sum_of_products2::<S>(y, czz, x, s, xs);

        //let m22 = z * z + (1.0 - z * z) * cos_theta;
        let m22 = sum_of_products::<S>(z, z, z.nmul_add(z, one), c);

        let forward = Matrix4([
            [m00, m01, m02, zero],
            [m10, m11, m12, zero],
            [m20, m21, m22, zero],
            [zero, zero, zero, one],
        ]);

        Transform3 {
            forward,
            inverse: forward.transpose(),
        }
    }
}
