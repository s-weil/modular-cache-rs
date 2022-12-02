use cache_benchmarks::{queued_cache_parallel, queued_lookup_cache_parallel};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

criterion_group!(
    benches,
    // concurrent_queued_cache_comparison,
    concurrent_queued_cache_comparison_bounded,
);
criterion_main!(benches);

pub fn concurrent_queued_cache_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("Concurrent Queued Cache Comparison");

    group.bench_function(
        "Simple: n_keys: high, value_size: low, max_capacity: high",
        |b| b.iter(|| queued_cache_parallel(black_box((10_000, 5_000, 2)))),
    );

    group.bench_function(
        "Lookup: n_keys: high, value_size: low, max_capacity: high",
        |b| b.iter(|| queued_lookup_cache_parallel(black_box((10_000, 5_000, 2)))),
    );

    group.finish()
}

pub fn concurrent_queued_cache_comparison_bounded(c: &mut Criterion) {
    let mut group = c.benchmark_group("Concurrent Queued Cache Comparison");

    group.bench_function(
        "Simple: n_keys: high, value_size: low, max_capacity: high",
        |b| b.iter(|| queued_cache_parallel(black_box((5_000, 10_000, 2)))),
    );

    group.bench_function(
        "Lookup: n_keys: high, value_size: low, max_capacity: high",
        |b| b.iter(|| queued_lookup_cache_parallel(black_box((5_000, 10_000, 2)))),
    );

    group.finish()
}
