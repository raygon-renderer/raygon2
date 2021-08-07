use std::ops::Deref;
use std::slice::{from_raw_parts, from_raw_parts_mut};

use thermite::*;

pub struct Stack<'a, S: Simd> {
    stack: &'a mut [Vf32<S>],
    top: usize,
}

impl<S: Simd> Deref for Stack<'_, S> {
    type Target = [Vf32<S>];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.stack[..self.top]
    }
}

impl<'a, S: Simd> Stack<'a, S> {
    #[inline(always)]
    pub fn new(stack: &'a mut [Vf32<S>]) -> Stack<'a, S> {
        Stack { stack, top: 0 }
    }

    #[inline(always)]
    pub fn slice(&self, first: usize, n: usize) -> &'a [Vf32<S>] {
        unsafe { from_raw_parts(self.stack.as_ptr().add(first), n) }
    }

    #[inline(always)]
    pub fn slice_mut(&mut self, first: usize, n: usize) -> &'a mut [Vf32<S>] {
        unsafe { from_raw_parts_mut(self.stack.as_mut_ptr().add(first), n) }
    }
}

impl<S: Simd> Stack<'_, S> {
    #[inline(always)]
    pub fn peek<F, U>(&self, n: usize, f: F) -> U
    where
        F: FnOnce(&[Vf32<S>]) -> U,
    {
        debug_assert!(self.top >= n);
        f(self.slice(self.top - n, n))
    }

    #[inline(always)]
    pub fn peek_mut<F, U>(&mut self, n: usize, f: F) -> U
    where
        F: FnOnce(&mut [Vf32<S>]) -> U,
    {
        debug_assert!(self.top >= n);
        f(self.slice_mut(self.top - n, n))
    }

    #[inline(always)]
    pub fn peek_one<F, U>(&self, f: F) -> U
    where
        F: FnOnce(&Vf32<S>) -> U,
    {
        self.peek(1, |s| unsafe { f(s.get_unchecked(0)) })
    }

    #[inline(always)]
    pub fn peek_one_mut<F, U>(&mut self, f: F) -> U
    where
        F: FnOnce(&mut Vf32<S>) -> U,
    {
        self.peek_mut(1, |s| unsafe { f(s.get_unchecked_mut(0)) })
    }

    #[inline(always)]
    pub fn pop<F, U>(&mut self, n: usize, f: F) -> U
    where
        F: FnOnce(&mut [Vf32<S>]) -> U,
    {
        debug_assert!(self.top >= n);
        self.top -= n;
        f(self.slice_mut(self.top, n))
    }

    #[inline(always)]
    pub fn push<F>(&mut self, n: usize, f: F)
    where
        F: FnOnce(&mut [Vf32<S>]),
    {
        debug_assert!((self.top + n) < self.stack.len());
        f(self.slice_mut(self.top, n));
        self.top += n;
    }

    #[inline(always)]
    pub fn pop_to(&mut self, buf: &mut [Vf32<S>]) {
        // in debug, automatically checks for overflow
        let start = self.top - buf.len();

        unsafe {
            std::ptr::copy_nonoverlapping(self.stack.as_ptr().add(start), buf.as_mut_ptr(), buf.len());
        }

        self.top = start;
    }

    #[inline(always)]
    pub fn push_from(&mut self, buf: &[Vf32<S>]) {
        let new_top = self.top + buf.len();
        debug_assert!(new_top < self.stack.len());

        unsafe {
            std::ptr::copy_nonoverlapping(buf.as_ptr(), self.stack.as_mut_ptr().add(self.top), buf.len());
        }

        self.top = new_top;
    }

    #[inline(always)]
    pub fn peek_n<const N: usize>(&mut self) -> [Vf32<S>; N] {
        self.peek_mut(N, |head| unsafe {
            let mut buf = [Vf32::<S>::undefined(); N];
            std::ptr::copy_nonoverlapping(head.as_ptr(), buf.as_mut_ptr(), N);
            buf
        })
    }

    #[inline(always)]
    pub fn pop_n<const N: usize>(&mut self) -> [Vf32<S>; N] {
        unsafe {
            let mut buf = [Vf32::<S>::undefined(); N];
            self.pop_to(&mut buf);
            buf
        }
    }

    #[inline(always)]
    pub fn push_n<const N: usize>(&mut self, values: [Vf32<S>; N]) {
        self.push_from(&values);
    }

    #[inline(always)]
    pub fn map<F, const M: usize, const N: usize>(&mut self, mapper: F)
    where
        F: FnOnce([Vf32<S>; M]) -> [Vf32<S>; N],
    {
        let res = mapper(self.pop_n());
        self.push_n(res);
    }

    #[inline(always)]
    pub fn reduce<F, const N: usize>(&mut self, reducer: F)
    where
        F: FnOnce([Vf32<S>; N]) -> Vf32<S>,
    {
        self.map(|head| [reducer(head); 1]);
    }
}
