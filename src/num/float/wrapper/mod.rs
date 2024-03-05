// devela::num::flat::wrapper
//
//! Floating point wrapper struct.
//
// TOC
// - define Floating struct
// - implement Floating methods
//   - when std is enabled
//   - when libm is enabled
//   - when neither std or libm are enabled

mod shared; // shared methods

// #[cfg(any(feature = "libm", feature = "std"))]
mod libm_std; // methods depending on libm, std, or their absence

mod consts; // define constants

/// Provides floating-point operations for `T`.
///
/// Every operation returns the inner type `T` instead of `Self`.
/// It is designed as a utility namespace and does not hold or wrap data itself.
/// Instead, it operates on slices provided directly as arguments to its static methods.
///
///
/// See also the [`ExtFloat`][super::ExtFloat] trait.
///
/// # Constants
/// - [Related to Pi (π)](#mathematical-constants-related-to-pi-π)
/// - [Related to Tau (τ)](#mathematical-constants-related-to-tau-τ)
/// - [Related to Phi (φ)](#mathematical-constants-related-to-phi-φ)
/// - [Related to integer roots](#mathematical-constants-related-to-integer-roots)
/// - [Other constants](#other-mathematical-constants)
///
/// # Features
///
/// The wrapper leverages `std` or `libm` if enabled, otherwise implements fallbacks.
/// It also favors `std` style for method's names, but changes a few like `minimum`
/// for `min_nan` and `maximum` for `max_nan`, for consistency.
///
/// If both the `libm` and `std` features are enabled the `libm` functions will
/// be used, since it contains more functions, namely:
/// - Gamma functions: [`gamma`][Floating#method.gamma], [`lgamma`][Floating#method.lgamma],
///   [`lgamma_r`][Floating#method.lgamma_r].
/// - Bessel functions:
///   [`j0`][Floating#method.j0], [`j1`][Floating#method.j1], [`jn`][Floating#method.jn],
///   [`y0`][Floating#method.y0], [`y1`][Floating#method.y1], [`yn`][Floating#method.yn].
/// - Error functions: [`erf`][Floating#method.erf], [`erfc`][Floating#method.erfc].
/// - [`exp10`][Floating#method.exp10].

#[derive(Debug, Clone, Copy)]
pub struct Floating<T>(core::marker::PhantomData<T>);
