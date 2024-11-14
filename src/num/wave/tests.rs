// devela::num::wave::tests

#[cfg(feature = "alloc")]
#[cfg(any(feature = "std", feature = "_float_f64"))]
mod alloc {
    use super::super::*;
    use crate::vec_ as vec;

    /// Checks that the forward transform produces the correct coefficients for a basic input,
    /// testing the correct averaging and differencing.
    #[test]
    fn test_haar_forward_transform() {
        let haar = WaveletHaar;
        let input = vec![9.0, 7.0, 3.0, 5.0];
        let result = haar.forward(&input);
        // Expected: Overall average, followed by detail coefficients
        assert_eq!(result, vec![6.0, 2.0, 1.0, -1.0]);
    }

    /// Ensures the inverse transform accurately reconstructs the original values from the wavelet
    /// coefficients.
    #[test]
    fn test_haar_inverse_transform() {
        let haar = WaveletHaar;
        let coeffs = vec![6.0, 2.0, 1.0, -1.0];
        let result = haar.inverse(&coeffs);
        // Reconstructed values should match the original input
        assert_eq!(result, vec![9.0, 7.0, 3.0, 5.0]);
    }

    /// Verifies that the WaveletScalingFunction struct initializes with the correct level,
    /// position, and values.
    #[test]
    fn test_scaling_function_initialization() {
        let scaling_func = WaveletUnitVec::new(WaveletUnitRole::Scaling, 1, 0, vec![0.5, 0.5]);
        assert_eq!(scaling_func.level, 1);
        assert_eq!(scaling_func.position, 0);
        assert_eq!(scaling_func.values, vec![0.5, 0.5]);
    }

    /// Confirms that the WaveletFunction struct initializes with the expected properties.
    #[test]
    fn test_wavelet_function_initialization() {
        let wavelet_func = WaveletUnitVec::new(WaveletUnitRole::Wavelet, 1, 1, vec![1.0, -1.0]);
        assert_eq!(wavelet_func.level, 1);
        assert_eq!(wavelet_func.position, 1);
        assert_eq!(wavelet_func.values, vec![1.0, -1.0]);
    }

    /// Validates that the compression method zeroes out coefficients below the specified
    /// tolerance, while leaving significant values untouched.
    #[test]
    fn test_haar_compression() {
        let haar = WaveletHaar;
        let coeffs = vec![6.0, 0.1, 1.0, -0.05];
        let result = haar.compress(&coeffs, 0.5);
        // Expect values below 0.5 to be zeroed out
        assert_eq!(result, vec![6.0, 0.0, 1.0, 0.0]);
    }

    /// Verifies that a forward and inverse transform round-trip reproduces the original input
    /// within a small floating-point tolerance, ensuring transform consistency.
    #[test]
    fn test_forward_inverse_roundtrip() {
        let haar = WaveletHaar;
        let input = vec![10.0, 2.0, 8.0, 4.0];
        let forward_result = haar.forward(&input);
        let inverse_result = haar.inverse(&forward_result);
        // Verify round-trip accuracy within floating-point tolerance
        for (a, b) in input.iter().zip(inverse_result.iter()) {
            assert!((a - b).abs() < 1e-10);
        }
    }
}
