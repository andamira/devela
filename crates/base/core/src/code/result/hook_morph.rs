// devela_base_core::code::result::hook_morph
//
//! Defines [`Hook`], [`hook!`], [`Morph`], [`morph!`].
//!
//! > Threading a value through a sequence of operations.
//
// Based on the code by George Burton, Unlicense licensed.
// https://crates.io/crates/apply/0.3.0
//
// TOC
// - trait Hook
// - macro hook!
// - trait Morph
// - macro morph!

#[doc = crate::_tags!(code value)]
/// Hooks operations into a value without breaking its flow.
#[doc = crate::_doc_location!("code/result")]
///
/// `Hook` provides a way to intercept a value,
/// allowing mutation (`hook`) or observation (`tap`) while preserving identity.
///
/// Use this trait when working with APIs that mutate in place or return `()`,
/// but where fluent sequencing is still desired.
///
/// This is the ergonomic, runtime counterpart of the [`hook!`] macro.
///
/// # Examples
/// ```
/// # extern crate devela_base_core as devela;
/// use devela::Hook;
///
/// # #[cfg(feature = "alloc")] {
/// let v = vec![3, 2, 1, 5]
///     .hook(|v| v.sort())
///     .tap(|v| assert_eq![v, &[1, 2, 3, 5]])
///     .hook(|v| v.push(7));
/// assert_eq![v, vec![1, 2, 3, 5, 7]];
/// # }
/// ```
/// # Design
/// `Hook` preserves value identity.
/// For transformations that replace the value, see [`Morph`].
#[rustfmt::skip]
#[doc = crate::_doc!(vendor: "apply")]
pub trait Hook: Sized {
    /// Hooks a mutation step into the value and returns it.
    ///
    /// The provided function receives a mutable reference to the value,
    /// allowing in-place modification while preserving flow.
    ///
    /// This is the method form of the [`hook!`] macro.
    #[must_use]
    fn hook<F>(mut self, f: F) -> Self where F: FnOnce(&mut Self) { f(&mut self); self }

    /// Taps into the value for observation and returns it unchanged.
    ///
    /// The provided function receives a shared reference to the value.
    /// This is useful for inspection, logging, or assertions.
    #[must_use]
    fn tap<F>(self, f: F) -> Self where F: FnOnce(&Self) { f(&self); self }
}
impl<T: Sized> Hook for T {}

#[doc = crate::_tags!(code value)]
/// Hooks one or more mutation steps into a value and returns it.
#[doc = crate::_doc_location!("code/result")]
///
/// `hook!` binds a value to a local mutable name, applies a sequence of
/// side-effecting operations to it, and then returns the value.
///
/// This macro preserves the value's identity: steps may mutate it, but do not replace it.
///
/// This is the trait-free, const-capable mechanical form of [`Hook`].
///
/// Steps can be written as:
/// - a comma- or semicolon-separated list of expressions, or
/// - a sequence of adjacent `{ ... }` blocks.
///
/// Block syntax is useful when a step requires multiple statements
/// or local control flow.
///
/// All steps are applied in order to the same value.
///
/// # Example
/// ```
/// # use devela_base_core::{const_assert, is, hook};
/// /* expression syntax */
/// const ARRAY: [u8; 4] = [3, 1, 4, 1];
/// const DATA: [u8; 4] = hook!(ARRAY; |v| v[0] = 9; v[3] = 7);
/// const_assert![eq_buf &DATA, &[9, 1, 4, 7]];
///
/// /* block syntax */
/// const INPUT: [u8; 4] = [3, 0, 9, 1];
/// const OUTPUT: [u8; 4] = hook!(INPUT, |v| { v[0] = 1; is!(v[2] >7 ; v[2] -= 7) } { v[3] += 5 });
/// const_assert![eq_buf &OUTPUT, &[1, 0, 2, 6]];
/// ```
/// For transformations that replace the value, see [`morph!`].
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! hook {
    // (comma separated)
    // expr-sequence form
    ($t:expr, |$v:ident| $($step:expr),+ $(,)?) => {{
        let mut $v = $t; $( $step; )+ $v
    }};
    // block-sequence form
    ($t:expr, |$v:ident| $( { $($body:tt)* } $(,)? )+) => {{
        let mut $v = $t; $( { $($body)* }; )+ $v
    }};

    // (semicolon separated)
    // expr-sequence form
    ($t:expr; |$v:ident| $($step:expr);+ $(;)?) => {{
        let mut $v = $t; $( $step; )+ $v
    }};
    // block-sequence form
    ($t:expr; |$v:ident| $( { $($body:tt)* } $(;)? )+) => {{
        let mut $v = $t; $( { $($body)* }; )+ $v
    }};
}
#[doc(inline)]
pub use hook;

