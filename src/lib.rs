// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

mod coarse_now;
#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
mod tsc_now;

pub use minstant_macro::timing;

use std::time::{SystemTime, UNIX_EPOCH};

static mut NANOS_PER_CYCLE: f64 = 1.0;

#[inline]
pub fn nanos_per_cycle() -> f64 {
    unsafe { NANOS_PER_CYCLE }
}

#[derive(Copy, Clone)]
pub struct Anchor {
    unix_time_ns: u64,
    cycle: u64,
    nanos_per_cycle: f64,
}

impl Default for Anchor {
    fn default() -> Self {
        Self::new()
    }
}

impl Anchor {
    #[inline]
    pub fn new() -> Anchor {
        let unix_time_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("unexpected time drift")
            .as_nanos() as u64;
        Anchor {
            unix_time_ns,
            cycle: now(),
            nanos_per_cycle: unsafe { NANOS_PER_CYCLE },
        }
    }

    pub fn cycle_to_unix_nanos(&self, cycle: u64) -> u64 {
        if cycle > self.cycle {
            let forward_ns = (cycle - self.cycle) as f64 * self.nanos_per_cycle;
            self.unix_time_ns + forward_ns as u64
        } else {
            let backward_ns = (self.cycle - cycle) as f64 * self.nanos_per_cycle;
            self.unix_time_ns - backward_ns as u64
        }
    }
}

#[inline]
pub fn tsc_available() -> bool {
    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    if true {
        return tsc_now::tsc_available();
    }

    false
}

#[inline]
pub fn now() -> u64 {
    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    if tsc_available() {
        return tsc_now::now();
    }

    coarse_now::now()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::time::{Duration, Instant};

    #[test]
    fn test_tsc_available() {
        let _ = tsc_available();
    }

    #[test]
    fn test_monotonic() {
        let mut prev = 0;
        for _ in 0..10000 {
            let cur = now();
            assert!(cur >= prev);
            prev = cur;
        }
    }

    #[test]
    fn test_nanos_per_cycle() {
        let _ = nanos_per_cycle();
    }

    #[test]
    fn test_duration() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let cur_cycle = now();
            let cur_instant = Instant::now();
            std::thread::sleep(Duration::from_millis(rng.gen_range(100..500)));
            let check = move || {
                let duration_ns_minstant = (now() - cur_cycle) as f64 * unsafe { NANOS_PER_CYCLE };
                let duration_ns_std = Instant::now().duration_since(cur_instant).as_nanos();

                #[cfg(target_os = "windows")]
                let expect_max_delta = 20_000_000.0;
                #[cfg(not(target_os = "windows"))]
                let expect_max_delta = 5_000_000.0;

                let real_delta = (duration_ns_std as f64 - duration_ns_minstant).abs();
                assert!(real_delta < expect_max_delta, "real delta: {}", real_delta);
            };
            check();
            std::thread::spawn(check).join().expect("join failed");
        }
    }
}
