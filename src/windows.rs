// Copyright (C) 2020 Stephane Raux. Distributed under the MIT license.

use std::{
    io,
    time::Duration,
};
use winapi::{
    shared::ntdef::LARGE_INTEGER,
    um::profileapi::{QueryPerformanceCounter, QueryPerformanceFrequency},
};

#[derive(Debug)]
pub struct Clock {
    freq: u64,
    last_elapsed: Duration,
}

impl Clock {
    pub fn new() -> Self {
        let mut freq = LARGE_INTEGER::default();
        let status = unsafe {
            QueryPerformanceFrequency(&mut freq)
        };
        if status == 0 {
            let error = io::Error::last_os_error();
            panic!("QueryPerformanceFrequency failed (it is documented as infallible since \
                Windows XP): {}", error);
        }
        Self {
            freq: unsafe { *freq.QuadPart() as u64 },
            last_elapsed: Default::default(),
        }
    }

    pub fn elapsed(&mut self) -> Duration {
        let mut counter = LARGE_INTEGER::default();
        let status = unsafe {
            QueryPerformanceCounter(&mut counter)
        };
        if status == 0 {
            let error = io::Error::last_os_error();
            panic!("QueryPerformanceCounter failed (it is documented as infallible since \
                Windows XP): {}", error);
        }
        let counter = unsafe { *counter.QuadPart() as u64 };
        let seconds = counter / self.freq;
        let nanoseconds = (counter % self.freq) as u128 * 1_000_000_000 / self.freq as u128;
        let nanoseconds = nanoseconds as u32;
        let d = Duration::new(seconds, nanoseconds);
        if self.last_elapsed < d {
            self.last_elapsed = d;
        }
        self.last_elapsed
    }
}
