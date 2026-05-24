use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
#[cfg(feature = "nightly")]
use example_simd::simd_std;
use example_simd::{scalar, scalar_unrolled, simd_wide};
use rand::Rng;

fn generate_data(size: usize) -> (Vec<f32>, Vec<f32>) {
    let mut rng = rand::thread_rng();
    let a: Vec<f32> = (0..size).map(|_| rng.gen::<f32>()).collect();
    let b: Vec<f32> = (0..size).map(|_| rng.gen::<f32>()).collect();
    (a, b)
}

fn benchmark_dot_product(c: &mut Criterion) {
    let sizes = [1024, 4096, 16384, 65536, 262144];

    let mut group = c.benchmark_group("dot_product");

    for size in sizes {
        let (a, b) = generate_data(size);
        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(
            BenchmarkId::new("scalar", size),
            &(&a, &b),
            |bench, (a, b)| {
                bench.iter(|| scalar::dot_product::dot_product(black_box(a), black_box(b)))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("scalar_unrolled", size),
            &(&a, &b),
            |bench, (a, b)| {
                bench.iter(|| scalar_unrolled::dot_product::dot_product(black_box(a), black_box(b)))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("simd_wide", size),
            &(&a, &b),
            |bench, (a, b)| {
                bench.iter(|| simd_wide::dot_product::dot_product(black_box(a), black_box(b)))
            },
        );

        #[cfg(feature = "nightly")]
        group.bench_with_input(
            BenchmarkId::new("simd_std", size),
            &(&a, &b),
            |bench, (a, b)| {
                bench.iter(|| simd_std::dot_product::dot_product(black_box(a), black_box(b)))
            },
        );
    }

    group.finish();
}

criterion_group!(benches, benchmark_dot_product);
criterion_main!(benches);
