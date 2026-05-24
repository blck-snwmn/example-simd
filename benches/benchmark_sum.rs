use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
#[cfg(feature = "nightly")]
use example_simd::simd_std;
use example_simd::{scalar, scalar_unrolled, simd_wide};
use rand::Rng;

fn generate_data(size: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen::<f32>()).collect()
}

fn benchmark_sum(c: &mut Criterion) {
    let sizes = [1024, 4096, 16384, 65536, 262144];

    let mut group = c.benchmark_group("array_sum");

    for size in sizes {
        let data = generate_data(size);
        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(BenchmarkId::new("scalar", size), &data, |b, data| {
            b.iter(|| scalar::sum::sum(black_box(data)))
        });

        group.bench_with_input(
            BenchmarkId::new("scalar_unrolled", size),
            &data,
            |b, data| b.iter(|| scalar_unrolled::sum::sum(black_box(data))),
        );

        group.bench_with_input(BenchmarkId::new("simd_wide", size), &data, |b, data| {
            b.iter(|| simd_wide::sum::sum(black_box(data)))
        });

        #[cfg(feature = "nightly")]
        group.bench_with_input(BenchmarkId::new("simd_std", size), &data, |b, data| {
            b.iter(|| simd_std::sum::sum(black_box(data)))
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_sum);
criterion_main!(benches);
