/// Array sum (manual loop unrolling)
#[inline]
pub fn sum(data: &[f32]) -> f32 {
    let mut sum0 = 0.0f32;
    let mut sum1 = 0.0f32;
    let mut sum2 = 0.0f32;
    let mut sum3 = 0.0f32;
    let mut sum4 = 0.0f32;
    let mut sum5 = 0.0f32;
    let mut sum6 = 0.0f32;
    let mut sum7 = 0.0f32;

    let chunks = data.chunks_exact(8);
    let remainder = chunks.remainder();

    for chunk in chunks {
        sum0 += chunk[0];
        sum1 += chunk[1];
        sum2 += chunk[2];
        sum3 += chunk[3];
        sum4 += chunk[4];
        sum5 += chunk[5];
        sum6 += chunk[6];
        sum7 += chunk[7];
    }

    sum0 + sum1 + sum2 + sum3 + sum4 + sum5 + sum6 + sum7 + remainder.iter().sum::<f32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        assert!((sum(&data) - 55.0).abs() < 1e-6);
    }

    #[test]
    fn test_sum_empty() {
        let data: Vec<f32> = vec![];
        assert!((sum(&data) - 0.0).abs() < 1e-6);
    }
}
