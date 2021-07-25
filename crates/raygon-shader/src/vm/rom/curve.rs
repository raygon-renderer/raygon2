use thermite::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, DeepSizeOf)]
pub enum InterpolationMode {
    Nearest,
    Linear,
    CubicHermite,
}

#[derive(Debug, Clone, PartialEq, DeepSizeOf)]
pub enum Curve {
    Poly(Vec<f32>),
    LookupTable {
        values: Vec<(f32, f32, f32)>,
        interpolation: InterpolationMode,
    },
}

impl Curve {
    #[inline(always)]
    fn eval_scalar(&self, x: f32) -> f32 {
        match self {
            Curve::LookupTable { values, interpolation } => match raygon_core::lower_bound(values.len(), |idx| values[idx].0 < x) {
                Some(idx) if idx > 0 && idx < values.len() => {
                    let a = unsafe { values.get_unchecked(idx - 1) };
                    let b = unsafe { values.get_unchecked(idx) };

                    let t = (x - a.0) / (b.0 - a.0);

                    match interpolation {
                        InterpolationMode::Linear => (1.0 - t) * a.1 + t * b.1,
                        InterpolationMode::Nearest => {
                            if t < 0.5 {
                                a.1
                            } else {
                                b.1
                            }
                        }
                        InterpolationMode::CubicHermite => {
                            let t_inverse = 1.0 - t;
                            let t_inverse_sqr = t_inverse * t_inverse;
                            let t_squared = t * t;
                            let t2 = 2.0 * t;

                            // https://en.wikipedia.org/wiki/Cubic_Hermite_spline#Representations
                            let h00 = (1.0 + t2) * t_inverse_sqr;
                            let h10 = t * t_inverse_sqr;
                            let h01 = t_squared * (3.0 - t2);
                            let h11 = t_squared * (t - 1.0);

                            ((h00 * a.1) + (h10 * a.2)) + ((h01 * b.1) + (h11 * b.2))
                        }
                    }
                }
                Some(idx) => unsafe { values.get_unchecked(idx).1 },
                None => 0.0,
            },
            _ => 0.0,
        }
    }

    #[inline(always)]
    pub fn eval<S: Simd>(&self, x: Vf32<S>) -> Vf32<S> {
        match *self {
            Curve::Poly(ref poly) => x.poly(poly),

            // TODO: Figure out something better than a scalar map
            // possibly use a linear search if the curve is less than (LANES * log2(Points))
            _ => x.map_scalar(|_, x| self.eval_scalar(x)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use thermite::backends::avx2::AVX2;

    type Vf32 = <AVX2 as Simd>::Vf32;

    // See https://www.desmos.com/calculator/mb6ieiav2n

    #[test]
    fn test_curve() {
        let poly = Curve::Poly(vec![0.0, 0.0, 1.0]);

        for i in 0..128 {
            let ix = i as f32 / 128.0;

            let x = ix * 2.0;

            let xv = Vf32::splat(x);
            let y = poly.eval::<AVX2>(xv).extract(0);
            assert!((y - (x * x)).abs() < 0.001, "{} == {}", y, (x * x));
        }

        let curve = Curve::LookupTable {
            values: vec![(0.0, 0.1, -0.1), (0.3, 0.3, 0.7), (0.5, 0.6, 0.0), (1.0, 0.2, -0.3)],
            interpolation: InterpolationMode::CubicHermite,
        };

        {
            let x = 0.65;
            let y = curve.eval_scalar(x);
            let expected = 0.5325;

            assert!((expected - y).abs() < 0.001, "{} == {}", expected, y);
        }
    }
}
