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

This library is used by an high performance tracing library [`minitrace-rust`](https://github.com/tikv/minitrace-rust). The main purpose is to use [TSC](https://en.wikipedia.org/wiki/Time_Stamp_Counter) on x86 processors to measure time at high speed without losing much accuracy.

## Platform Support

Currently, only the Linux on `x86` or `x86_64` is backed by [TSC](https://en.wikipedia.org/wiki/Time_Stamp_Counter). On other platforms, `minstant` falls back to coarse time. If TSC is unstable, it will also fall back to coarse time.

## Benchmark

Benchmark platform is `Intel(R) Xeon(R) CPU E5-2630 v4 @ 2.20GHz` on CentOS 7.

```sh
> cargo bench

minstant::Instant::now()
            time:   [10.463 ns 10.496 ns 10.533 ns]
            change: [-2.1521% -1.0827% -0.2262%] (p = 0.02 < 0.05)
            Change within noise threshold.

std::Instant::now()
            time:   [26.720 ns 26.855 ns 27.026 ns]
            change: [-0.1996% +0.2410% +0.7321%] (p = 0.32 > 0.05)
            No change in performance detected.

minstant::Instant::as_unix_nanos()
            time:   [15.364 ns 15.456 ns 15.574 ns]
            change: [+0.1835% +0.6363% +1.1179%] (p = 0.01 < 0.05)
            Change within noise threshold.
```
