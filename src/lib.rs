// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

#[cfg(test)]
mod test;

pub mod duration;
pub mod instant;
mod utils;

#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
mod tsc_now;
