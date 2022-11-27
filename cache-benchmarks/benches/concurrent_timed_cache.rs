use cache_benchmarks::{timed_cache_parallel, timed_cache_v2_parallel};
use criterion::{black_box, Criterion};

pub fn criterion_timed_cache_comparison_parallel(c: &mut Criterion) {
    let mut group =
        c.benchmark_group("Evaluate the performance of timed cache implementations in paralell");

    group.bench_function(
        "TimedCache - concurrent comparison - v1: n_keys: high, value_size: low, max_capacity: high",
        |b| b.iter(|| timed_cache_parallel(black_box((10_000, 5_000, 2)))),
    );

    group.bench_function(
        "TimedCache - concurrent comparison - v2: n_keys: high, value_size: low, max_capacity: high",
        |b| b.iter(|| timed_cache_v2_parallel(black_box((10_000, 5_000, 2)))),
    );

    group.finish()
}

pub fn criterion_timed_cache_comparison2_parallel(c: &mut Criterion) {
    let mut group =
        c.benchmark_group("Evaluate the performance of timed cache implementations in paralell");

    group.bench_function(
        "TimedCache - concurrent comparison - v1: n_keys: high, value_size: low, max_capacity: high",
        |b| b.iter(|| timed_cache_parallel(black_box((10_000, 20_000, 2)))),
    );

    group.bench_function(
        "TimedCache - concurrent 2comparison - v2: n_keys: high, value_size: low, max_capacity: high",
        |b| b.iter(|| timed_cache_v2_parallel(black_box((10_000, 20_000, 2)))),
    );

    group.finish()
}
