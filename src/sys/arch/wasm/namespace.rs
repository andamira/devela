// devela::sys::arch:wasm::namespace
//
//! Defines the [`Wasm`] namespace.
//!
//! # Links
//! - <https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html>
//! - <https://doc.rust-lang.org/rustc/platform-support/wasm64-unknown-unknown.html>
//! - <https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip1.html>
//! - <https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip1-threads.html>
//! - <https://doc.rust-lang.org/rustc/platform-support/wasm32-wasip2.html>
//! - <https://surma.dev/things/rust-to-webassembly/> (2023-02-10)
//!
//! # Info
//! - Show all features
//!   `rustc --print target-features --target wasm32-unknown-unknown`

#[cfg(target_family = "wasm")]
use ::core::arch::wasm32::{memory_grow, memory_size};

/// A Wasm namespace.
pub struct Wasm;

impl Wasm {
    /// Number of bytes per memory page.
    pub const PAGE_BYTES: usize = 65_536;

    /// Maximum possible memory pages.
    pub const MAX_PAGES: usize = 65_536;

    /// Maximum addressable memory in bytes.
    pub const MAX_MEMORY: usize = Self::MAX_PAGES.saturating_mul(Self::PAGE_BYTES);

    /// Returns `true` if the crate was compiled for a WebAssembly target.
    ///
    /// This is a **compile-time check**, not a runtime check.
    ///
    /// Equivalent to `cfg!(target_family = "wasm")`.
    pub const fn is_wasm_target() -> bool {
        cfg!(target_family = "wasm")
    }

    /// Checks if WebAssembly atomics are enabled.
    ///
    /// Returns `true` if it's been compiled with `RUSTFLAGS='-C target-feature=+atomics`.
    ///
    /// See <https://doc.rust-lang.org/core/arch/wasm32/index.html#atomics>.
    pub const fn has_atomics() -> bool {
        cfg!(target_feature = "atomics")
    }

    /// Checks if bulk memory operations are available.
    ///
    /// Returns `true` if it's been compiled with `RUSTFLAGS='-C target-feature=+bulk-memory`.
    pub fn has_bulk_memory() -> bool {
        cfg!(target_feature = "bulk-memory")
    }

    /// Checks if mutable-globals are available.
    ///
    /// Returns `true` if it's been compiled with `RUSTFLAGS='-C target-feature=+mutable-globals`.
    pub fn has_mutable_globals() -> bool {
        cfg!(target_feature = "mutable-globals")
    }

    /// Checks if SIMD is available.
    ///
    /// Returns `true` if it's been compiled with `RUSTFLAGS='-C target-feature=+simd128`.
    ///
    /// See <https://doc.rust-lang.org/core/arch/wasm32/index.html#simd>.
    pub fn has_simd() -> bool {
        cfg!(target_feature = "simd128")
    }

    /// Returns the current memory size in units of pages.
    ///
    /// On non-WASM platforms, always returns `0`.
    ///
    /// See `core::arch::wasm32::`[`memory_size`].
    pub fn memory_size() -> usize {
        #[cfg(target_family = "wasm")]
        return memory_size(0);
        #[cfg(not(target_family = "wasm"))]
        0
    }

    /// Returns the current memory size in bytes.
    ///
    /// On non-WASM platforms, always returns `0`.
    pub fn memory_bytes() -> usize {
        Self::memory_size().saturating_mul(Self::PAGE_BYTES)
    }

    /// Attempts to grow the default linear memory by the specified `delta` of pages.
    ///
    /// Returns `true` on success.
    /// On non-WASM platforms, always returns `false`.
    ///
    /// See `core::arch::wasm32::`[`memory_grow`].
    #[allow(unused_variables, reason = "cross-platform")]
    pub fn memory_grow(delta: usize) -> bool {
        #[cfg(target_family = "wasm")]
        {
            let previous = Self::memory_size();
            let result = memory_grow(0, delta);
            return previous != result;
        }
        #[cfg(not(target_family = "wasm"))]
        false
    }

    /// Returns `true` if memory can still grow.
    ///
    /// On non-WASM platforms, always returns `false`.
    pub fn memory_can_grow() -> bool {
        #[cfg(target_family = "wasm")]
        return Self::memory_size() < Self::MAX_PAGES;
        #[cfg(not(target_family = "wasm"))]
        false
    }
}
