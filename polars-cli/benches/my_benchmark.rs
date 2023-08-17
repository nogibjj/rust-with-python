use criterion::{criterion_group, criterion_main, Criterion};
use polars_cli::calculate;

// This function benchmarks the `calculate` function of your project
fn benchmark_calculate(c: &mut Criterion) {
    c.bench_function("calculate", |b| {
        b.iter(|| calculate())  // `calculate` is the function you want to benchmark
    });
}

// Create a Criterion group for organizing related benchmarks
criterion_group!(benches, benchmark_calculate);

// The `main` function of the benchmark
criterion_main!(benches);