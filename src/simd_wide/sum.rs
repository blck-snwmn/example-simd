use wide::f32x8;

/// Array sum (SIMD, wide crate)
#[inline]
pub fn sum(data: &[f32]) -> f32 {
    let chunks = data.chunks_exact(8);
    let remainder = chunks.remainder();

    let mut acc = f32x8::ZERO;

    for chunk in chunks {
        let v = f32x8::new(chunk.try_into().unwrap());
        acc += v;
    }

    // horizontal sum
    let arr: [f32; 8] = acc.into();
    arr.iter().sum::<f32>() + remainder.iter().sum::<f32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert!((sum(&data) - 15.0).abs() < 1e-6);
    }

    #[test]
    fn test_sum_large() {
        let data: Vec<f32> = (1..=100).map(|x| x as f32).collect();
        // sum of 1 to 100 = 5050
        assert!((sum(&data) - 5050.0).abs() < 1e-3);
    }

    #[test]
    fn test_sum_empty() {
        let data: Vec<f32> = vec![];
        assert!((sum(&data) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_sum_exact_chunk() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        assert!((sum(&data) - 36.0).abs() < 1e-6);
    }
}
