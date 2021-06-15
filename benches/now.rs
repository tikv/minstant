use std::time::{SystemTime, UNIX_EPOCH};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Copy, Clone)]
pub struct Anchor {
    unix_time_ns: u64,
    cycle: u64,
    nanos_per_cycle: f64,
}

impl Default for Anchor {
    fn default() -> Self {
        Self::new()
    }
}

impl Anchor {
    #[inline]
    pub fn new() -> Anchor {
        let unix_time_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("unexpected time drift")
            .as_nanos() as u64;
        Anchor {
            unix_time_ns,
            cycle: minstant::instant::Instant::now().into(),
            nanos_per_cycle: minstant::nanos_per_cycle(),
        }
    }

    pub fn cycle_to_unix_nanos(&self, cycle: u64) -> u64 {
        if cycle > self.cycle {
            let forward_ns = (cycle - self.cycle) as f64 * self.nanos_per_cycle;
            self.unix_time_ns + forward_ns as u64
        } else {
            let backward_ns = (self.cycle - cycle) as f64 * self.nanos_per_cycle;
            self.unix_time_ns - backward_ns as u64
        }
    }
}

fn bench_now(c: &mut Criterion) {
    c.bench_function("now", |b| {
        b.iter(minstant::instant::Instant::now);
    });
}

fn bench_std_now(c: &mut Criterion) {
    c.bench_function("std now", |b| {
        b.iter(std::time::Instant::now);
    });
}

fn bench_unix_time(c: &mut Criterion) {
    let anchor = Anchor::new();
    c.bench_function("unix_time", |b| {
        b.iter(|| {
            black_box(anchor.cycle_to_unix_nanos(minstant::instant::Instant::now().into()));
        });
    });
}

criterion_group!(benches, bench_now, bench_std_now, bench_unix_time);
criterion_main!(benches);
