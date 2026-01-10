# example-simd

SIMD performance comparison in Rust.

## Implementations

| Directory | Description | Rust |
|-----------|-------------|------|
| `scalar` | Iterator-based | stable |
| `scalar_unrolled` | Manual loop unrolling | stable |
| `simd_wide` | wide crate | stable |
| `simd_std` | std::simd | nightly |

## Algorithms

- Array sum
- Dot product
- Euclidean distance

## Run Benchmarks

```bash
# stable (wide crate)
RUSTFLAGS="-C target-cpu=native" cargo bench

# nightly (includes std::simd)
RUSTFLAGS="-C target-cpu=native" cargo +nightly bench --features nightly
```

## Benchmark Results (262,144 elements)

### Array Sum

| Implementation | Time | Speedup |
|----------------|------|---------|
| scalar | 134 µs | 1.0x |
| scalar_unrolled | 24 µs | 5.7x |
| simd_wide | 19 µs | 7.0x |
| simd_std | 19 µs | 7.0x |

### Dot Product

| Implementation | Time | Speedup |
|----------------|------|---------|
| scalar | 141 µs | 1.0x |
| scalar_unrolled | 23 µs | 6.2x |
| simd_wide | 23 µs | 6.0x |
| simd_std | 23 µs | 6.2x |

### Euclidean Distance

| Implementation | Time | Speedup |
|----------------|------|---------|
| scalar | 138 µs | 1.0x |
| scalar_unrolled | 24 µs | 5.9x |
| simd_wide | 23 µs | 5.9x |
| simd_std | 24 µs | 5.8x |

## Why is scalar_unrolled fast?

Loop unrolling removes data dependencies, possibly enabling the compiler to auto-vectorize.
