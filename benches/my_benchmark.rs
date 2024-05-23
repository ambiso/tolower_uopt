use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use tolower_uopt::*;

fn criterion_benchmark(c: &mut Criterion) {
    for size in [10, 32, 4096] {
        let s: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(size)
            .map(char::from)
            .collect();
        c.bench_function(&format!("schoen {size}"), |b| {
            b.iter_batched(
                || s.clone().into_bytes(),
                |mut sc| tolower_schoen_array(black_box(&mut sc)),
                criterion::BatchSize::PerIteration,
            )
        });
        c.bench_function(&format!("switch {size}"), |b| {
            b.iter_batched(
                || s.clone().into_bytes(),
                |mut sc| tolower_switch_array(black_box(&mut sc)),
                criterion::BatchSize::PerIteration,
            )
        });
        c.bench_function(&format!("lut {size}"), |b| {
            b.iter_batched(
                || s.clone().into_bytes(),
                |mut sc| tolower_lut_array(black_box(&mut sc)),
                criterion::BatchSize::PerIteration,
            )
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
