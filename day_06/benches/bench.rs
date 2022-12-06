use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_06::{part_1, part_2};

const INPUT: &str = include_str!("../input.txt");

fn part_1_benchmark(c: &mut Criterion) {
    c.bench_function("part_1", |b| b.iter(|| part_1(black_box(INPUT))));
}

fn part_2_benchmark(c: &mut Criterion) {
    c.bench_function("part_2", |b| b.iter(|| part_2(black_box(INPUT))));
}

criterion_group!(benches, part_1_benchmark, part_2_benchmark);
criterion_main!(benches);
