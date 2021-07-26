#[macro_use]
extern crate raygon_core;

#[macro_use]
extern crate deepsize;

use std::fmt::{self, Display};

mod counter;
mod profiler;

pub use self::counter::Counter;
pub use self::profiler::Profiler;

use clocksource::Clocksource;

thread_local! {
    /// A thread-local clock-source
    static THREAD_CLOCK: Clocksource = Clocksource::new();
}

/// Returns the clock time for the current thread
#[inline(always)]
pub fn thread_time() -> u64 {
    THREAD_CLOCK.with(|clock| clock.time())
}

/// Times a function uses the thread-local clock source
#[inline(always)]
pub fn timeit<F, U>(f: F) -> (U, u64)
where
    F: FnOnce() -> U,
{
    THREAD_CLOCK.with(|clock| {
        let start_time = clock.time();

        let res = f();

        let diff = clock.time() - start_time;

        (res, diff)
    })
}

/// Split nanoseconds into seconds and remaining nanoseconds
#[inline]
pub fn split_seconds(nanoseconds: u64) -> (f32, u64) {
    let seconds = nanoseconds as f64 / 1_000_000_000f64;
    let ns = nanoseconds % 1_000_000_000;

    (seconds as f32, ns)
}

/// Convenient representation for minutes, seconds, milliseconds and nanoseconds
///
/// The `Display` implementation tries to leave out unnecessary parts.
#[derive(Debug, Clone, Copy, PartialEq, DeepSizeOf)]
pub struct SplitDuration {
    pub minutes: u32,
    pub seconds: u32,
    pub milliseconds: u32,
    pub nanoseconds: u64,
}

impl SplitDuration {
    const NS_PER_MINUTE: u64 = 60_000_000_000;
    const NS_PER_SECOND: u64 = 1_000_000_000;
    const NS_PER_MILLIS: u64 = 1_000_000;

    /// Splits nanoseconds into minutes, seconds, milliseconds and remaining nanoseconds
    pub fn split_ns(mut ns: u64) -> SplitDuration {
        let minutes = ns / Self::NS_PER_MINUTE;
        ns %= Self::NS_PER_MINUTE;

        let seconds = ns / Self::NS_PER_SECOND;
        ns %= Self::NS_PER_SECOND;

        let milliseconds = ns / Self::NS_PER_MILLIS;
        ns %= Self::NS_PER_MILLIS;

        SplitDuration {
            minutes: minutes as u32,
            seconds: seconds as u32,
            milliseconds: milliseconds as u32,
            nanoseconds: ns,
        }
    }
}

impl Display for SplitDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (
            self.minutes > 0,
            self.seconds > 0,
            self.milliseconds > 0,
            self.nanoseconds > 0 && self.nanoseconds < 10000,
        ) {
            (false, false, false, false) => f.write_str("0ns"),
            (true, false, false, false) => write!(f, "{}m", self.minutes),
            (false, true, false, false) => write!(f, "{}s", self.seconds),
            (true, true, false, false) => write!(f, "{}m {}s", self.minutes, self.seconds),
            (false, false, true, false) => write!(f, "{}ms", self.milliseconds),
            (true, false, true, false) => write!(f, "{}m {}ms", self.minutes, self.milliseconds),
            (false, true, true, false) => write!(f, "{}s {}ms", self.seconds, self.milliseconds),
            (true, true, true, false) => write!(f, "{}m {}s {}ms", self.minutes, self.seconds, self.milliseconds),
            (false, false, false, true) => write!(f, "{}ns", self.nanoseconds),
            (true, false, false, true) => write!(f, "{}m {}ns", self.minutes, self.nanoseconds),
            (false, true, false, true) => write!(f, "{}s {}ns", self.seconds, self.nanoseconds),
            (true, true, false, true) => write!(f, "{}m {}s {}ns", self.minutes, self.seconds, self.nanoseconds),
            (false, false, true, true) => write!(f, "{}ms {}ns", self.milliseconds, self.nanoseconds),
            (true, false, true, true) => write!(f, "{}m {}ms {}ns", self.minutes, self.milliseconds, self.nanoseconds),
            (false, true, true, true) => write!(f, "{}s {}ms {}ns", self.seconds, self.milliseconds, self.nanoseconds),
            (true, true, true, true) => write!(
                f,
                "{}m {}s {}ms {}ns",
                self.minutes, self.seconds, self.milliseconds, self.nanoseconds
            ),
        }
    }
}
