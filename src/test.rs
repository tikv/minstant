use crate::duration;

use super::*;
use rand::Rng;
use std::{time, u128};

#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
use super::tsc_now::*;

#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
#[test]
fn test_tsc_available() {
    if tsc_available() {
        println!("TSC available")
    }
}

#[test]
fn test_monotonic() {
    let mut prev = instant::Instant::now();
    for _ in 0..10000 {
        let cur = instant::Instant::now();
        assert!(cur >= prev);
        prev = cur;
    }
}
#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
#[test]
fn test_nanos_per_cycle() {
    let _ = nanos_per_cycle();
}

#[test]
fn test_duration() {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let i_minstant_start = instant::Instant::now();
        let i_std_start = time::Instant::now();
        let sleep_nanos = rng.gen_range(100_000_000..500_000_000);
        std::thread::sleep(time::Duration::from_nanos(sleep_nanos));
        let check = move || {
            let i_minstant_end = instant::Instant::now();
            let i_std_end = time::Instant::now();
            let dur_ns_minstant = (i_minstant_end - i_minstant_start).as_nanos();
            let dur_ns_std = (i_std_end - i_std_start).as_nanos();
            println!(
                "sleep_nanos: {:?}, dur minstant: {:?}, dur std: {:?}",
                sleep_nanos, dur_ns_minstant, dur_ns_std
            );

            #[cfg(target_os = "windows")]
            let expect_max_delta_ns = 20_000_000;
            #[cfg(not(target_os = "windows"))]
            let expect_max_delta_ns = 5_000_000;

            assert!((dur_ns_minstant as i128 - dur_ns_std as i128).abs() < expect_max_delta_ns);
        };
        check();
        std::thread::spawn(check).join().expect("join failed");
    }
}

/// tests from `tsc_now`

#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
#[test]
fn test_parse_list_format() {
    assert_eq!(
        parse_cpu_list_format("0-2,7,12-14\n").unwrap().into_vec(),
        &[0, 1, 2, 7, 12, 13, 14]
    );
    assert_eq!(
        parse_cpu_list_format("0-4,9\n").unwrap().into_vec(),
        &[0, 1, 2, 3, 4, 9]
    );
    assert_eq!(
        parse_cpu_list_format("0-15\n").unwrap().into_vec(),
        (0..=15).collect::<Vec<_>>()
    );
}

/// tests from `coarse_now`
#[test]
fn test_now() {
    let mut prev = instant::Instant::now();
    for _ in 0..100 {
        let n = instant::Instant::now();
        assert!(n >= prev);
        prev = n;
    }
}

// duration tests
#[test]
fn test_duration_ty() {
    assert_eq!(
        duration::Duration::from(time::Duration::from_secs(114514)),
        duration::Duration::from_secs(114514)
    );
    assert_eq!(
        duration::Duration::from(time::Duration::from_millis(114514)),
        duration::Duration::from_millis(114514)
    );
    assert_eq!(
        duration::Duration::from(time::Duration::from_nanos(114514)),
        duration::Duration::from_nanos(114514)
    );
    assert_eq!(
        time::Duration::from_secs(114514).as_nanos(),
        duration::Duration::from_secs(114514).as_nanos()
    );
    assert_eq!(
        time::Duration::from_millis(114514).as_secs(),
        duration::Duration::from_millis(114514).as_secs()
    );
}
