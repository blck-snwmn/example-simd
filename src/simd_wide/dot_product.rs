use wide::f32x8;

/// Dot product (SIMD, wide crate)
#[inline]
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    let a_chunks = a.chunks_exact(8);
    let b_chunks = b.chunks_exact(8);
    let a_remainder = a_chunks.remainder();
    let b_remainder = b_chunks.remainder();

    let mut acc = f32x8::ZERO;

    for (a_chunk, b_chunk) in a_chunks.zip(b_chunks) {
        let va = f32x8::new(a_chunk.try_into().unwrap());
        let vb = f32x8::new(b_chunk.try_into().unwrap());
        acc += va * vb;
    }

    // horizontal sum
    let arr: [f32; 8] = acc.into();
    let simd_sum: f32 = arr.iter().sum();

    let remainder_sum: f32 = a_remainder
        .iter()
        .zip(b_remainder.iter())
        .map(|(x, y)| x * y)
        .sum();

    simd_sum + remainder_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        assert!((dot_product(&a, &b) - 32.0).abs() < 1e-6);
    }

    #[test]
    fn test_dot_product_large() {
        let a: Vec<f32> = (1..=100).map(|x| x as f32).collect();
        let b: Vec<f32> = vec![1.0; 100];
        // sum of 1 to 100 = 5050
        assert!((dot_product(&a, &b) - 5050.0).abs() < 1e-3);
    }

    #[test]
    fn test_dot_product_empty() {
        let a: Vec<f32> = vec![];
        let b: Vec<f32> = vec![];
        assert!((dot_product(&a, &b) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_dot_product_exact_chunk() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let b = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        assert!((dot_product(&a, &b) - 36.0).abs() < 1e-6);
    }
}
