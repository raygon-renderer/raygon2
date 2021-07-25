use std::fmt::{self, Display};
use std::sync::atomic::{AtomicU64, Ordering};

use cache_padded::CachePadded;

/// A simple atomic counter
///
/// The counter is cache-aligned to avoid accidental purging of nearby cache items
#[derive(Debug)]
#[repr(transparent)]
pub struct Counter(CachePadded<AtomicU64>);

impl Display for Counter {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.load(Ordering::SeqCst).fmt(f)
    }
}

impl Counter {
    /// Create a new counter at 0
    #[inline(always)]
    pub const fn new() -> Counter {
        Counter(CachePadded::new(AtomicU64::new(0)))
    }
}

impl Counter {
    /// Increments the counter by 1
    ///
    /// This uses a Relaxed memory ordering, so it should not be used for logic.
    #[inline(always)]
    pub fn incr(&self) -> u64 {
        self.add(1)
    }

    /// Add a `u64` to the counter
    ///
    /// This uses a Relaxed memory ordering, so it should not be used for logic.
    #[inline(always)]
    pub fn add(&self, value: u64) -> u64 {
        self.0.fetch_add(value, Ordering::Relaxed)
    }

    /// Get the value of the counter.
    ///
    /// This uses a Relaxed memory ordering, so it should not be used for logic.
    #[inline(always)]
    pub fn get(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }
}
