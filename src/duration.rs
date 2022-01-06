// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

pub struct Duration(u64);


impl Duration {
    /// The duration of one second.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(duration_constants)]
    /// use std::time::Duration;
    ///
    /// assert_eq!(Duration::SECOND, Duration::from_secs(1));
    /// ```
    #[unstable(feature = "duration_constants", issue = "57391")]
    pub const SECOND: Duration = Duration::from_secs(1);

    /// The duration of one millisecond.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(duration_constants)]
    /// use std::time::Duration;
    ///
    /// assert_eq!(Duration::MILLISECOND, Duration::from_millis(1));
    /// ```
    #[unstable(feature = "duration_constants", issue = "57391")]
    pub const MILLISECOND: Duration = Duration::from_millis(1);

    /// The duration of one microsecond.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(duration_constants)]
    /// use std::time::Duration;
    ///
    /// assert_eq!(Duration::MICROSECOND, Duration::from_micros(1));
    /// ```
    #[unstable(feature = "duration_constants", issue = "57391")]
    pub const MICROSECOND: Duration = Duration::from_micros(1);

    /// The duration of one nanosecond.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(duration_constants)]
    /// use std::time::Duration;
    ///
    /// assert_eq!(Duration::NANOSECOND, Duration::from_nanos(1));
    /// ```
    #[unstable(feature = "duration_constants", issue = "57391")]
    pub const NANOSECOND: Duration = Duration::from_nanos(1);

    /// A duration of zero time.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let duration = Duration::ZERO;
    /// assert!(duration.is_zero());
    /// assert_eq!(duration.as_nanos(), 0);
    /// ```
    #[stable(feature = "duration_zero", since = "1.53.0")]
    pub const ZERO: Duration = Duration::from_nanos(0);

    /// The maximum duration.
    ///
    /// May vary by platform as necessary. Must be able to contain the difference between
    /// two instances of [`Instant`] or two instances of [`SystemTime`].
    /// This constraint gives it a value of about 584,942,417,355 years in practice,
    /// which is currently used on all platforms.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// assert_eq!(Duration::MAX, Duration::new(u64::MAX, 1_000_000_000 - 1));
    /// ```
    /// [`Instant`]: ../../std/time/struct.Instant.html
    /// [`SystemTime`]: ../../std/time/struct.SystemTime.html
    #[stable(feature = "duration_saturating_ops", since = "1.53.0")]
    pub const MAX: Duration = Duration::new(u64::MAX, NANOS_PER_SEC - 1);

    /// Creates a new `Duration` from the specified number of whole seconds and
    /// additional nanoseconds.
    ///
    /// If the number of nanoseconds is greater than 1 billion (the number of
    /// nanoseconds in a second), then it will carry over into the seconds provided.
    ///
    /// # Panics
    ///
    /// This constructor will panic if the carry from the nanoseconds overflows
    /// the seconds counter.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let five_seconds = Duration::new(5, 0);
    /// ```
    #[stable(feature = "duration", since = "1.3.0")]
    #[inline]
    #[rustc_const_unstable(feature = "duration_consts_2", issue = "72440")]
    #[must_use]
    pub const fn new(secs: u64, nanos: u32) -> Duration {
        let secs = match secs.checked_add((nanos / NANOS_PER_SEC) as u64) {
            Some(secs) => secs,
            None => panic!("overflow in Duration::new"),
        };
        let nanos = nanos % NANOS_PER_SEC;
        Duration { secs, nanos }
    }

    /// Creates a new `Duration` from the specified number of whole seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let duration = Duration::from_secs(5);
    ///
    /// assert_eq!(5, duration.as_secs());
    /// assert_eq!(0, duration.subsec_nanos());
    /// ```
    #[stable(feature = "duration", since = "1.3.0")]
    #[must_use]
    #[inline]
    #[rustc_const_stable(feature = "duration_consts", since = "1.32.0")]
    pub const fn from_secs(secs: u64) -> Duration {
        Duration { secs, nanos: 0 }
    }

