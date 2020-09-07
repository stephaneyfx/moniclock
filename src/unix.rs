// Copyright (C) 2020 Stephane Raux. Distributed under the MIT license.

use libc::timespec;
use std::{
    convert::TryInto,
    io,
    time::Duration,
};

#[derive(Debug)]
pub struct Clock {
    last_elapsed: Duration,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            last_elapsed: Default::default(),
        }
    }

    pub fn elapsed(&mut self) -> Duration {
        let mut t = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let status = unsafe {
            libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut t)
        };
        assert_eq!(status, 0, "clock_gettime failed: {}", io::Error::last_os_error());
        let d = Duration::new(
            t.tv_sec.try_into().expect("The number of seconds should fit in an u64"),
            t.tv_nsec.try_into().expect("The number of nanoseconds should fit in an u32"),
        );
        if self.last_elapsed < d {
            self.last_elapsed = d;
        }
        self.last_elapsed
    }
}
