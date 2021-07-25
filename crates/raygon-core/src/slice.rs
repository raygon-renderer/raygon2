//! Slice utils

/// Kind of safe-unsafe access to vector elements
pub trait SliceExt<T> {
    /// In debug, check if the index is valid, otherwise do no such check
    unsafe fn get_unchecked_debug_checked(&self, index: usize) -> &T;

    /// In debug, check if the index is valid, otherwise do no such check
    unsafe fn get_unchecked_debug_checked_mut(&mut self, index: usize) -> &mut T;
}

impl<T> SliceExt<T> for [T] {
    #[inline(always)]
    unsafe fn get_unchecked_debug_checked(&self, index: usize) -> &T {
        debug_assert!(index < self.len());

        self.get_unchecked(index)
    }

    #[inline(always)]
    unsafe fn get_unchecked_debug_checked_mut(&mut self, index: usize) -> &mut T {
        debug_assert!(index < self.len());

        self.get_unchecked_mut(index)
    }
}
