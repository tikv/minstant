use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_now(c: &mut Criterion) {
    c.bench_function("now", |b| {
        b.iter(minstant::Instant::now);
    });
}

fn bench_std_now(c: &mut Criterion) {
    c.bench_function("std now", |b| {
        b.iter(std::time::Instant::now);
    });
}

fn bench_unix_time(c: &mut Criterion) {
    let anchor = minstant::Anchor::new();
    c.bench_function("unix_time", |b| {
        b.iter(|| {
            black_box(minstant::Instant::now().as_unix_nanos(&anchor));
        });
    });
}

criterion_group!(benches, bench_now, bench_std_now, bench_unix_time);
criterion_main!(benches);
