// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

use minstant_macro::timing;

#[timing(|elapsed_cycles| *statistic += elapsed_cycles)]
fn heavy_load(statistic: &mut minstant::duration::Duration) {
    std::thread::sleep(std::time::Duration::from_secs(1));
}

#[test]
fn test_macro() {
    let start = std::time::Instant::now();
    let mut statistic = minstant::duration::Duration::default();
    heavy_load(&mut statistic);
    let std_nanos = start.elapsed().as_nanos() as i64;
    let minstant_nanos = statistic.as_nanos() as i64;
    println!("minstant: {:?}, std: {:?}", minstant_nanos, std_nanos);
    #[cfg(target_os = "windows")]
    let expect_max_delta_ns = 20_000_000;
    #[cfg(not(target_os = "windows"))]
    let expect_max_delta_ns = 5_000_000;
    assert!((minstant_nanos - std_nanos).abs() < expect_max_delta_ns);
}
