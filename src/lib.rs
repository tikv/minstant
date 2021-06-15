// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

mod coarse_now;

#[cfg(test)]
mod test;

#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
mod tsc_now;

pub use minstant_macro::timing;

use std::time::{SystemTime, UNIX_EPOCH};

#[inline]
pub fn now() -> u64 {
    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    if tsc_available() {
        tsc_now::now()
    } else {
        coarse_now::now()
    }
    #[cfg(not(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64"))))]
    {
        coarse_now::now()
    }
}

#[inline]
pub fn tsc_available() -> bool {
    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        tsc_now::tsc_available()
    }
    #[cfg(not(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64"))))]
    {
        false
    }
}

#[inline]
pub fn nanos_per_cycle() -> f64 {
    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    {
        tsc_now::nanos_per_cycle()
    }
    #[cfg(not(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64"))))]
    {
        1.0
    }
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
            nanos_per_cycle: nanos_per_cycle(),
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
