# Minstant
[![Actions Status](https://github.com/tikv/minstant/workflows/CI/badge.svg)](https://github.com/tikv/minstant/actions)
[![Build Status](https://travis-ci.org/tikv/minstant.svg?branch=master)](https://travis-ci.org/tikv/minstant)
[![LICENSE](https://img.shields.io/github/license/tikv/minstant.svg)](https://github.com/tikv/minstant/blob/master/LICENSE)

A Rust library to measure time with high performance.


## Usage

```toml
[dependencies]
minstant = { git = "https://github.com/tikv/minstant.git", branch = "master" }
```

```rust
let start = minstant::now();

// Code snipppet to measure

let end = minstant::now();

let elapsed_nanos = (end - start) as f64 * minstant::nanos_per_cycle();
```


## Motivation

The main purpose is to use [TSC](https://en.wikipedia.org/wiki/Time_Stamp_Counter) on x86 processors to measure time at high speed without losing much accuracy. If TSC is inaccessible (on non-x86 systems) or unreliable, it will fallback to coarse time.
