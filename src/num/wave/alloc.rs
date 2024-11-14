// devela::num::wave::alloc
//
//! Wavelet items that allocate.
//
// TOC
// - definitions
//   - trait WaveletCompressionVec
//   - trait WaveletTransformVec
//   - struct WaveletUnitVec
// - impls for WaveletHaar

use crate::{vec_ as vec, Vec, WaveletHaar, WaveletUnitRole};

#[cfg(feature = "_float_f64")]
#[allow(unused_imports, reason = "!std: abs")]
use crate::ExtFloat;

/// Trait for lossy compression of wavelet coefficients.
///
/// Compresses coefficients based on a given tolerance, zeroing out those
/// deemed insignificant to the desired error threshold.
pub trait WaveletCompressionVec {
    /// Compresses wavelet coefficients by thresholding small values.
    fn compress(&self, coeffs: &[f64], tolerance: f64) -> Vec<f64>;
}

/// Trait defining essential wavelet transform operations.
pub trait WaveletTransformVec {
    /// Computes the forward wavelet transform on the given input.
    #[must_use]
    fn forward(&self, input: &[f64]) -> Vec<f64>;

    /// Computes the inverse wavelet transform on the given coefficients.
    #[must_use]
    fn inverse(&self, coeffs: &[f64]) -> Vec<f64>;
}

/// A single unit of the wavelet decomposition at a specific level and position.
pub struct WaveletUnitVec {
    /// The type of the wavelet component (scaling or wavelet).
    pub component_type: WaveletUnitRole,
    /// Resolution level of the component, indicating its level of detail.
    pub level: usize,
    /// Position index of the component within the resolution level.
    pub position: usize,
    /// Coefficient values defining the component's shape.
    pub values: Vec<f64>,
}

impl WaveletUnitVec {
    /// Creates a new wavelet component of a specified type, level, position, and values.
    #[inline]
    pub fn new(
        component_type: WaveletUnitRole,
        level: usize,
        position: usize,
        values: Vec<f64>,
    ) -> Self {
        Self { component_type, level, position, values }
    }
}

/* impls for WaveletHaar */

impl WaveletCompressionVec for WaveletHaar {
    fn compress(&self, coeffs: &[f64], tolerance: f64) -> Vec<f64> {
        coeffs.iter().map(|&c| if c.abs() < tolerance { 0.0 } else { c }).collect()
    }
}

impl WaveletTransformVec for WaveletHaar {
    fn forward(&self, input: &[f64]) -> Vec<f64> {
        let mut output = input.to_vec();
        let mut length = output.len();

        // Iteratively apply averaging and differencing until we reach the coarsest level.
        while length > 1 {
            let mut temp = vec![0.0; length];
            for i in 0..length / 2 {
                // Calculate average and detail coefficients for each pair.
                let average = (output[2 * i] + output[2 * i + 1]) / 2.0;
                let difference = (output[2 * i] - output[2 * i + 1]) / 2.0;
                temp[i] = average;
                temp[length / 2 + i] = difference; // Store details in the second half
            }
            // Copy temporary results back to output for the next level.
            output[..length].clone_from_slice(&temp);
            length /= 2; // Reduce length to work on the next coarser level
        }
        output
    }

    fn inverse(&self, coeffs: &[f64]) -> Vec<f64> {
        let mut output = coeffs.to_vec();
        let mut length = 2;

        // Iteratively reconstruct from coarsest to finest level
        while length <= output.len() {
            let mut temp = vec![0.0; length];
            for i in 0..length / 2 {
                // Reconstruct original values from average and detail coefficients.
                let average = output[i];
                let difference = output[length / 2 + i];
                temp[2 * i] = average + difference; // First reconstructed value
                temp[2 * i + 1] = average - difference; // Second reconstructed value
            }
            // Copy reconstructed values back to output for the next finer level.
            output[..length].clone_from_slice(&temp);
            length *= 2; // Expand length to work on the next finer level
        }
        output
    }
}
