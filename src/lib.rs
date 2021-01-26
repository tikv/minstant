use std::time::{SystemTime, UNIX_EPOCH};

mod coarse_now;
#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
mod tsc_now;

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Cycle(pub u64);

impl Cycle {
    #[inline]
    pub fn now() -> Self {
        Self(now())
    }

    #[inline]
    pub fn zero() -> Self {
        Self(0)
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    pub fn into_unix_time_ns(self, anchor: Anchor) -> u64 {
        if self > anchor.cycle {
            let forward_ns = ((self.0 - anchor.cycle.0) as f64 * 1_000_000_000.0
                / anchor.cycles_per_second as f64) as u64;
            anchor.unix_time_ns + forward_ns
        } else {
            let backward_ns = ((anchor.cycle.0 - self.0) as f64 * 1_000_000_000.0
                / anchor.cycles_per_second as f64) as u64;
            anchor.unix_time_ns - backward_ns
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Anchor {
    pub unix_time_ns: u64,
    pub cycle: Cycle,
    pub cycles_per_second: u64,
}

impl Anchor {
    #[inline]
    pub fn new() -> Anchor {
        let cycle = Cycle(now());
        let unix_time_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("unexpected time drift")
            .as_nanos() as u64;
        Anchor {
            unix_time_ns,
            cycle,
            cycles_per_second: cycles_per_second(),
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

#[inline]
pub fn cycles_per_second() -> u64 {
    #[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
    if tsc_available() {
        return tsc_now::cycles_per_second();
    }

    1_000_000_000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tsc_available() {
        let _ = tsc_available();
    }

    #[test]
    fn test_now() {
        let mut prev = 0;
        for _ in 0..100 {
            let cur = now();
            assert!(cur >= prev);
            prev = cur;
        }
    }

    #[test]
    fn test_cycles_per_second() {
        let _ = cycles_per_second();
    }
}
