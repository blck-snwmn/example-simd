use example_simd::{scalar, scalar_unrolled, simd_wide};

fn assert_close(actual: f32, expected: f32) {
    let tolerance = 1e-4 * expected.abs().max(1.0);
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, got {actual}"
    );
}

fn sample_data(len: usize) -> (Vec<f32>, Vec<f32>) {
    let a = (0..len).map(|i| ((i as f32 * 1.25) % 17.0) - 8.0).collect();
    let b = (0..len).map(|i| ((i as f32 * 0.75) % 11.0) - 5.0).collect();

    (a, b)
}

#[test]
fn sum_implementations_match_scalar() {
    for len in [0, 1, 7, 8, 9, 15, 16, 17, 31, 64, 257] {
        let (data, _) = sample_data(len);
        let expected = scalar::sum::sum(&data);

        assert_close(scalar_unrolled::sum::sum(&data), expected);
        assert_close(simd_wide::sum::sum(&data), expected);
    }
}

#[test]
fn dot_product_implementations_match_scalar() {
    for len in [0, 1, 7, 8, 9, 15, 16, 17, 31, 64, 257] {
        let (a, b) = sample_data(len);
        let expected = scalar::dot_product::dot_product(&a, &b);

        assert_close(scalar_unrolled::dot_product::dot_product(&a, &b), expected);
        assert_close(simd_wide::dot_product::dot_product(&a, &b), expected);
    }
}

#[test]
fn euclidean_distance_implementations_match_scalar() {
    for len in [0, 1, 7, 8, 9, 15, 16, 17, 31, 64, 257] {
        let (a, b) = sample_data(len);
        let expected = scalar::distance::euclidean_distance(&a, &b);

        assert_close(
            scalar_unrolled::distance::euclidean_distance(&a, &b),
            expected,
        );
        assert_close(simd_wide::distance::euclidean_distance(&a, &b), expected);
    }
}

#[test]
fn binary_operations_use_the_shorter_input() {
    let a = vec![1.0, 2.0, 3.0, 4.0];
    let b = vec![2.0, 3.0];

    let expected_dot = scalar::dot_product::dot_product(&a, &b);
    let expected_distance = scalar::distance::euclidean_distance(&a, &b);

    assert_close(
        scalar_unrolled::dot_product::dot_product(&a, &b),
        expected_dot,
    );
    assert_close(simd_wide::dot_product::dot_product(&a, &b), expected_dot);
    assert_close(
        scalar_unrolled::distance::euclidean_distance(&a, &b),
        expected_distance,
    );
    assert_close(
        simd_wide::distance::euclidean_distance(&a, &b),
        expected_distance,
    );
}
