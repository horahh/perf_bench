use criterion::{criterion_group, criterion_main, Criterion};
use perf_bench::perf_bench;

fn my_benchmark_function(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-10");

    group.significance_level(0.1).sample_size(10);
    group.bench_function("perf_bench", |b| {
        b.iter(|| {
            // Code to be benchmarked
            let _ = perf_bench();
        });
    });
    group.finish();
}

criterion_group!(benches, my_benchmark_function);
criterion_main!(benches);
