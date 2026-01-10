use std::simd::{f32x8, num::SimdFloat};

/// Euclidean distance (std::simd)
#[inline]
pub fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    let a_chunks = a.chunks_exact(8);
    let b_chunks = b.chunks_exact(8);
    let a_remainder = a_chunks.remainder();
    let b_remainder = b_chunks.remainder();

    let mut acc = f32x8::splat(0.0);

    for (a_chunk, b_chunk) in a_chunks.zip(b_chunks) {
        let va = f32x8::from_slice(a_chunk);
        let vb = f32x8::from_slice(b_chunk);
        let diff = va - vb;
        acc += diff * diff;
    }

    let remainder_sum: f32 = a_remainder
        .iter()
        .zip(b_remainder.iter())
        .map(|(x, y)| {
            let diff = x - y;
            diff * diff
        })
        .sum();

    (acc.reduce_sum() + remainder_sum).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        let a = vec![0.0, 0.0, 0.0];
        let b = vec![3.0, 4.0, 0.0];
        // sqrt(9 + 16) = 5
        assert!((euclidean_distance(&a, &b) - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_euclidean_distance_large() {
        let a = vec![0.0; 100];
        let b = vec![1.0; 100];
        // sqrt(100) = 10
        assert!((euclidean_distance(&a, &b) - 10.0).abs() < 1e-5);
    }

    #[test]
    fn test_euclidean_distance_same_point() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        assert!((euclidean_distance(&a, &b) - 0.0).abs() < 1e-6);
    }
}
