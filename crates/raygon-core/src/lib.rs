#[macro_use]
pub mod macros;

pub mod slice;

#[inline]
pub fn upper_bound<F>(size: usize, mut gt: F) -> Option<usize>
where
    F: FnMut(usize) -> bool,
{
    if size == 0 {
        return None;
    }

    let mut count = size;
    let mut first = 0;
    while count > 1 {
        let half = count / 2;
        let mid = first + half;
        let greater = gt(mid);
        first = if greater { first } else { mid };
        count -= half;
    }
    let not_greater = !gt(first);
    let idx = first + not_greater as usize;

    if idx == size {
        None
    } else {
        Some(idx)
    }
}

#[inline]
pub fn lower_bound<F>(size: usize, mut lt: F) -> Option<usize>
where
    F: FnMut(usize) -> bool,
{
    if size == 0 {
        return None;
    }

    let mut count = size;
    let mut first = 0;
    while count > 1 {
        let half = count / 2;
        let mid = first + half;
        let less = lt(mid);
        first = if less { mid } else { first };
        count -= half;
    }

    let less = lt(first);
    let idx = first + less as usize;

    if idx == size {
        None
    } else {
        Some(idx)
    }
}

#[rustfmt::skip]
#[inline]
pub fn partition_bidir<T, P>(v: &mut [T], mut pred: P) -> usize
where
    P: FnMut(&T) -> bool,
{
    let mut l = 0;
    let mut r = v.len();

    unsafe {
        loop {
            'inner_left: loop {
                if l == r { return l; }

                if !pred(v.get_unchecked(l)) {
                    break 'inner_left;
                }

                l += 1;
            }

            'inner_right: loop {
                r -= 1;

                if l == r { return l; }

                if pred(v.get_unchecked(r)) {
                    break 'inner_right;
                }
            }

            core::ptr::swap(
                v.get_unchecked_mut(l) as *mut T,
                v.get_unchecked_mut(r) as *mut T,
            );

            l += 1;
        }
    }
}
