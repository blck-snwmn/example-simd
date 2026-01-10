/// Dot product (manual loop unrolling)
#[inline]
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    let mut sum0 = 0.0f32;
    let mut sum1 = 0.0f32;
    let mut sum2 = 0.0f32;
    let mut sum3 = 0.0f32;
    let mut sum4 = 0.0f32;
    let mut sum5 = 0.0f32;
    let mut sum6 = 0.0f32;
    let mut sum7 = 0.0f32;

    let a_chunks = a.chunks_exact(8);
    let b_chunks = b.chunks_exact(8);
    let a_remainder = a_chunks.remainder();
    let b_remainder = b_chunks.remainder();

    for (a_chunk, b_chunk) in a_chunks.zip(b_chunks) {
        sum0 += a_chunk[0] * b_chunk[0];
        sum1 += a_chunk[1] * b_chunk[1];
        sum2 += a_chunk[2] * b_chunk[2];
        sum3 += a_chunk[3] * b_chunk[3];
        sum4 += a_chunk[4] * b_chunk[4];
        sum5 += a_chunk[5] * b_chunk[5];
        sum6 += a_chunk[6] * b_chunk[6];
        sum7 += a_chunk[7] * b_chunk[7];
    }

    let remainder_sum: f32 = a_remainder
        .iter()
        .zip(b_remainder.iter())
        .map(|(x, y)| x * y)
        .sum();

    sum0 + sum1 + sum2 + sum3 + sum4 + sum5 + sum6 + sum7 + remainder_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let b = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        // 1+2+3+4+5+6+7+8+9+10 = 55
        assert!((dot_product(&a, &b) - 55.0).abs() < 1e-6);
    }

    #[test]
    fn test_dot_product_empty() {
        let a: Vec<f32> = vec![];
        let b: Vec<f32> = vec![];
        assert!((dot_product(&a, &b) - 0.0).abs() < 1e-6);
    }
}
