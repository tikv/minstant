use super::*;
use rand::Rng;
use std::time::{Duration, Instant};

#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
use super::tsc_now;

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
            let duration_ns_minstant = (now() - cur_cycle) as f64 * nanos_per_cycle();
            let duration_ns_std = Instant::now().duration_since(cur_instant).as_nanos();

            #[cfg(target_os = "windows")]
            let expect_max_delta_ns = 20_000_000.0;
            #[cfg(not(target_os = "windows"))]
            let expect_max_delta_ns = 5_000_000.0;

            let real_delta = (duration_ns_std as f64 - duration_ns_minstant).abs();
            assert!(
                real_delta < expect_max_delta_ns,
                "real delta: {}",
                real_delta
            );
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
        parse_cpu_list_format("0-2,7,12-14\n").unwrap(),
        &[0, 1, 2, 7, 12, 13, 14]
    );
    assert_eq!(
        parse_cpu_list_format("0-4,9\n").unwrap(),
        &[0, 1, 2, 3, 4, 9]
    );
    assert_eq!(
        parse_cpu_list_format("0-15\n").unwrap(),
        (0..=15).collect::<Vec<_>>()
    );
}

/// tests from `coarse_now`
#[test]
fn test_now() {
    let mut prev = now();
    for _ in 0..100 {
        let n = now();
        assert!(n >= prev);
        prev = n;
    }
}
