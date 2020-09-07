// Copyright (C) 2020 Stephane Raux. Distributed under the MIT license.

//! Defines a monotonic clock whose values are instances of `Duration`.
//!
//! # Why not `std::time::Instant`?
//!
//! `Instant` is opaque and cannot be serialized.
//!
//! # Example
//!
//! ```rust
//! let mut clock = moniclock::Clock::new();
//! let t0 = clock.elapsed();
//! let sleep_duration = std::time::Duration::from_millis(100);
//! std::thread::sleep(sleep_duration);
//! let t1 = clock.elapsed();
//! assert!(t1 - t0 >= sleep_duration);
//! ```

#![deny(warnings)]
#![deny(missing_docs)]

use std::time::Duration;

#[cfg(unix)]
#[path = "unix.rs"]
mod platform;

#[cfg(windows)]
#[path = "windows.rs"]
mod platform;

use platform::Clock as Inner;

/// Monotonic clock
#[derive(Debug)]
pub struct Clock {
    inner: Inner,
}

impl Clock {
    /// Instantiates a `Clock`.
    ///
    /// A given `Clock` instance is monotonic.
    pub fn new() -> Self {
        Self {
            inner: Inner::new(),
        }
    }

    /// Returns the time elapsed since some arbitrary epoch. The epoch is constant across all
    /// `Clock` instances for as long as the system runs.
    ///
    /// This function is monotonically increasing. Each returned `Duration` is greater or equal to
    /// any previous `Duration` returned for the same `Clock` instance.
    pub fn elapsed(&mut self) -> Duration {
        self.inner.elapsed()
    }
}

#[cfg(test)]
mod tests {
    use crate::Clock;
    use std::{
        thread::sleep,
        time::Duration,
    };

    #[test]
    fn a_clock_is_instantiated() {
        let _ = Clock::new();
    }

    #[test]
    fn clock_is_monotonic() {
        let mut clock = Clock::new();
        let t0 = clock.elapsed();
        sleep(Duration::from_millis(100));
        let t1 = clock.elapsed();
        assert!(t0 <= t1);
    }

    #[test]
    fn clock_reports_correct_duration() {
        let mut clock = Clock::new();
        let t0 = clock.elapsed();
        let sleep_duration = Duration::from_millis(100);
        sleep(sleep_duration);
        let t1 = clock.elapsed();
        assert!(t1 - t0 >= sleep_duration);
    }
}
