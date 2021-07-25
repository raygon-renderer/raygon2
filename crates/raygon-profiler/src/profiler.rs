use std::sync::Arc;

use clocksource::Clocksource;

#[derive(Debug)]
pub struct Profiler {
    clock: Option<Arc<Clocksource>>,
    total_time: u64,
    total_count: u64,
}

impl_deepsizeof_pod!(Profiler);

#[derive(Debug)]
pub struct ProfilerGuard<'a> {
    profiler: &'a mut Profiler,
    start: u64,
}

impl ProfilerGuard<'_> {
    #[inline]
    pub fn elapsed(&self) -> u64 {
        match self.profiler.clock {
            Some(ref clock) => clock.time() - self.start,
            None => 0,
        }
    }
}

impl Drop for ProfilerGuard<'_> {
    #[inline(always)]
    fn drop(&mut self) {
        self.profiler.total_time += self.elapsed();
        self.profiler.total_count += 1;
    }
}

impl Profiler {
    pub fn new(active: bool) -> Profiler {
        Profiler {
            clock: if active { Some(Arc::new(Clocksource::new())) } else { None },
            total_time: 0,
            total_count: 0,
        }
    }

    pub fn new_with_clock(clock: Option<Arc<Clocksource>>) -> Profiler {
        Profiler {
            clock,
            total_time: 0,
            total_count: 0,
        }
    }

    pub fn set_clock(&mut self, clock: Option<Arc<Clocksource>>) {
        self.clock = clock;
    }

    pub fn set_active(&mut self, active: bool) {
        self.clock = match (self.clock.take(), active) {
            (Some(clock), true) => Some(clock),
            (_, false) => None,
            (_, true) => Some(Arc::new(Clocksource::new())),
        };
    }

    #[inline]
    pub fn time(&self) -> u64 {
        match self.clock {
            Some(ref clock) => clock.time(),
            None => 0,
        }
    }

    /// Begin profiling using RAII to trigger the end of the profiling scope.
    ///
    /// Then the resulting `ProfilerGuard` is dropped, the profiling will complete.
    #[must_use = "The profiled code section will end when the guard is dropped"]
    #[inline(always)]
    pub fn profile(&mut self) -> ProfilerGuard<'_> {
        let start = self.time();
        ProfilerGuard { profiler: self, start }
    }

    /// Returns how many samples were taken.
    ///
    /// For a `Counter` profiler, this is just the value of the counter
    pub fn samples(&self) -> u64 {
        self.total_count
    }

    /// Get the average time, in nanoseconds, of profiled code segments.
    pub fn get_average(&self) -> u64 {
        if self.total_count == 0 {
            0
        } else {
            self.total_time / self.total_count
        }
    }

    /// Get the average time, in nanoseconds, of profiled code segments, but as a double precision floating point value.
    pub fn get_average_float(&self) -> f64 {
        if self.total_count == 0 {
            0.0
        } else {
            self.total_time as f64 / self.total_count as f64
        }
    }
}
