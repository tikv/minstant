use criterion::{black_box, criterion_group, criterion_main, Criterion};
use minstant::instant::Instant;

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
    let start = Instant::now();
    c.bench_function("unix_time", |b| {
        b.iter(|| {
            black_box(start.elapsed());
        });
    });
}

criterion_group!(benches, bench_now, bench_std_now, bench_unix_time);
criterion_main!(benches);
