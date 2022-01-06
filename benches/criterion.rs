use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_nows(c: &mut Criterion) {
    // The first call will take some time for calibartion
    quanta::Instant::now();

    let mut group = c.benchmark_group("Instant::now()");
    group.bench_function("minstant", |b| {
        b.iter(minstant::Instant::now);
    });
    group.bench_function("quanta", |b| {
        b.iter(quanta::Instant::now);
    });
    group.bench_function("std", |b| {
        b.iter(std::time::Instant::now);
    });
    group.finish();
}

fn bench_anchor_new(c: &mut Criterion) {
    c.bench_function("minstant::Anchor::new()", |b| {
        b.iter(minstant::Anchor::new);
    });
}

fn bench_as_unix_nanos(c: &mut Criterion) {
    let anchor = minstant::Anchor::new();
    c.bench_function("minstant::Instant::as_unix_nanos()", |b| {
        b.iter(|| {
            black_box(minstant::Instant::now().as_unix_nanos(&anchor));
        });
    });
}

criterion_group!(benches, bench_nows, bench_anchor_new, bench_as_unix_nanos);
criterion_main!(benches);
