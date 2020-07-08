use criterion::{criterion_group, criterion_main, Criterion};

fn bench_now(c: &mut Criterion) {
    c.bench_function("now", |b| {
        b.iter(minstant::now);
    });
}

criterion_group!(benches, bench_now);
criterion_main!(benches);
