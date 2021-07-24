use std::ops::*;

use thermite::*;

use crate::math::*;

use crate::{Point3, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Matrix4<S: Simd>(pub [[Vf32<S>; 4]; 4]);

impl<S: Simd> Deref for Matrix4<S> {
    type Target = [[Vf32<S>; 4]; 4];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S: Simd> DerefMut for Matrix4<S> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[dispatch]
impl<S: Simd> Matrix4<S> {
    #[inline(always)]
    pub fn splat(value: Vf32<S>) -> Self {
        Matrix4([[value; 4]; 4])
    }

    #[inline(always)]
    pub fn identity() -> Self {
        let one = Vf32::<S>::one();
        let zero = Vf32::<S>::zero();

        Matrix4([
            [one, zero, zero, zero],
            [zero, one, zero, zero],
            [zero, zero, one, zero],
            [zero, zero, zero, one],
        ])
    }

    #[inline(always)]
    pub fn at(&self, row: usize, col: usize) -> &Vf32<S> {
        &self.0[row][col]
    }

    #[inline(always)]
    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut Vf32<S> {
        &mut self.0[row][col]
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn transpose(self) -> Self {
        let [[m00, m01, m02, m03],
             [m10, m11, m12, m13],
             [m20, m21, m22, m23],
             [m30, m31, m32, m33]] = self.0;

        Self([[m00, m10, m20, m30],
              [m01, m11, m21, m31],
              [m02, m12, m22, m32],
              [m03, m13, m23, m33]])
    }

    #[rustfmt::skip]
    pub fn inverse(&self) -> (Matrix4<S>, Mask<S, Vf32<S>>) {
        let m = self;

        let s0 = difference_of_products::<S>(m[0][0], m[1][1], m[1][0], m[0][1]);
        let s1 = difference_of_products::<S>(m[0][0], m[1][2], m[1][0], m[0][2]);
        let s2 = difference_of_products::<S>(m[0][0], m[1][3], m[1][0], m[0][3]);

        let s3 = difference_of_products::<S>(m[0][1], m[1][2], m[1][1], m[0][2]);
        let s4 = difference_of_products::<S>(m[0][1], m[1][3], m[1][1], m[0][3]);
        let s5 = difference_of_products::<S>(m[0][2], m[1][3], m[1][2], m[0][3]);

        let c0 = difference_of_products::<S>(m[2][0], m[3][1], m[3][0], m[2][1]);
        let c1 = difference_of_products::<S>(m[2][0], m[3][2], m[3][0], m[2][2]);
        let c2 = difference_of_products::<S>(m[2][0], m[3][3], m[3][0], m[2][3]);

        let c3 = difference_of_products::<S>(m[2][1], m[3][2], m[3][1], m[2][2]);
        let c4 = difference_of_products::<S>(m[2][1], m[3][3], m[3][1], m[2][3]);
        let c5 = difference_of_products::<S>(m[2][2], m[3][3], m[3][2], m[2][3]);

        let det = s0.mul_add(c5, s1.nmul_add(c4, s2 * c3)) +
                                  s3.mul_add(c2, s4.nmul_add(c1, s5 * c0));

        //let det = CompensatedFloat::<S>::inner_product((s0, c5, (-s1, c4, (s2, c3, (s3, c2, (s5, c0, (-s4, c1))))))).value();

        let mask = det.eq(Vf32::<S>::zero());

        if mask.all() {
            return (*self, mask);
        }

        let s = Vf32::<S>::one() / det;

        let inverse = Matrix4([
            [
                s * inner_product6::<S>(m[1][1], c5, m[1][3], c3, -m[1][2], c4),
                s * inner_product6::<S>(-m[0][1], c5, m[0][2], c4, -m[0][3], c3),
                s * inner_product6::<S>(m[3][1], s5, m[3][3], s3, -m[3][2], s4),
                s * inner_product6::<S>(-m[2][1], s5, m[2][2], s4, -m[2][3], s3),
            ],
            [
                s * inner_product6::<S>(-m[1][0], c5, m[1][2], c2, -m[1][3], c1),
                s * inner_product6::<S>(m[0][0], c5, m[0][3], c1, -m[0][2], c2),
                s * inner_product6::<S>(-m[3][0], s5, m[3][2], s2, -m[3][3], s1),
                s * inner_product6::<S>(m[2][0], s5, m[2][3], s1, -m[2][2], s2),
            ],
            [
                s * inner_product6::<S>(m[1][0], c4, m[1][3], c0, -m[1][1], c2),
                s * inner_product6::<S>(-m[0][0], c4, m[0][1], c2, -m[0][3], c0),
                s * inner_product6::<S>(m[3][0], s4, m[3][3], s0, -m[3][1], s2),
                s * inner_product6::<S>(-m[2][0], s4, m[2][1], s2, -m[2][3], s0),
            ],
            [
                s * inner_product6::<S>(-m[1][0], c3, m[1][1], c1, -m[1][2], c0),
                s * inner_product6::<S>(m[0][0], c3, m[0][2], c0, -m[0][1], c1),
                s * inner_product6::<S>(-m[3][0], s3, m[3][1], s1, -m[3][2], s0),
                s * inner_product6::<S>(m[2][0], s3, m[2][2], s0, -m[2][1], s1),
            ],
        ]);

        (inverse, mask)
    }
}

#[inline(always)]
pub fn inner_product8<S: Simd>(a: Vf32<S>, b: Vf32<S>, c: Vf32<S>, d: Vf32<S>, e: Vf32<S>, f: Vf32<S>, g: Vf32<S>, h: Vf32<S>) -> Vf32<S> {
    // sum_of_products::<S>(a, b, c, d) + sum_of_products::<S>(e, f, g, h);
    // CompensatedFloat::<S>::sum(sum_of_products::<S>(a, b, c, d), sum_of_products::<S>(e, f, g, h)).value();

    // slightly shorter dependency chain here
    return a.mul_add(b, c * d) + e.mul_add(f, g * h);

    //return a.mul_add(b, c.mul_add(d, e.mul_add(f, g * h)));

    //if S::INSTRSET.has_true_fma() {
    //    a.mul_add(b, c.mul_add(d, sum_of_products::<S>(e, f, g, h)))
    //} else {
    //    CompensatedFloat::<S>::inner_product((a, b, (c, d, (e, f, (g, h))))).value()
    //}
}

#[inline(always)]
pub fn inner_product7<S: Simd>(a: Vf32<S>, b: Vf32<S>, c: Vf32<S>, d: Vf32<S>, e: Vf32<S>, f: Vf32<S>, g: Vf32<S>) -> Vf32<S> {
    //return a.mul_add(b, c * d) + e.mul_add(f, g);
    return a.mul_add(b, c.mul_add(d, e.mul_add(f, g)));

    //if S::INSTRSET.has_true_fma() {
    //    a.mul_add(b, c.mul_add(d, e.mul_add(f, g)))
    //} else {
    //    CompensatedFloat::<S>::inner_product((a, b, (c, d, (e, f)))).add(g).value()
    //}
}

#[inline(always)]
pub fn inner_product6<S: Simd>(a: Vf32<S>, b: Vf32<S>, c: Vf32<S>, d: Vf32<S>, e: Vf32<S>, f: Vf32<S>) -> Vf32<S> {
    //return a.mul_add(b, c * d) + (e * f);
    return a.mul_add(b, c.mul_add(d, e * f));

    //if S::INSTRSET.has_true_fma() {
    //    a.mul_add(b, sum_of_products::<S>(c, d, e, f))
    //} else {
    //    CompensatedFloat::<S>::inner_product((a, b, (c, d, (e, f)))).value()
    //}
}

impl<S: Simd> Mul for &Matrix4<S> {
    type Output = Matrix4<S>;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Matrix4<S> {
        let mut res = Matrix4::splat(unsafe { Vf32::<S>::undefined() });

        for i in 0..4 {
            for j in 0..4 {
                res[i][j] = inner_product8::<S>(
                    self[i][0], rhs[0][j], // fmt
                    self[i][1], rhs[1][j], // fmt
                    self[i][2], rhs[2][j], // fmt
                    self[i][3], rhs[3][j], // fmt
                );
            }
        }

        res
    }
}

impl<S: Simd> Mul<Point3<S>> for &Matrix4<S> {
    type Output = Point3<S>;

    #[inline(always)]
    fn mul(self, rhs: Point3<S>) -> Point3<S> {
        let m = self;

        let x = inner_product7::<S>(m[0][0], rhs.x, m[0][1], rhs.y, m[0][2], rhs.x, m[0][3]);
        let y = inner_product7::<S>(m[1][0], rhs.x, m[1][1], rhs.y, m[1][2], rhs.x, m[1][3]);
        let z = inner_product7::<S>(m[2][0], rhs.x, m[2][1], rhs.y, m[2][2], rhs.x, m[2][3]);
        let w = inner_product7::<S>(m[3][0], rhs.x, m[3][1], rhs.y, m[3][2], rhs.x, m[3][3]);

        let mut p = Point3 { x, y, z };

        // avoid slow division
        let wp = w.reciprocal();

        p.x *= wp;
        p.y *= wp;
        p.z *= wp;

        p
    }
}

impl<S: Simd> Mul<Vector3<S>> for &Matrix4<S> {
    type Output = Vector3<S>;

    #[inline(always)]
    fn mul(self, rhs: Vector3<S>) -> Vector3<S> {
        let m = self;

        Vector3 {
            x: inner_product6::<S>(m[0][0], rhs.x, m[0][1], rhs.y, m[0][2], rhs.x),
            y: inner_product6::<S>(m[1][0], rhs.x, m[1][1], rhs.y, m[1][2], rhs.x),
            z: inner_product6::<S>(m[2][0], rhs.x, m[2][1], rhs.y, m[2][2], rhs.x),
        }
    }
}
