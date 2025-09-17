// devela::num::int::num_trait::ref
//
//! Defines the `NumRefInt` trait and auto-implements it.
//
// TOC
// - trait NumRefInt
//   - base
//   - core
//   - combinatorics
//   - division
//   - factors
//   - primes
//   - modulo
//   - roots
// - macro helpers
//   - impl_int_ref
//   - own_fn

#[cfg(feature = "alloc")]
use crate::Vec;
use crate::{Deref, GcdReturn, IntResult as Result, Num, NumInt, NumRef, ValueQuant};

#[doc = crate::_TAG_NUM!()]
/// Common auto-trait for referenced integer types.
///
/// # Notes
/// - This is automatically implemented for references of types implementing [`NumInt`].
/// - Mutable operations are only available for exclusive (`&mut`) references.
//
// In sync with src/num/int/num_trait/mod.rs (int_ref_* methods)
#[cfg_attr(nightly_doc, doc(notable_trait))]
#[rustfmt::skip]
pub trait NumRefInt<'a>: NumRef<'a>
where
    Self: Deref<Target = <Self as NumRef<'a>>::Own>,
    <Self as NumRef<'a>>::Own: NumInt
{
    /* base */

    impl_int_ref![int_ref_digital_root(&self) -> Out];
    impl_int_ref![int_ref_digital_root_base(&self, base: &Rhs) -> Out];

    impl_int_ref![int_ref_digits(&self) -> Out];
    impl_int_ref![int_ref_digits_sign(&self) -> Out];
    impl_int_ref![int_ref_digits_base(&self, base: &Rhs) -> Out];
    impl_int_ref![int_ref_digits_base_sign(&self, base: &Rhs) -> Out];

    /* core */

    impl_int_ref![int_ref_abs(&self) -> Out];
    impl_int_ref![int_ref_is_even(&self) -> bool];
    impl_int_ref![int_ref_is_odd(&self) -> bool];

    impl_int_ref![int_ref_gcd(&self, other: &Rhs) -> Out];

    #[doc = own_fn!["int_ref_gcd_ext"]]
    #[allow(clippy::type_complexity, reason = "unavoidable")]
    fn int_ref_gcd_ext(&self, other: &<Self::Own as Num>::Rhs)
        -> Result<GcdReturn<<Self::Own as Num>::Out, <Self::Own as NumInt>::OutI>> {
            self.deref().int_ref_gcd_ext(other) }
    // impl_int_ref![int_ref_gcd_ext(&self, other: &Rhs) -> GcdReturn<Out, OutI>]; MAYBE

    impl_int_ref![int_ref_lcm(&self, other: &Rhs) -> Out];
    impl_int_ref![int_ref_scale(&self, min: &Rhs, max: &Rhs, a: &Rhs, b: &Rhs) -> Out];
    impl_int_ref![int_ref_scale_wrap(&self, min: &Rhs, max: &Rhs, a: &Rhs, b: &Rhs) -> Out];
    impl_int_ref![int_ref_midpoint(&self, other: &Rhs) -> Out];

    /* combinatorics */

    impl_int_ref![int_ref_factorial(&self) -> Out];
    impl_int_ref![int_ref_subfactorial(&self) -> Out];
    impl_int_ref![int_ref_permute(&self, r: &Rhs) -> Out];
    impl_int_ref![int_ref_permute_rep(&self, r: &Rhs) -> Out];
    impl_int_ref![int_ref_combine(&self, r: &Rhs) -> Out];
    impl_int_ref![int_ref_combine_rep(&self, r: &Rhs) -> Out];

    /* division */

    impl_int_ref![int_ref_div_rem(&self, r: &Rhs) -> [Out; 2]];
    impl_int_ref![int_ref_div_ceil(&self, r: &Rhs) -> Out];
    impl_int_ref![int_ref_div_floor(&self, r: &Rhs) -> Out];
    impl_int_ref![int_ref_div_ties_away(&self, r: &Rhs) -> Out];
    impl_int_ref![int_ref_div_ties_towards(&self, r: &Rhs) -> Out];
    impl_int_ref![int_ref_div_ties_even(&self, r: &Rhs) -> Out];
    impl_int_ref![int_ref_div_ties_odd(&self, r: &Rhs) -> Out];

    /* factors (allocating) */

    #[doc = own_fn!["int_ref_factors"]]
    #[cfg(feature = "alloc")] #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    fn int_ref_factors(&self) -> Result<Vec<<Self::Own as Num>::Out>> {
            self.deref().int_ref_factors() }
    #[doc = own_fn!["int_ref_factors_proper"]]
    #[cfg(feature = "alloc")] #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    fn int_ref_factors_proper(&self) -> Result<Vec<<Self::Own as Num>::Out>> {
            self.deref().int_ref_factors_proper() }
    #[doc = own_fn!["int_ref_factors_prime"]]
    #[cfg(feature = "alloc")] #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    fn int_ref_factors_prime(&self) -> Result<Vec<<Self::Own as Num>::Out>> {
            self.deref().int_ref_factors_prime() }
    #[doc = own_fn!["int_ref_factors_prime_unique"]]
    #[cfg(feature = "alloc")] #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    fn int_ref_factors_prime_unique(&self) -> Result<Vec<<Self::Own as Num>::Out>> {
            self.deref().int_ref_factors_prime_unique() }

    /* factors (non-allocating) */

    #[doc = own_fn!["int_ref_factors_buf"]]
    fn int_ref_factors_buf(&self,
        fbuf: &mut [<Self::Own as Num>::Out], upfbuf: &mut [<Self::Own as Num>::Out])
        -> Result<(usize, usize)> {
            self.deref().int_ref_factors_buf(fbuf, upfbuf) }
    #[doc = own_fn!["int_ref_factors_proper_buf"]]
    fn int_ref_factors_proper_buf(&self,
        fbuf: &mut [<Self::Own as Num>::Out], upfbuf: &mut [<Self::Own as Num>::Out])
        -> Result<(usize, usize)> {
            self.deref().int_ref_factors_proper_buf(fbuf, upfbuf) }
    #[doc = own_fn!["int_ref_factors_prime_buf"]]
    fn int_ref_factors_prime_buf(&self, buffer: &mut [<Self::Own as Num>::Out]) -> Result<usize> {
            self.deref().int_ref_factors_prime_buf(buffer) }
    #[doc = own_fn!["int_ref_factors_prime_unique_buf"]]
    fn int_ref_factors_prime_unique_buf(&self, buffer: &mut [<Self::Own as Num>::Out])
        -> Result<usize> {
            self.deref().int_ref_factors_prime_unique_buf(buffer) }

    /* primes */

    impl_int_ref![int_ref_is_prime(&self) -> bool];
    impl_int_ref![int_ref_prime_nth(&self) -> Out];
    impl_int_ref![int_ref_prime_pi(&self) -> usize];
    impl_int_ref![int_ref_totient(&self) -> Out];

    /* roots (square) */

    impl_int_ref![int_ref_is_square(&self) -> bool];
    impl_int_ref![int_ref_sqrt_ceil(&self) -> Out];
    impl_int_ref![int_ref_sqrt_floor(&self) -> Out];
    impl_int_ref![int_ref_sqrt_round(&self) -> Out];

    /* modulo */

    impl_int_ref![int_ref_modulo(&self, modulus: &Rhs) -> Out];
    impl_int_ref![int_ref_modulo_cycles(&self, modulus: &Rhs) -> ValueQuant<Out, Out>];

    impl_int_ref![int_ref_modulo_add(&self, other: &Rhs, modulus: &Rhs) -> Out];
    impl_int_ref![int_ref_modulo_add_cycles(&self, other: &Rhs, modulus: &Rhs)
        -> ValueQuant<Out, Out>];
    impl_int_ref![int_ref_modulo_add_inv(&self, modulus: &Rhs) -> Out];
    impl_int_ref![int_ref_modulo_sub(&self, other: &Rhs, modulus: &Rhs) -> Out];
    impl_int_ref![int_ref_modulo_sub_cycles(&self, other: &Rhs, modulus: &Rhs)
        -> ValueQuant<Out, Out>];
    impl_int_ref![int_ref_modulo_mul(&self, other: &Rhs, modulus: &Rhs) -> Out];
    impl_int_ref![int_ref_modulo_mul_cycles(&self, other: &Rhs, modulus: &Rhs)
        -> ValueQuant<Out, Out>];
    impl_int_ref![int_ref_modulo_mul_inv(&self, modulus: &Rhs) -> Out];
    impl_int_ref![int_ref_modulo_div(&self, other: &Rhs, modulus: &Rhs) -> Out];

    /* roots */

    impl_int_ref![int_ref_root_ceil(&self, nth: u32) -> Out];
    impl_int_ref![int_ref_root_floor(&self, nth: u32) -> Out];
}