#[doc = crate::_tags!(code value)]
/// Morphs a value by threading it through a function.
#[doc = crate::_doc_location!("code/result")]
///
/// `Morph` provides a fluent way to pass a value (by value, shared reference,
/// or exclusive reference) into a transformation and return the result.
///
/// Unlike [`Hook`], `Morph` does not preserve value identity:
/// each operation yields a new result.
///
/// This is the ergonomic, runtime counterpart of the [`morph!`] macro.
///
/// # Examples
/// ```
/// # extern crate devela_base_core as devela;
/// use devela::Morph;
///
/// let x = 3.morph(|v| v * 2).morph(|v| v + 1);
/// assert_eq![x, 7];
/// ```
///
/// ```compile_fail
/// # extern crate devela_base_core as devela;
/// use devela::Morph;
///
/// // We can mutate it, but we do not receive the value back.
/// let v: Vec<i32> = vec![3, 2, 1, 5].morph_mut(|it| it.sort());
/// ```
#[rustfmt::skip]
#[doc = crate::_doc!(vendor: "apply")]
pub trait Morph<R> {
    /// Morphs the value into a new one and returns it.
    ///
    /// The provided function consumes the value and produces a replacement.
    #[must_use]
    fn morph<F>(self, f: F) -> R where F: FnOnce(Self) -> R, Self: Sized { f(self) }

    /// Morphs the value by shared reference and returns the result.
    ///
    /// The provided function receives `&Self` and may compute any result from it.
    #[must_use]
    fn morph_ref<F>(&self, f: F) -> R where F: FnOnce(&Self) -> R { f(self) }

    /// Morphs the value by exclusive reference and returns the result.
    ///
    /// The provided function receives `&mut Self` and may mutate it before producing the result.
    #[must_use]
    fn morph_mut<F>(&mut self, f: F) -> R where F: FnOnce(&mut Self) -> R { f(self) }
}
impl<T: ?Sized, R> Morph<R> for T {}

#[doc = crate::_tags!(code value)]
/// Morphs a value through one or more transformation steps and returns the result.
#[doc = crate::_doc_location!("code/result")]
///
/// `morph!` expresses a left-to-right transformation pipeline
/// without explicit intermediate bindings in user code.
///
/// Conceptually, `morph!(x, a, b, c)` corresponds to `c(b(a(x)))`.
///
/// A single binding introduces the pipeline variable,
/// which is reused implicitly by subsequent steps.
/// Each step may change the value's type.
///
/// This macro is const-capable and does not create closures.
/// It is the trait-free mechanical form of [`Morph`].
///
/// Steps can be written as:
/// - a comma-separated list of expressions, or
/// - a semicolon-separated list of expressions.
///
/// # Example
/// ```
/// # use devela_base_core::{const_assert, is, morph};
/// const S: &str = morph!(3u8, |v| v as usize, v * 2, v + 1, if v == 7 { "7" } else { "not7" });
/// const_assert![eq_str S, "7"];
/// ```
/// For pipelines that preserve value identity and mutate in place, see [`hook!`].
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! morph {
    // (comma separated)
    // base case: single-step
    ($t:expr, |$v:ident| $e:expr $(,)?) => {{ let $v = $t; $e }};
    // recursive case: multiple steps
    ($t:expr, |$v:ident| $e:expr, $($rest:expr),+ $(,)?) => {{
        morph!( { let $v = $t; $e }, |$v| $($rest),+ )
    }};

    // (semicolon separated)
    // base case: single-step
    ($t:expr; |$v:ident| $e:expr $(;)?) => {{ let $v = $t; $e }};
    // recursive case: multiple steps
    ($t:expr; |$v:ident| $e:expr; $($rest:expr);+ $(;)?) => {{
        morph!( { let $v = $t; $e }; |$v| $($rest);+ )
    }};
}
#[doc(inline)]
pub use morph;
