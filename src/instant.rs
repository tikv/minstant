// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

use crate::{duration::*, utils::*};

#[allow(unused_imports)]
use std::ptr::*;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Instant {
    Coarse(u64),
    Cycle { cycle: u64, anchor_coarse: u64 },
}
use Instant::*;

// ===== WINDOWS ===== //

#[cfg(windows)]
extern "system" {
    pub fn GetTickCount() -> libc::c_ulong;
}

// ===== MACOS ===== //

#[cfg(any(target_os = "macos", target_os = "freebsd"))]
#[allow(non_camel_case_types)]
type clockid_t = libc::c_int;

#[cfg(target_os = "macos")]
const CLOCK_MONOTONIC_RAW_APPROX: clockid_t = 5;

#[cfg(target_os = "macos")]
extern "system" {
    pub fn clock_gettime_nsec_np(clk_id: clockid_t) -> u64;
}

// ===== FREEBSD ===== //

#[cfg(target_os = "freebsd")]
const CLOCK_MONOTONIC_FAST: clockid_t = 12;

// ===== WASM ===== //

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
))]
use wasm_bindgen::prelude::*;

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
))]
#[wasm_bindgen]
extern "C" {
    type performance;

    #[wasm_bindgen(static_method_of = performance)]
    pub fn now() -> f64;
}

impl Instant {
    /// OSs which are not linux x86, the default impl is coarse.
    #[cfg(not(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64"))))]
    #[inline]
    pub fn coarse_now() -> Self {
        Self::now()
    }

    /// linux x86 strictly use coarse instead of cycles
    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline]
    pub fn coarse_now() -> Self {
        use std::mem::MaybeUninit;
        let mut tp = MaybeUninit::<libc::timespec>::uninit();
        let tp = unsafe {
            libc::clock_gettime(libc::CLOCK_MONOTONIC_COARSE, tp.as_mut_ptr());
            tp.assume_init()
        };
        Coarse(timespec_to_u64(tp.tv_sec as u64, tp.tv_nsec as u32))
    }

    #[inline]
    pub(crate) fn coarse_as_u64(&self) -> u64 {
        match self {
            Coarse(c) => *c,
            Cycle { anchor_coarse, .. } => *anchor_coarse,
        }
    }

    /// TSC is used if is available in x86 arch
    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    #[inline]
    pub fn now() -> Self {
        use crate::tsc_now;
        let (cycle, anchor_coarse) = tsc_now::now();
        if cycle != 0 {
            Cycle {
                cycle,
                anchor_coarse,
            }
        } else {
            Coarse(anchor_coarse)
        }
    }

    /// linux not x86
    #[cfg(all(
        target_os = "linux",
        target_os = "android",
        not(any(target_arch = "x86", target_arch = "x86_64"))
    ))]
    #[inline]
    pub fn now() -> Self {
        use std::mem::MaybeUninit;
        let mut tp = MaybeUninit::<libc::timespec>::uninit();
        let tp = unsafe {
            libc::clock_gettime(libc::CLOCK_MONOTONIC_COARSE, tp.as_mut_ptr());
            tp.assume_init()
        };
        Coarse(timespec_to_u64(tp.tv_sec as u64, tp.tv_nsec as u32))
    }

    #[cfg(target_os = "macos")]
    #[inline]
    pub fn now() -> Self {
        let nanos = unsafe { clock_gettime_nsec_np(CLOCK_MONOTONIC_RAW_APPROX) };
        Coarse(nanos_to_u64(nanos))
    }

    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    #[inline]
    pub fn now() -> Self {
        use std::mem::MaybeUninit;
        let mut tp = MaybeUninit::<libc::timespec>::uninit();
        let tp = unsafe {
            libc::clock_gettime(libc::CLOCK_MONOTONIC_FAST, tp.as_mut_ptr());
            tp.assume_init()
        };
        Coarse(timespec_to_u64(tp.tv_sec as u64, tp.tv_nsec as u32))
    }

    #[cfg(windows)]
    #[inline]
    pub fn now() -> Self {
        let tc = unsafe { GetTickCount() } as u64;
        Coarse(millis_to_u64(tc))
    }

    #[cfg(target_os = "wasi")]
    #[inline]
    pub fn now() -> Self {
        use wasi::{clock_time_get, CLOCKID_MONOTONIC, CLOCKID_REALTIME};
        let nanos = unsafe { clock_time_get(CLOCKID_MONOTONIC, 1_000_000) }
            .or_else(|_| unsafe { clock_time_get(CLOCKID_REALTIME, 1_000_000) })
            .expect("Clock not available");
        Coarse(nanos_to_u64(nanos))
    }

    #[cfg(all(
        any(target_arch = "wasm32", target_arch = "wasm64"),
        target_os = "unknown"
    ))]
    #[inline]
    pub fn now() -> Self {
        Coarse(millis_to_u64(performance::now() as u64))
    }

    #[inline]
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        match earlier {
            Coarse(co0) => match self {
                Coarse(co1) => Duration::Timespec(co1 - co0),
                Cycle { anchor_coarse, .. } => Duration::Timespec(anchor_coarse - co0),
            },
            Cycle {
                cycle: cycle0,
                anchor_coarse,
            } => match self {
                Coarse(old) => Duration::Timespec(*old - anchor_coarse),
                Cycle { cycle: cycle1, .. } => Duration::Cycle(cycle1 - cycle0),
            },
        }
    }

    #[inline]
    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }
}

impl Default for Instant {
    #[inline]
    fn default() -> Instant {
        Self::now()
    }
}

use std::ops::*;
impl Sub<Instant> for Instant {
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: Instant) -> Duration {
        self.duration_since(rhs)
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    #[inline]
    fn sub(self, rhs: Duration) -> Instant {
        match self {
            Coarse(co0) => match rhs {
                Duration::Timespec(t0) => Coarse(co0 - t0),
                _ => Coarse((Duration::timespec_from_u64(co0) - rhs).timespec_as_u64()),
            },
            Cycle {
                cycle,
                anchor_coarse,
            } => match rhs {
                Duration::Timespec(t0) => Coarse(t0 - anchor_coarse),
                Duration::Cycle(cy1) => Cycle {
                    anchor_coarse,
                    cycle: cycle - cy1,
                },
            },
        }
    }
}

impl SubAssign<Duration> for Instant {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;

    #[inline]
    fn add(self, rhs: Duration) -> Instant {
        match self {
            Coarse(co0) => match rhs {
                Duration::Timespec(t0) => Coarse(co0 + t0),
                Duration::Cycle(_) => {
                    Coarse((Duration::timespec_from_u64(co0) + rhs).timespec_as_u64())
                }
            },
            Cycle {
                cycle,
                anchor_coarse,
            } => match rhs {
                Duration::Timespec(t0) => Coarse(t0 + anchor_coarse),
                Duration::Cycle(cy1) => Cycle {
                    anchor_coarse,
                    cycle: cycle + cy1,
                },
            },
        }
    }
}

impl AddAssign<Duration> for Instant {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}
