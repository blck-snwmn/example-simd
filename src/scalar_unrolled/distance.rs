/// Euclidean distance (manual loop unrolling)
#[inline]
pub fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
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
        let d0 = a_chunk[0] - b_chunk[0];
        let d1 = a_chunk[1] - b_chunk[1];
        let d2 = a_chunk[2] - b_chunk[2];
        let d3 = a_chunk[3] - b_chunk[3];
        let d4 = a_chunk[4] - b_chunk[4];
        let d5 = a_chunk[5] - b_chunk[5];
        let d6 = a_chunk[6] - b_chunk[6];
        let d7 = a_chunk[7] - b_chunk[7];

        sum0 += d0 * d0;
        sum1 += d1 * d1;
        sum2 += d2 * d2;
        sum3 += d3 * d3;
        sum4 += d4 * d4;
        sum5 += d5 * d5;
        sum6 += d6 * d6;
        sum7 += d7 * d7;
    }

    let remainder_sum: f32 = a_remainder
        .iter()
        .zip(b_remainder.iter())
        .map(|(x, y)| {
            let diff = x - y;
            diff * diff
        })
        .sum();

    (sum0 + sum1 + sum2 + sum3 + sum4 + sum5 + sum6 + sum7 + remainder_sum).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        let a = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let b = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        // sqrt(10) ≈ 3.162
        assert!((euclidean_distance(&a, &b) - 10.0_f32.sqrt()).abs() < 1e-6);
    }

    #[test]
    fn test_euclidean_distance_same_point() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        assert!((euclidean_distance(&a, &b) - 0.0).abs() < 1e-6);
    }
}