/// Implements the given method.
macro_rules! impl_int_ref {
    (
    // >=0 Num::Rhs args, returns Self::Out
    $fn:ident(&$self:ident $(, $arg:ident: &Rhs)*) -> Out) => { $crate::paste! {
        #[doc = own_fn![$fn]]
        fn $fn(&$self $(, $arg: &<Self::Own as Num>::Rhs)*)
            -> Result<<Self::Own as Num>::Out> {
            $self.deref().$fn($($arg),*) }
    }};
    (
    // >=0 Num::Rhs args, returns an array of Self::Out
    $fn:ident(&$self:ident $(, $arg:ident: &Rhs)*)
     -> [Out; $LEN:literal]) => { $crate::paste! {
        #[doc = own_fn![$fn]]
        fn $fn(&$self $(, $arg: &<Self::Own as Num>::Rhs)*)
            -> Result<[<Self::Own as Num>::Out; $LEN]> {
            $self.deref().$fn($($arg),*) }
    }};
    (
    // >=0 Num::Rhs args, returns ValueQuant<Self::Out, Self::Out>
    $fn:ident(&$self:ident $(, $arg:ident: &Rhs)*)
    -> ValueQuant<Out, Out>) => { $crate::paste! {
        #[doc = own_fn![$fn]]
        fn $fn(&$self $(, $arg: &<Self::Own as Num>::Rhs)*)
            -> Result<ValueQuant<<Self::Own as Num>::Out, <Self::Own as Num>::Out>> {
            $self.deref().$fn($($arg),*) }
    }};
    (
    // >=0 ty args, returns Self::Out
    $fn:ident(&$self:ident $(, $arg:ident: $arg_ty:ty)*) -> Out) => { $crate::paste! {
        #[doc = own_fn![$fn]]
        fn $fn(&$self $(, $arg: $arg_ty)*) -> Result<<Self::Own as Num>::Out> {
            $self.deref().$fn($($arg),*) }
    }};
    (
    // 0 args returns ty (bool, usize…)
    $fn:ident(&$self:ident) -> $out:ty) => { $crate::paste! {
        #[doc = own_fn![$fn]]
        fn $fn(&$self) -> Result<$out> {
            $self.deref().$fn() }
    }};
}
use impl_int_ref;

/// Links to the version that operates on values.
macro_rules! own_fn {
    ($fn:ident) => { own_fn!(@stringify!($fn)) };
    ($fn:literal) => { own_fn!(@$fn) };
    (@$fn_str:expr) => {
        concat! { "Calls [`NumInt::", $fn_str, "][NumInt::", $fn_str, "]." }
    };
}
use own_fn;