    /// Creates a new `Duration` from the specified number of milliseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let duration = Duration::from_millis(2569);
    ///
    /// assert_eq!(2, duration.as_secs());
    /// assert_eq!(569_000_000, duration.subsec_nanos());
    /// ```
    #[stable(feature = "duration", since = "1.3.0")]
    #[must_use]
    #[inline]
    #[rustc_const_stable(feature = "duration_consts", since = "1.32.0")]
    pub const fn from_millis(millis: u64) -> Duration {
        Duration {
            secs: millis / MILLIS_PER_SEC,
            nanos: ((millis % MILLIS_PER_SEC) as u32) * NANOS_PER_MILLI,
        }
    }

    /// Creates a new `Duration` from the specified number of microseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let duration = Duration::from_micros(1_000_002);
    ///
    /// assert_eq!(1, duration.as_secs());
    /// assert_eq!(2000, duration.subsec_nanos());
    /// ```
    #[stable(feature = "duration_from_micros", since = "1.27.0")]
    #[must_use]
    #[inline]
    #[rustc_const_stable(feature = "duration_consts", since = "1.32.0")]
    pub const fn from_micros(micros: u64) -> Duration {
        Duration {
            secs: micros / MICROS_PER_SEC,
            nanos: ((micros % MICROS_PER_SEC) as u32) * NANOS_PER_MICRO,
        }
    }

    /// Creates a new `Duration` from the specified number of nanoseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let duration = Duration::from_nanos(1_000_000_123);
    ///
    /// assert_eq!(1, duration.as_secs());
    /// assert_eq!(123, duration.subsec_nanos());
    /// ```
    #[stable(feature = "duration_extras", since = "1.27.0")]
    #[must_use]
    #[inline]
    #[rustc_const_stable(feature = "duration_consts", since = "1.32.0")]
    pub const fn from_nanos(nanos: u64) -> Duration {
        Duration {
            secs: nanos / (NANOS_PER_SEC as u64),
            nanos: (nanos % (NANOS_PER_SEC as u64)) as u32,
        }
    }

