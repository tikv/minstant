// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

use core::time;
use std::ops::*;

/// A simple wrapper for u64 for duration.
#[derive(Copy, Clone, Debug, Hash, Ord, Eq, PartialOrd, PartialEq)]
pub enum Duration {
    Timespec(u64),
    Cycle(u64),
}

use crate::utils::*;
use Duration::*;

#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
use crate::tsc_now::nanos_per_cycle;

/// dummy for non-linux-x86 platform
#[cfg(not(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64"))))]
fn nanos_per_cycle() -> f64 {
    1.0
}

impl Default for Duration {
    fn default() -> Self {
        Timespec(0)
    }
}

// the reason of using const function is sometimes you need
// ```rs
// const delta: Duration = Duration::from_sec(114514);
// ```
impl Duration {
    /// same params as timespec
    #[inline]
    pub const fn new(secs: u64, nsecs: u32) -> Self {
        Timespec(timespec_to_u64(secs, nsecs))
    }

    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline]
    pub const fn from_cycles(cycles: u64) -> Self {
        Cycle(cycles)
    }

    #[inline]
    pub const fn from_secs(secs: u64) -> Self {
        Timespec(secs_to_u64(secs))
    }

    #[inline]
    pub const fn from_mins(mins: u64) -> Self {
        Timespec(secs_to_u64(mins * 60))
    }

    #[inline]
    pub const fn from_hours(hours: u64) -> Self {
        Timespec(secs_to_u64(hours * 3600))
    }

    #[inline]
    pub const fn from_millis(millis: u64) -> Self {
        Timespec(millis_to_u64(millis))
    }

    #[inline]
    pub const fn from_nanos(nanos: u64) -> Self {
        Timespec(nanos_to_u64(nanos))
    }

    #[inline]
    pub fn as_secs(&self) -> u64 {
        match self {
            Timespec(t) => t >> 32,
            Cycle(c) => (*c as f64 * nanos_per_cycle() / 1_000_000_000.0) as u64,
        }
    }

    #[inline]
    pub fn as_nanos(&self) -> u128 {
        match self {
            Timespec(t) => ((*t as u128 * 0x7735940) >> 29) as u128,
            Cycle(c) => (*c as f64 * nanos_per_cycle()) as u128,
        }
    }

    #[inline]
    pub fn into_time(&mut self) {
        match self {
            Cycle(c) => {
                let nanos = *c as f64 * nanos_per_cycle();
                *self = Self::from_nanos(nanos as u64);
            }
            _ => (),
        }
    }

    #[inline]
    pub fn subsec_nanos(&self) -> u32 {
        match self {
            Timespec(t) => ((*t as u32 as u64 * 0x7735940) >> 29) as u32 + 1,
            Cycle(c) => ((*c as f64 * nanos_per_cycle()) as u128 % 1_000_000_000) as u32,
        }
    }
    #[inline]
    pub(crate) fn timespec_from_u64(from: u64) -> Self {
        Timespec(from)
    }

    #[inline]
    pub(crate) fn timespec_as_u64(&self) -> u64 {
        match self {
            Timespec(u) => *u,
            Cycle(_) => panic!("???"),
        }
    }
}

impl Add for Duration {
    type Output = Duration;

    #[inline]
    fn add(self, rhs: Duration) -> Duration {
        match self {
            Timespec(t1) => match rhs {
                Timespec(t2) => Timespec(t1 + t2),
                Cycle(c1) => Cycle((self.as_nanos() as f64 / nanos_per_cycle()) as u64 + c1),
            },
            Cycle(c1) => match rhs {
                Timespec(t2) => Cycle(c1 + (rhs.as_nanos() as f64 / nanos_per_cycle()) as u64),
                Cycle(c2) => Cycle(c1 + c2),
            },
        }
    }
}

impl AddAssign for Duration {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub for Duration {
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: Duration) -> Duration {
        match self {
            Timespec(t1) => match rhs {
                Timespec(t2) => Timespec(t1 - t2),
                Cycle(c1) => Cycle((self.as_nanos() as f64 / nanos_per_cycle()) as u64 - c1),
            },
            Cycle(c1) => match rhs {
                Timespec(t2) => Cycle(c1 - (rhs.as_nanos() as f64 / nanos_per_cycle()) as u64),
                Cycle(c2) => Cycle(c1 - c2),
            },
        }
    }
}

impl SubAssign for Duration {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Into<time::Duration> for Duration {
    #[inline]
    fn into(self) -> time::Duration {
        time::Duration::new(self.as_secs(), self.subsec_nanos())
    }
}

impl From<time::Duration> for Duration {
    #[inline]
    fn from(duration_sys: time::Duration) -> Duration {
        Duration::new(duration_sys.as_secs(), duration_sys.subsec_nanos())
    }
}
