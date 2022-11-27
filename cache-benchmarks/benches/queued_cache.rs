use cache_benchmarks::{queued_cache_sequential, queued_lookup_cache_sequential};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(
    benches,
    // criterion_timed_cache_v1,
    // criterion_timed_cache_v2,
    criterion_timed_cache_comparison,
    criterion_timed_cache_comparison2,
);
criterion_main!(benches);

pub fn criterion_timed_cache_v1(c: &mut Criterion) {
    let mut group = c.benchmark_group("Evaluate the performance of timed cache implementations");

    group.bench_function(
        "TimedCache - v1: n_keys: small, value_size: small, max_capacity: small",
        |b| b.iter(|| queued_cache_sequential(black_box((100, 10, 30)))),
    );

    group.bench_function(
        "TimedCache - v1: n_keys: high, value_size: small, max_capacity: high",
        |b| b.iter(|| queued_cache_sequential(black_box((10_000, 10, 1_000)))),
    );

    group.bench_function(
        "TimedCache - v1: n_keys: high, value_size: high, max_capacity: high",
        |b| b.iter(|| queued_cache_sequential(black_box((10_000, 1_000, 5_000)))),
    );

    group.finish()
}

pub fn criterion_timed_cache_v2(c: &mut Criterion) {
    let mut group = c.benchmark_group("Evaluate the performance of timed cache implementations");

    group.bench_function(
        "TimedCache - v2: n_keys: small, value_size: small, max_capacity: small",
        |b| b.iter(|| queued_lookup_cache_sequential(black_box((100, 10, 30)))),
    );

    group.bench_function(
        "TimedCache - v2: n_keys: high, value_size: small, max_capacity: high",
        |b| b.iter(|| queued_lookup_cache_sequential(black_box((10_000, 10, 1_000)))),
    );

    group.bench_function(
        "TimedCache - v2: n_keys: high, value_size: high, max_capacity: high",
        |b| b.iter(|| queued_lookup_cache_sequential(black_box((10_000, 1_000, 5_000)))),
    );

    group.finish()
}

pub fn criterion_timed_cache_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("Evaluate the performance of timed cache implementations");

    group.bench_function(
        "TimedCache - comparison - v1: n_keys: high, value_size: low, max_capacity: high",
        |b| b.iter(|| queued_cache_sequential(black_box((10_000, 5_000, 2)))),
    );

    group.bench_function(
        "TimedCache - comparison - v2: n_keys: high, value_size: low, max_capacity: high",
        |b| b.iter(|| queued_lookup_cache_sequential(black_box((10_000, 5_000, 2)))),
    );

    group.finish()
}

pub fn criterion_timed_cache_comparison2(c: &mut Criterion) {
    let mut group = c.benchmark_group("Evaluate the performance of timed cache implementations");

    group.bench_function(
        "TimedCache - comparison - v1: n_keys: high, value_size: low, max_capacity: high",
        |b| b.iter(|| queued_cache_sequential(black_box((10_000, 20_000, 2)))),
    );

    group.bench_function(
        "TimedCache - comparison - v2: n_keys: high, value_size: low, max_capacity: high",
        |b| b.iter(|| queued_lookup_cache_sequential(black_box((10_000, 20_000, 2)))),
    );

    group.finish()
}
