/// Array sum (iterator version)
#[inline]
pub fn sum(data: &[f32]) -> f32 {
    data.iter().sum()
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
    fn test_sum_empty() {
        let data: Vec<f32> = vec![];
        assert!((sum(&data) - 0.0).abs() < 1e-6);
    }
}