    /// Returns true if this `Duration` spans no time.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// assert!(Duration::ZERO.is_zero());
    /// assert!(Duration::new(0, 0).is_zero());
    /// assert!(Duration::from_nanos(0).is_zero());
    /// assert!(Duration::from_secs(0).is_zero());
    ///
    /// assert!(!Duration::new(1, 1).is_zero());
    /// assert!(!Duration::from_nanos(1).is_zero());
    /// assert!(!Duration::from_secs(1).is_zero());
    /// ```
    #[must_use]
    #[stable(feature = "duration_zero", since = "1.53.0")]
    #[rustc_const_stable(feature = "duration_zero", since = "1.53.0")]
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.secs == 0 && self.nanos == 0
    }

    /// Returns the number of _whole_ seconds contained by this `Duration`.
    ///
    /// The returned value does not include the fractional (nanosecond) part of the
    /// duration, which can be obtained using [`subsec_nanos`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let duration = Duration::new(5, 730023852);
    /// assert_eq!(duration.as_secs(), 5);
    /// ```
    ///
    /// To determine the total number of seconds represented by the `Duration`,
    /// use `as_secs` in combination with [`subsec_nanos`]:
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let duration = Duration::new(5, 730023852);
    ///
    /// assert_eq!(5.730023852,
    ///            duration.as_secs() as f64
    ///            + duration.subsec_nanos() as f64 * 1e-9);
    /// ```
    ///
    /// [`subsec_nanos`]: Duration::subsec_nanos
    #[stable(feature = "duration", since = "1.3.0")]
    #[rustc_const_stable(feature = "duration", since = "1.32.0")]
    #[must_use]
    #[inline]
    pub const fn as_secs(&self) -> u64 {
        self.secs
    }

    /// Returns the fractional part of this `Duration`, in whole milliseconds.
    ///
    /// This method does **not** return the length of the duration when
    /// represented by milliseconds. The returned number always represents a
    /// fractional portion of a second (i.e., it is less than one thousand).
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let duration = Duration::from_millis(5432);
    /// assert_eq!(duration.as_secs(), 5);
    /// assert_eq!(duration.subsec_millis(), 432);
    /// ```
    #[stable(feature = "duration_extras", since = "1.27.0")]
    #[rustc_const_stable(feature = "duration_extras", since = "1.32.0")]
    #[inline]
    pub const fn subsec_millis(&self) -> u32 {
        self.nanos / NANOS_PER_MILLI
    }

    /// Returns the fractional part of this `Duration`, in whole microseconds.
    ///
    /// This method does **not** return the length of the duration when
    /// represented by microseconds. The returned number always represents a
    /// fractional portion of a second (i.e., it is less than one million).
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let duration = Duration::from_micros(1_234_567);
    /// assert_eq!(duration.as_secs(), 1);
    /// assert_eq!(duration.subsec_micros(), 234_567);
    /// ```
    #[stable(feature = "duration_extras", since = "1.27.0")]
    #[rustc_const_stable(feature = "duration_extras", since = "1.32.0")]
    #[inline]
    pub const fn subsec_micros(&self) -> u32 {
        self.nanos / NANOS_PER_MICRO
    }

    /// Returns the fractional part of this `Duration`, in nanoseconds.
    ///
    /// This method does **not** return the length of the duration when
    /// represented by nanoseconds. The returned number always represents a
    /// fractional portion of a second (i.e., it is less than one billion).
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let duration = Duration::from_millis(5010);
    /// assert_eq!(duration.as_secs(), 5);
    /// assert_eq!(duration.subsec_nanos(), 10_000_000);
    /// ```
    #[stable(feature = "duration", since = "1.3.0")]
    #[rustc_const_stable(feature = "duration", since = "1.32.0")]
    #[inline]
    pub const fn subsec_nanos(&self) -> u32 {
        self.nanos
    }

    /// Returns the total number of whole milliseconds contained by this `Duration`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let duration = Duration::new(5, 730023852);
    /// assert_eq!(duration.as_millis(), 5730);
    /// ```
    #[stable(feature = "duration_as_u128", since = "1.33.0")]
    #[rustc_const_stable(feature = "duration_as_u128", since = "1.33.0")]
    #[must_use]
    #[inline]
    pub const fn as_millis(&self) -> u128 {
        self.secs as u128 * MILLIS_PER_SEC as u128 + (self.nanos / NANOS_PER_MILLI) as u128
    }

    /// Returns the total number of whole microseconds contained by this `Duration`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let duration = Duration::new(5, 730023852);
    /// assert_eq!(duration.as_micros(), 5730023);
    /// ```
    #[stable(feature = "duration_as_u128", since = "1.33.0")]
    #[rustc_const_stable(feature = "duration_as_u128", since = "1.33.0")]
    #[must_use]
    #[inline]
    pub const fn as_micros(&self) -> u128 {
        self.secs as u128 * MICROS_PER_SEC as u128 + (self.nanos / NANOS_PER_MICRO) as u128
    }

    /// Returns the total number of nanoseconds contained by this `Duration`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let duration = Duration::new(5, 730023852);
    /// assert_eq!(duration.as_nanos(), 5730023852);
    /// ```
    #[stable(feature = "duration_as_u128", since = "1.33.0")]
    #[rustc_const_stable(feature = "duration_as_u128", since = "1.33.0")]
    #[must_use]
    #[inline]
    pub const fn as_nanos(&self) -> u128 {
        self.secs as u128 * NANOS_PER_SEC as u128 + self.nanos as u128
    }

    /// Checked `Duration` addition. Computes `self + other`, returning [`None`]
    /// if overflow occurred.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// assert_eq!(Duration::new(0, 0).checked_add(Duration::new(0, 1)), Some(Duration::new(0, 1)));
    /// assert_eq!(Duration::new(1, 0).checked_add(Duration::new(u64::MAX, 0)), None);
    /// ```
    #[stable(feature = "duration_checked_ops", since = "1.16.0")]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    #[rustc_const_unstable(feature = "duration_consts_2", issue = "72440")]
    pub const fn checked_add(self, rhs: Duration) -> Option<Duration> {
        if let Some(mut secs) = self.secs.checked_add(rhs.secs) {
            let mut nanos = self.nanos + rhs.nanos;
            if nanos >= NANOS_PER_SEC {
                nanos -= NANOS_PER_SEC;
                if let Some(new_secs) = secs.checked_add(1) {
                    secs = new_secs;
                } else {
                    return None;
                }
            }
            debug_assert!(nanos < NANOS_PER_SEC);
            Some(Duration { secs, nanos })
        } else {
            None
        }
    }

    /// Saturating `Duration` addition. Computes `self + other`, returning [`Duration::MAX`]
    /// if overflow occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(duration_constants)]
    /// use std::time::Duration;
    ///
    /// assert_eq!(Duration::new(0, 0).saturating_add(Duration::new(0, 1)), Duration::new(0, 1));
    /// assert_eq!(Duration::new(1, 0).saturating_add(Duration::new(u64::MAX, 0)), Duration::MAX);
    /// ```
    #[stable(feature = "duration_saturating_ops", since = "1.53.0")]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    #[rustc_const_unstable(feature = "duration_consts_2", issue = "72440")]
    pub const fn saturating_add(self, rhs: Duration) -> Duration {
        match self.checked_add(rhs) {
            Some(res) => res,
            None => Duration::MAX,
        }
    }

    /// Checked `Duration` subtraction. Computes `self - other`, returning [`None`]
    /// if the result would be negative or if overflow occurred.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// assert_eq!(Duration::new(0, 1).checked_sub(Duration::new(0, 0)), Some(Duration::new(0, 1)));
    /// assert_eq!(Duration::new(0, 0).checked_sub(Duration::new(0, 1)), None);
    /// ```
    #[stable(feature = "duration_checked_ops", since = "1.16.0")]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    #[rustc_const_unstable(feature = "duration_consts_2", issue = "72440")]
    pub const fn checked_sub(self, rhs: Duration) -> Option<Duration> {
        if let Some(mut secs) = self.secs.checked_sub(rhs.secs) {
            let nanos = if self.nanos >= rhs.nanos {
                self.nanos - rhs.nanos
            } else if let Some(sub_secs) = secs.checked_sub(1) {
                secs = sub_secs;
                self.nanos + NANOS_PER_SEC - rhs.nanos
            } else {
                return None;
            };
            debug_assert!(nanos < NANOS_PER_SEC);
            Some(Duration { secs, nanos })
        } else {
            None
        }
    }

    /// Saturating `Duration` subtraction. Computes `self - other`, returning [`Duration::ZERO`]
    /// if the result would be negative or if overflow occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// assert_eq!(Duration::new(0, 1).saturating_sub(Duration::new(0, 0)), Duration::new(0, 1));
    /// assert_eq!(Duration::new(0, 0).saturating_sub(Duration::new(0, 1)), Duration::ZERO);
    /// ```
    #[stable(feature = "duration_saturating_ops", since = "1.53.0")]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    #[rustc_const_unstable(feature = "duration_consts_2", issue = "72440")]
    pub const fn saturating_sub(self, rhs: Duration) -> Duration {
        match self.checked_sub(rhs) {
            Some(res) => res,
            None => Duration::ZERO,
        }
    }

    /// Checked `Duration` multiplication. Computes `self * other`, returning
    /// [`None`] if overflow occurred.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// assert_eq!(Duration::new(0, 500_000_001).checked_mul(2), Some(Duration::new(1, 2)));
    /// assert_eq!(Duration::new(u64::MAX - 1, 0).checked_mul(2), None);
    /// ```
    #[stable(feature = "duration_checked_ops", since = "1.16.0")]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    #[rustc_const_unstable(feature = "duration_consts_2", issue = "72440")]
    pub const fn checked_mul(self, rhs: u32) -> Option<Duration> {
        // Multiply nanoseconds as u64, because it cannot overflow that way.
        let total_nanos = self.nanos as u64 * rhs as u64;
        let extra_secs = total_nanos / (NANOS_PER_SEC as u64);
        let nanos = (total_nanos % (NANOS_PER_SEC as u64)) as u32;
        if let Some(s) = self.secs.checked_mul(rhs as u64) {
            if let Some(secs) = s.checked_add(extra_secs) {
                debug_assert!(nanos < NANOS_PER_SEC);
                return Some(Duration { secs, nanos });
            }
        }
        None
    }

    /// Saturating `Duration` multiplication. Computes `self * other`, returning
    /// [`Duration::MAX`] if overflow occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(duration_constants)]
    /// use std::time::Duration;
    ///
    /// assert_eq!(Duration::new(0, 500_000_001).saturating_mul(2), Duration::new(1, 2));
    /// assert_eq!(Duration::new(u64::MAX - 1, 0).saturating_mul(2), Duration::MAX);
    /// ```
    #[stable(feature = "duration_saturating_ops", since = "1.53.0")]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    #[rustc_const_unstable(feature = "duration_consts_2", issue = "72440")]
    pub const fn saturating_mul(self, rhs: u32) -> Duration {
        match self.checked_mul(rhs) {
            Some(res) => res,
            None => Duration::MAX,
        }
    }

    /// Checked `Duration` division. Computes `self / other`, returning [`None`]
    /// if `other == 0`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// assert_eq!(Duration::new(2, 0).checked_div(2), Some(Duration::new(1, 0)));
    /// assert_eq!(Duration::new(1, 0).checked_div(2), Some(Duration::new(0, 500_000_000)));
    /// assert_eq!(Duration::new(2, 0).checked_div(0), None);
    /// ```
    #[stable(feature = "duration_checked_ops", since = "1.16.0")]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    #[rustc_const_unstable(feature = "duration_consts_2", issue = "72440")]
    pub const fn checked_div(self, rhs: u32) -> Option<Duration> {
        if rhs != 0 {
            let secs = self.secs / (rhs as u64);
            let carry = self.secs - secs * (rhs as u64);
            let extra_nanos = carry * (NANOS_PER_SEC as u64) / (rhs as u64);
            let nanos = self.nanos / rhs + (extra_nanos as u32);
            debug_assert!(nanos < NANOS_PER_SEC);
            Some(Duration { secs, nanos })
        } else {
            None
        }
    }

    /// Returns the number of seconds contained by this `Duration` as `f64`.
    ///
    /// The returned value does include the fractional (nanosecond) part of the duration.
    ///
    /// # Examples
    /// ```
    /// use std::time::Duration;
    ///
    /// let dur = Duration::new(2, 700_000_000);
    /// assert_eq!(dur.as_secs_f64(), 2.7);
    /// ```
    #[stable(feature = "duration_float", since = "1.38.0")]
    #[must_use]
    #[inline]
    #[rustc_const_unstable(feature = "duration_consts_2", issue = "72440")]
    pub const fn as_secs_f64(&self) -> f64 {
        (self.secs as f64) + (self.nanos as f64) / (NANOS_PER_SEC as f64)
    }

    /// Returns the number of seconds contained by this `Duration` as `f32`.
    ///
    /// The returned value does include the fractional (nanosecond) part of the duration.
    ///
    /// # Examples
    /// ```
    /// use std::time::Duration;
    ///
    /// let dur = Duration::new(2, 700_000_000);
    /// assert_eq!(dur.as_secs_f32(), 2.7);
    /// ```
    #[stable(feature = "duration_float", since = "1.38.0")]
    #[must_use]
    #[inline]
    #[rustc_const_unstable(feature = "duration_consts_2", issue = "72440")]
    pub const fn as_secs_f32(&self) -> f32 {
        (self.secs as f32) + (self.nanos as f32) / (NANOS_PER_SEC as f32)
    }

    /// Creates a new `Duration` from the specified number of seconds represented
    /// as `f64`.
    ///
    /// # Panics
    /// This constructor will panic if `secs` is not finite, negative or overflows `Duration`.
    ///
    /// # Examples
    /// ```
    /// use std::time::Duration;
    ///
    /// let dur = Duration::from_secs_f64(2.7);
    /// assert_eq!(dur, Duration::new(2, 700_000_000));
    /// ```
    #[stable(feature = "duration_float", since = "1.38.0")]
    #[must_use]
    #[inline]
    #[rustc_const_unstable(feature = "duration_consts_2", issue = "72440")]
    pub const fn from_secs_f64(secs: f64) -> Duration {
        match Duration::try_from_secs_f64(secs) {
            Ok(v) => v,
            Err(e) => crate::panicking::panic(e.description()),
        }
    }

    /// The checked version of [`from_secs_f64`].
    ///
    /// [`from_secs_f64`]: Duration::from_secs_f64
    ///
    /// This constructor will return an `Err` if `secs` is not finite, negative or overflows `Duration`.
    ///
    /// # Examples
    /// ```
    /// #![feature(duration_checked_float)]
    ///
    /// use std::time::Duration;
    ///
    /// let dur = Duration::try_from_secs_f64(2.7);
    /// assert_eq!(dur, Ok(Duration::new(2, 700_000_000)));
    ///
    /// let negative = Duration::try_from_secs_f64(-5.0);
    /// assert!(negative.is_err());
    /// ```
    #[unstable(feature = "duration_checked_float", issue = "83400")]
    #[inline]
    pub const fn try_from_secs_f64(secs: f64) -> Result<Duration, FromSecsError> {
        const MAX_NANOS_F64: f64 = ((u64::MAX as u128 + 1) * (NANOS_PER_SEC as u128)) as f64;
        let nanos = secs * (NANOS_PER_SEC as f64);
        if !nanos.is_finite() {
            Err(FromSecsError { kind: FromSecsErrorKind::NonFinite })
        } else if nanos >= MAX_NANOS_F64 {
            Err(FromSecsError { kind: FromSecsErrorKind::Overflow })
        } else if nanos < 0.0 {
            Err(FromSecsError { kind: FromSecsErrorKind::Underflow })
        } else {
            let nanos = nanos as u128;
            Ok(Duration {
                secs: (nanos / (NANOS_PER_SEC as u128)) as u64,
                nanos: (nanos % (NANOS_PER_SEC as u128)) as u32,
            })
        }
    }

    /// Creates a new `Duration` from the specified number of seconds represented
    /// as `f32`.
    ///
    /// # Panics
    /// This constructor will panic if `secs` is not finite, negative or overflows `Duration`.
    ///
    /// # Examples
    /// ```
    /// use std::time::Duration;
    ///
    /// let dur = Duration::from_secs_f32(2.7);
    /// assert_eq!(dur, Duration::new(2, 700_000_000));
    /// ```
    #[stable(feature = "duration_float", since = "1.38.0")]
    #[must_use]
    #[inline]
    #[rustc_const_unstable(feature = "duration_consts_2", issue = "72440")]
    pub const fn from_secs_f32(secs: f32) -> Duration {
        match Duration::try_from_secs_f32(secs) {
            Ok(v) => v,
            Err(e) => crate::panicking::panic(e.description()),
        }
    }

    /// The checked version of [`from_secs_f32`].
    ///
    /// [`from_secs_f32`]: Duration::from_secs_f32
    ///
    /// This constructor will return an `Err` if `secs` is not finite, negative or overflows `Duration`.
    ///
    /// # Examples
    /// ```
    /// #![feature(duration_checked_float)]
    ///
    /// use std::time::Duration;
    ///
    /// let dur = Duration::try_from_secs_f32(2.7);
    /// assert_eq!(dur, Ok(Duration::new(2, 700_000_000)));
    ///
    /// let negative = Duration::try_from_secs_f32(-5.0);
    /// assert!(negative.is_err());
    /// ```
    #[unstable(feature = "duration_checked_float", issue = "83400")]
    #[inline]
    pub const fn try_from_secs_f32(secs: f32) -> Result<Duration, FromSecsError> {
        const MAX_NANOS_F32: f32 = ((u64::MAX as u128 + 1) * (NANOS_PER_SEC as u128)) as f32;
        let nanos = secs * (NANOS_PER_SEC as f32);
        if !nanos.is_finite() {
            Err(FromSecsError { kind: FromSecsErrorKind::NonFinite })
        } else if nanos >= MAX_NANOS_F32 {
            Err(FromSecsError { kind: FromSecsErrorKind::Overflow })
        } else if nanos < 0.0 {
            Err(FromSecsError { kind: FromSecsErrorKind::Underflow })
        } else {
            let nanos = nanos as u128;
            Ok(Duration {
                secs: (nanos / (NANOS_PER_SEC as u128)) as u64,
                nanos: (nanos % (NANOS_PER_SEC as u128)) as u32,
            })
        }
    }

    /// Multiplies `Duration` by `f64`.
    ///
    /// # Panics
    /// This method will panic if result is not finite, negative or overflows `Duration`.
    ///
    /// # Examples
    /// ```
    /// use std::time::Duration;
    ///
    /// let dur = Duration::new(2, 700_000_000);
    /// assert_eq!(dur.mul_f64(3.14), Duration::new(8, 478_000_000));
    /// assert_eq!(dur.mul_f64(3.14e5), Duration::new(847_800, 0));
    /// ```
    #[stable(feature = "duration_float", since = "1.38.0")]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    #[rustc_const_unstable(feature = "duration_consts_2", issue = "72440")]
    pub const fn mul_f64(self, rhs: f64) -> Duration {
        Duration::from_secs_f64(rhs * self.as_secs_f64())
    }

    /// Multiplies `Duration` by `f32`.
    ///
    /// # Panics
    /// This method will panic if result is not finite, negative or overflows `Duration`.
    ///
    /// # Examples
    /// ```
    /// use std::time::Duration;
    ///
    /// let dur = Duration::new(2, 700_000_000);
    /// // note that due to rounding errors result is slightly different
    /// // from 8.478 and 847800.0
    /// assert_eq!(dur.mul_f32(3.14), Duration::new(8, 478_000_640));
    /// assert_eq!(dur.mul_f32(3.14e5), Duration::new(847799, 969_120_256));
    /// ```
    #[stable(feature = "duration_float", since = "1.38.0")]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    #[rustc_const_unstable(feature = "duration_consts_2", issue = "72440")]
    pub const fn mul_f32(self, rhs: f32) -> Duration {
        Duration::from_secs_f32(rhs * self.as_secs_f32())
    }

    /// Divide `Duration` by `f64`.
    ///
    /// # Panics
    /// This method will panic if result is not finite, negative or overflows `Duration`.
    ///
    /// # Examples
    /// ```
    /// use std::time::Duration;
    ///
    /// let dur = Duration::new(2, 700_000_000);
    /// assert_eq!(dur.div_f64(3.14), Duration::new(0, 859_872_611));
    /// // note that truncation is used, not rounding
    /// assert_eq!(dur.div_f64(3.14e5), Duration::new(0, 8_598));
    /// ```
    #[stable(feature = "duration_float", since = "1.38.0")]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    #[rustc_const_unstable(feature = "duration_consts_2", issue = "72440")]
    pub const fn div_f64(self, rhs: f64) -> Duration {
        Duration::from_secs_f64(self.as_secs_f64() / rhs)
    }

    /// Divide `Duration` by `f32`.
    ///
    /// # Panics
    /// This method will panic if result is not finite, negative or overflows `Duration`.
    ///
    /// # Examples
    /// ```
    /// use std::time::Duration;
    ///
    /// let dur = Duration::new(2, 700_000_000);
    /// // note that due to rounding errors result is slightly
    /// // different from 0.859_872_611
    /// assert_eq!(dur.div_f32(3.14), Duration::new(0, 859_872_576));
    /// // note that truncation is used, not rounding
    /// assert_eq!(dur.div_f32(3.14e5), Duration::new(0, 8_598));
    /// ```
    #[stable(feature = "duration_float", since = "1.38.0")]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    #[rustc_const_unstable(feature = "duration_consts_2", issue = "72440")]
    pub const fn div_f32(self, rhs: f32) -> Duration {
        Duration::from_secs_f32(self.as_secs_f32() / rhs)
    }

    /// Divide `Duration` by `Duration` and return `f64`.
    ///
    /// # Examples
    /// ```
    /// #![feature(div_duration)]
    /// use std::time::Duration;
    ///
    /// let dur1 = Duration::new(2, 700_000_000);
    /// let dur2 = Duration::new(5, 400_000_000);
    /// assert_eq!(dur1.div_duration_f64(dur2), 0.5);
    /// ```
    #[unstable(feature = "div_duration", issue = "63139")]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    #[rustc_const_unstable(feature = "duration_consts_2", issue = "72440")]
    pub const fn div_duration_f64(self, rhs: Duration) -> f64 {
        self.as_secs_f64() / rhs.as_secs_f64()
    }

    /// Divide `Duration` by `Duration` and return `f32`.
    ///
    /// # Examples
    /// ```
    /// #![feature(div_duration)]
    /// use std::time::Duration;
    ///
    /// let dur1 = Duration::new(2, 700_000_000);
    /// let dur2 = Duration::new(5, 400_000_000);
    /// assert_eq!(dur1.div_duration_f32(dur2), 0.5);
    /// ```
    #[unstable(feature = "div_duration", issue = "63139")]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    #[rustc_const_unstable(feature = "duration_consts_2", issue = "72440")]
    pub const fn div_duration_f32(self, rhs: Duration) -> f32 {
        self.as_secs_f32() / rhs.as_secs_f32()
    }
}

