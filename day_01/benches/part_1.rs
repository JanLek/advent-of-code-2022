use criterion::{criterion_group, criterion_main, Criterion};
use day_01::{part_1, part_2};

fn part_1_benchmark(c: &mut Criterion) {
    c.bench_function("part_1", |b| b.iter(part_1));
}

fn part_2_benchmark(c: &mut Criterion) {
    c.bench_function("part_2", |b| b.iter(part_2));
}

criterion_group!(benches, part_1_benchmark, part_2_benchmark);
criterion_main!(benches);