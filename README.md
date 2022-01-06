# Minstant
[![Actions Status](https://github.com/tikv/minstant/workflows/CI/badge.svg)](https://github.com/tikv/minstant/actions)
[![Build Status](https://travis-ci.org/tikv/minstant.svg?branch=master)](https://travis-ci.org/tikv/minstant)
[![LICENSE](https://img.shields.io/github/license/tikv/minstant.svg)](https://github.com/tikv/minstant/blob/master/LICENSE)

A drop-in replacement for [`std::time::Instant`](https://doc.rust-lang.org/std/time/struct.Instant.html) that measures time with high performance and high accuracy powered by [TSC](https://en.wikipedia.org/wiki/Time_Stamp_Counter).

## Usage

```toml
[dependencies]
minstant = { git = "https://github.com/tikv/minstant.git", branch = "master" }
```

```rust
let start = minstant::Instant::now();

// Code snipppet to measure

let duration: std::time::Duration = start.elapsed();
```


## Motivation

This library is used by a high performance tracing library [`minitrace-rust`](https://github.com/tikv/minitrace-rust). The main purpose is to use [TSC](https://en.wikipedia.org/wiki/Time_Stamp_Counter) on x86 processors to measure time at high speed without losing much accuracy.

## Platform Support

Currently, only the Linux on `x86` or `x86_64` is backed by [TSC](https://en.wikipedia.org/wiki/Time_Stamp_Counter). On other platforms, `minstant` falls back to coarse time. If TSC is unstable, it will also fall back to coarse time.

## Benchmark

Benchmark platform is `Intel(R) Xeon(R) CPU E5-2630 v4 @ 2.20GHz` on CentOS 7.

```sh
> cargo bench

minstant::Instant::now()
                        time:   [10.436 ns 10.536 ns 10.681 ns]
                        change: [-0.3585% +0.2484% +0.9376%] (p = 0.46 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe

quanta::Instant::now()  time:   [31.862 ns 31.944 ns 32.031 ns]
                        change: [-0.5409% -0.0894% +0.3226%] (p = 0.70 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

std::Instant::now()     time:   [26.273 ns 26.439 ns 26.652 ns]
                        change: [-1.4857% -0.1542% +1.3351%] (p = 0.84 > 0.05)
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  1 (1.00%) low severe
  8 (8.00%) low mild
  6 (6.00%) high severe

minstant::Anchor::new() time:   [46.674 ns 46.878 ns 47.143 ns]
                        change: [-0.6831% -0.3151% +0.1095%] (p = 0.13 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

minstant::Instant::as_unix_nanos()
                        time:   [15.377 ns 15.426 ns 15.476 ns]
                        change: [-0.3004% +0.1117% +0.5448%] (p = 0.61 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe
```
