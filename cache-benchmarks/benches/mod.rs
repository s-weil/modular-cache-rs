mod concurrent_timed_cache;
mod timed_cache;

use criterion::{criterion_group, criterion_main};

criterion_group!(
    benches,
    // criterion_timed_cache_v1,
    // criterion_timed_cache_v2,
    timed_cache::criterion_timed_cache_comparison,
    timed_cache::criterion_timed_cache_comparison2,
    concurrent_timed_cache::criterion_timed_cache_comparison_parallel,
    concurrent_timed_cache::criterion_timed_cache_comparison2_parallel
);
criterion_main!(benches);
