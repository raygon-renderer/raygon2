
#[inline(always)]
pub fn difference_of_products(a: f32, b: f32, c: f32, d: f32) -> f32 {
    difference_of_products2(a, b, c, d, c * d)
}

#[inline(always)]
pub fn difference_of_products2(a: f32, b: f32, c: f32, d: f32, cd: f32) -> f32 {
    a.mul_add(b, -cd) + c.mul_add(-d, cd)
}

#[inline(always)]
pub fn sum_of_products(a: f32, b: f32, c: f32, d: f32) -> f32 {
    sum_of_products2(a, b, c, d, c * d)
}

#[inline(always)]
pub fn sum_of_products2(a: f32, b: f32, c: f32, d: f32, cd: f32) -> f32 {
    a.mul_add(b, cd) + c.mul_add(d, -cd)
}
