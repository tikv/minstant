// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

use minstant_macro::timing;

#[timing(|elapsed_cycles| *statistic += elapsed_cycles)]
fn heavy_load(statistic: &mut u64) {
    std::thread::sleep(std::time::Duration::from_secs(1));
}

#[test]
fn test_macro() {
    let start = std::time::Instant::now();
    let mut statistic = 0;
    heavy_load(&mut statistic);
    let std_nanos = start.elapsed().as_nanos() as i64;
    let minstant_nanos = (statistic as f64 * minstant::nanos_per_cycle()) as i64;
    assert!((minstant_nanos - std_nanos).abs() < 10000);
}