#[stable(feature = "duration", since = "1.3.0")]
impl Add for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Duration {
        self.checked_add(rhs).expect("overflow when adding durations")
    }
}

#[stable(feature = "time_augmented_assignment", since = "1.9.0")]
impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

#[stable(feature = "duration", since = "1.3.0")]
impl Sub for Duration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Duration {
        self.checked_sub(rhs).expect("overflow when subtracting durations")
    }
}

#[stable(feature = "time_augmented_assignment", since = "1.9.0")]
impl SubAssign for Duration {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

#[stable(feature = "duration", since = "1.3.0")]
impl Mul<u32> for Duration {
    type Output = Duration;

    fn mul(self, rhs: u32) -> Duration {
        self.checked_mul(rhs).expect("overflow when multiplying duration by scalar")
    }
}

#[stable(feature = "symmetric_u32_duration_mul", since = "1.31.0")]
impl Mul<Duration> for u32 {
    type Output = Duration;

    fn mul(self, rhs: Duration) -> Duration {
        rhs * self
    }
}

#[stable(feature = "time_augmented_assignment", since = "1.9.0")]
impl MulAssign<u32> for Duration {
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

#[stable(feature = "duration", since = "1.3.0")]
impl Div<u32> for Duration {
    type Output = Duration;

    fn div(self, rhs: u32) -> Duration {
        self.checked_div(rhs).expect("divide by zero error when dividing duration by scalar")
    }
}

#[stable(feature = "time_augmented_assignment", since = "1.9.0")]
impl DivAssign<u32> for Duration {
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs;
    }
}

macro_rules! sum_durations {
    ($iter:expr) => {{
        let mut total_secs: u64 = 0;
        let mut total_nanos: u64 = 0;

        for entry in $iter {
            total_secs =
                total_secs.checked_add(entry.secs).expect("overflow in iter::sum over durations");
            total_nanos = match total_nanos.checked_add(entry.nanos as u64) {
                Some(n) => n,
                None => {
                    total_secs = total_secs
                        .checked_add(total_nanos / NANOS_PER_SEC as u64)
                        .expect("overflow in iter::sum over durations");
                    (total_nanos % NANOS_PER_SEC as u64) + entry.nanos as u64
                }
            };
        }
        total_secs = total_secs
            .checked_add(total_nanos / NANOS_PER_SEC as u64)
            .expect("overflow in iter::sum over durations");
        total_nanos = total_nanos % NANOS_PER_SEC as u64;
        Duration { secs: total_secs, nanos: total_nanos as u32 }
    }};
}

#[stable(feature = "duration_sum", since = "1.16.0")]
impl Sum for Duration {
    fn sum<I: Iterator<Item = Duration>>(iter: I) -> Duration {
        sum_durations!(iter)
    }
}

#[stable(feature = "duration_sum", since = "1.16.0")]
impl<'a> Sum<&'a Duration> for Duration {
    fn sum<I: Iterator<Item = &'a Duration>>(iter: I) -> Duration {
        sum_durations!(iter)
    }
}