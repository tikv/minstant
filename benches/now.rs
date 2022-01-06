use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_minstant_now(c: &mut Criterion) {
    c.bench_function("minstant::Instant::now()", |b| {
        b.iter(minstant::Instant::now);
    });
}

fn bench_quanta_now(c: &mut Criterion) {
    c.bench_function("minstant::Instant::now()", |b| {
        b.iter(quanta::Instant::now);
    });
}

fn bench_std_now(c: &mut Criterion) {
    c.bench_function("std::Instant::now()", |b| {
        b.iter(std::time::Instant::now);
    });
}

fn bench_anchor_new(c: &mut Criterion) {
    c.bench_function("minstant::Anchor::new()", |b| {
        b.iter(minstant::Anchor::new);
    });
}

fn bench_unix_time(c: &mut Criterion) {
    let anchor = minstant::Anchor::new();
    c.bench_function("minstant::Instant::as_unix_nanos()", |b| {
        b.iter(|| {
            black_box(minstant::Instant::now().as_unix_nanos(&anchor));
        });
    });
}

criterion_group!(
    benches,
    bench_minstant_now,
    bench_quanta_now,
    bench_std_now,
    bench_anchor_new,
    bench_unix_time
);
criterion_main!(benches);
