# Minstant
[![Actions Status](https://github.com/tikv/minstant/workflows/CI/badge.svg)](https://github.com/tikv/minstant/actions)
[![Build Status](https://travis-ci.org/tikv/minstant.svg?branch=master)](https://travis-ci.org/tikv/minstant)
[![LICENSE](https://img.shields.io/github/license/tikv/minstant.svg)](https://github.com/tikv/minstant/blob/master/LICENSE)

A Rust library to measure time with high performance.

## Performance
left `minstant` , right `std::time`

![our](https://pbs.twimg.com/media/E4nkOszVkAUkazO?format=png) ![std](https://pbs.twimg.com/media/E4nkfHQVgAEYHtl?format=png)

## Usage

```toml
[dependencies]
minstant = { git = "https://github.com/tikv/minstant.git", branch = "master" }
```

similar to `std::time::Instance`

```rust
let start = Instance::now();
// do blablabla
let dur = start.elapsed();
```


## Motivation

The main purpose is to use [TSC](https://en.wikipedia.org/wiki/Time_Stamp_Counter) on x86 processors to measure time at high speed without losing much accuracy. If TSC is inaccessible (on non-x86 systems) or unreliable, it will fallback to coarse time.
