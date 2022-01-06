// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

use std::{
    ops::{Add, AddAssign, Sub, SubAssign},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Copy)]
pub struct Instant(u64);

impl Instant {
#[inline]
    pub fn now() -> Instant {
        Instant(crate::current_cycle())
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        self.checked_duration_since(earlier)
            .expect("supplied instant is later than self")
    }

    pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration> {
        Some(Duration::from_nanos(
            (self.0.checked_sub(earlier.0)? as f64 * crate::nanos_per_cycle()).round() as u64,
        ))
    }

    pub fn saturating_duration_since(&self, earlier: Instant) -> Duration {
        self.checked_duration_since(earlier).unwrap_or_default()
    }

    #[inline]
    pub fn elapsed(&self) -> Duration {
        Instant::now() - *self
    }

    pub fn checked_add(&self, duration: Duration) -> Option<Instant> {
        self.0
            .checked_add(
                (duration.as_nanos() as u64 as f64 / crate::nanos_per_cycle()).round() as u64,
            )
            .map(Instant)
    }

    pub fn checked_sub(&self, duration: Duration) -> Option<Instant> {
        self.0
            .checked_sub(
                (duration.as_nanos() as u64 as f64 / crate::nanos_per_cycle()).round() as u64,
            )
            .map(Instant)
    }

    pub fn as_unix_nanos(&self, anchor: &Anchor) -> u64 {
        if self.0 > anchor.cycle {
            let forward_ns =
                ((self.0 as f64 - anchor.cycle as f64) * crate::nanos_per_cycle()).round() as u64;
            anchor.unix_time_ns + forward_ns
        } else {
            let backward_ns =
                ((anchor.cycle as f64 - self.0 as f64) * crate::nanos_per_cycle()).round() as u64;
            anchor.unix_time_ns - backward_ns
        }
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(self, other: Duration) -> Instant {
        self.checked_add(other)
            .expect("overflow when adding duration to instant")
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, other: Duration) {
        *self = *self + other;
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(self, other: Duration) -> Instant {
        self.checked_sub(other)
            .expect("overflow when subtracting duration from instant")
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, other: Duration) {
        *self = *self - other;
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, other: Instant) -> Duration {
        self.duration_since(other)
    }
}

impl std::fmt::Debug for Instant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Copy, Clone)]
pub struct Anchor {
    unix_time_ns: u64,
    cycle: u64,
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
            cycle: crate::current_cycle(),
        }
    }
}
