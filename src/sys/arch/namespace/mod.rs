// devela::sys::arch::namespace
//
//! Defines the [`Arch`] namespace.
//

#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_hint")))]
#[cfg(all(not(feature = "safe_sys"), feature = "unsafe_hint"))]
mod instructions; // architecture-specific instructions

#[cfg(feature = "dep_safe_arch")]
mod dep_safe_arch;

#[doc = crate::_TAG_NAMESPACE!()]
/// Arch-related functionality.
///
/// > If you are on this architecture, here is what the CPU itself can do.
///
/// # Methods
/// - Architecture-specific instructions
///   - [Portable abstractions](#portable-abstractions-over-architecture-dependent-instructions)
///   - [x86 or x86_64 instructions](#x86-or-x86_64-instructions)
///   - [arm instructions](#arm-instructions)
///   - [AArch64 instructions](#aarch64-instructions)
///   - [riscv instructions](#riscv-instructions)
/// - Implementations that depend on: `dep_safe_arch`, (`x86` or `x86_64`)
/// and some target feature:
///   - [none](#functions-not-requiring-any-target-feature).
///   - [`adx`](#functions-requiring-the-adx-target-feature).
///   - [`aes`](#functions-requiring-the-aes-target-feature).
///   - [`avx`](#functions-requiring-the-avx-target-feature).
///   - [`avx2`](#functions-requiring-the-avx2-target-feature).
///   - [`bmi1`](#functions-requiring-the-bmi1-target-feature).
///   - [`bmi2`](#functions-requiring-the-bmi2-target-feature).
///   - [`fma`](#functions-requiring-the-fma-target-feature).
///   - [`lzcnt`](#functions-requiring-the-lzcnt-target-feature).
///   - [`pclmulqdq`](#functions-requiring-the-pclmulqdq-target-feature).
///   - [`popcnt`](#functions-requiring-the-popcnt-target-feature).
///   - [`rdrand`](#functions-requiring-the-rdrand-target-feature).
///   - [`rdseed`](#functions-requiring-the-rdseed-target-feature).
///   - [`sse`](#functions-requiring-the-sse-target-feature)
///     ([generic](#generic-functions-requiring-the-sse-target-feature)).
///   - [`sse2`](#functions-requiring-the-sse2-target-feature).
///   - [`sse3`](#functions-requiring-the-sse3-target-feature).
///   - [`sse4.1`](#functions-requiring-the-sse41-target-feature).
///   - [`sse4.2`](#functions-requiring-the-sse42-target-feature).
///   - [`ssse3`](#functions-requiring-the-ssse3-target-feature).
#[derive(Debug)]
pub struct Arch;
