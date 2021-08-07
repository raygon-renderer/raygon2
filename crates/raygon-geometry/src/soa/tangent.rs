use std::ops::*;

use thermite::*;

use crate::math::soa::*;

use super::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct TangentFrame<S: Simd> {
    pub normal: Vector3<S>,
    pub tangent: Vector3<S>,
    pub bitangent: Vector3<S>,
}

#[dispatch]
impl<S: Simd> TangentFrame<S> {
    #[inline(always)]
    pub fn new(normal: Vector3<S>) -> Self {
        let one = Vf32::<S>::one();
        // avoid a couple multiplications by xor-ing with the sign bit instead
        let sign_bit = normal.z & Vf32::<S>::neg_zero();
        let sign = one | sign_bit; // copy sign-bit to 1.0

        let a = Vf32::<S>::neg_one() / (sign + normal.z);
        let b = normal.x * normal.y * a;

        TangentFrame {
            normal,
            tangent: Vector3 {
                x: a.mul_add((normal.x * normal.x) ^ sign_bit, one),
                y: sign_bit ^ b,
                z: sign_bit ^ -normal.x,
            },
            bitangent: Vector3 {
                x: b,
                y: a.mul_add(normal.y * normal.y, sign),
                z: -normal.y,
            },
        }
    }

    #[inline(always)]
    pub fn partial(normal: Vector3<S>, tangent: Vector3<S>) -> Self {
        TangentFrame {
            normal,
            bitangent: normal.cross(&tangent).normalize(),
            tangent: tangent.normalize(),
        }
    }

    #[inline(always)]
    pub fn to_world(&self, p: Vector3<S>) -> Vector3<S> {
        let TangentFrame {
            normal: n,
            tangent: t,
            bitangent: b,
        } = self;

        Vector3 {
            x: t.x.mul_add(p.x, sum_of_products::<S>(b.x, p.y, n.x, p.z)),
            y: t.y.mul_add(p.x, sum_of_products::<S>(b.y, p.y, n.y, p.z)),
            z: t.z.mul_add(p.x, sum_of_products::<S>(b.z, p.y, n.z, p.z)),
        }
    }

    #[inline(always)]
    pub fn to_local(&self, p: Vector3<S>) -> Vector3<S> {
        Vector3 {
            x: self.tangent.dot(&p),
            y: self.bitangent.dot(&p),
            z: self.normal.dot(&p),
        }
    }
}
