//! Get instant value in nanosecond unit as fast as possible
//! but less precise.

#[allow(unused_imports)]
use std::mem::MaybeUninit;

#[cfg(windows)]
extern "system" {
    pub fn GetTickCount() -> libc::c_ulong;
}

#[cfg(any(target_os = "macos", target_os = "freebsd"))]
#[allow(non_camel_case_types)]
type clockid_t = libc::c_int;

#[cfg(target_os = "macos")]
const CLOCK_MONOTONIC_RAW_APPROX: clockid_t = 5;

#[cfg(target_os = "macos")]
extern "system" {
    pub fn clock_gettime_nsec_np(clk_id: clockid_t) -> u64;
}

#[cfg(target_os = "freebsd")]
const CLOCK_MONOTONIC_FAST: clockid_t = 12;

#[cfg(any(target_os = "linux", target_os = "android"))]
#[inline]
pub(crate) fn now() -> u64 {
    let mut tp = MaybeUninit::<libc::timespec>::uninit();
    let tp = unsafe {
        libc::clock_gettime(libc::CLOCK_MONOTONIC_COARSE, tp.as_mut_ptr());
        tp.assume_init()
    };
    tp.tv_sec as u64 * 1_000_000_000 + tp.tv_nsec as u64
}

#[cfg(target_os = "macos")]
#[inline]
pub(crate) fn now() -> u64 {
    unsafe { clock_gettime_nsec_np(CLOCK_MONOTONIC_RAW_APPROX) }
}

#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
#[inline]
pub(crate) fn now() -> u64 {
    let mut tp = MaybeUninit::<libc::timespec>::uninit();
    let tp = unsafe {
        libc::clock_gettime(libc::CLOCK_MONOTONIC_FAST, tp.as_mut_ptr());
        tp.assume_init()
    };
    tp.tv_sec as u64 * 1_000_000_000 + tp.tv_nsec as u64
}

#[cfg(all(
    unix,
    not(any(
        target_os = "macos",
        target_os = "linux",
        target_os = "android",
        target_os = "freebsd",
        target_os = "dragonfly"
    ))
))]
#[inline]
pub(crate) fn now() -> u64 {
    let mut tv = MaybeUninit::<libc::timeval>::uninit();
    let tv = unsafe {
        libc::gettimeofday(tv.as_mut_ptr(), null_mut());
        tv.assume_init()
    };
    tv.tv_sec as u64 * 1_000_000_000 + tv.tv_usec as u64 * 1_000
}

#[cfg(windows)]
#[inline]
pub(crate) fn now() -> u64 {
    let millis = unsafe { GetTickCount() } as u64;
    millis * 1_000_000
}

#[cfg(target_os = "wasi")]
#[inline]
pub(crate) fn now() -> u64 {
    use wasi::{clock_time_get, CLOCKID_MONOTONIC};
    unsafe { clock_time_get(CLOCKID_MONOTONIC, 1_000_000).expect("Clock not available") };
}

#[cfg(not(any(windows, unix, target_os = "wasi")))]
pub(crate) fn now() -> u64 {
    panic!("Unsupported target");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now() {
        let mut prev = now();
        for _ in 0..100 {
            let n = now();
            assert!(n >= prev);
            prev = n;
        }
    }
}
