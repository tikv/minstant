# Minstant

A Rust library to measure time with high performance.


## Usage

```toml
[dependencies]
minstant = { git = "https://github.com/zhongzc/minstant.git" }
```

```rust
let start = minstant::now();

// Code snipppet to measure

let end = minstant::now();

let elapsed_nanos = (end - start) as f64 * *minstant::NANOS_PER_CYCLE;
```


## Motivation

The main purpose is to use [TSC](https://en.wikipedia.org/wiki/Time_Stamp_Counter) on x86 processors to measure time at high speed without losing much accuracy. If TSC is inaccessible (on non-x86 systems) or unreliable, it will fallback to coarse time.
