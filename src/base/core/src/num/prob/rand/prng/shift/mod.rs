// devela_base_core::num::prob::rand::prng::shift
//
//! Pseudo-random number generators based on [Xorshift].
//!
//! This module defines several types:
//! - classic *XorShift* algorithms:
//!   ([`XorShift32`], [`XorShift64`], [`XorShift128`], [`XorShift128p`]).
//! - variations with a smaller state:
//!   ([`XorShift16`], [`XorShift8`]).
//!
//! - Original paper: <https://www.jstatsoft.org/article/view/v008i14>
//!
//! [Xorshift]: https://en.wikipedia.org/wiki/Xorshift
//

mod u128p;
pub use u128p::XorShift128p; // (canonical)

#[cfg(feature = "rand")]
crate::items! {
    mod u128; // (11, 8,19)   (canonical)
    mod u16;  // ( 7, 9, 8)
    mod u32;  // ( 5,17,13)   (customizable, canonical default)
    mod u64;  // (13, 7,17)   (customizable, canonical default)
    mod u8;   // ( 3, 4, 2)   (customizable)
    #[cfg_attr(nightly_doc, doc(cfg(feature = "rand")))]
    pub use {
        u128::XorShift128,
        u16::XorShift16,
        u32::XorShift32,
        u64::XorShift64,
        u8::XorShift8,
    };

    #[doc(hidden)]
    pub use {u16::XOROSHIFT_16_TRIPLETS, u32::XOROSHIFT_32_TRIPLETS, u64::XOROSHIFT_64_TRIPLETS};

    /// Constructs a *XorShift* prng with custom bits, basis, triplet and seed.
    ///
    /// It can construct custom instances of [`XorShift16`], [`XorShift32`] and [`XorShift64`].
    ///
    /// The given `$triplet` is an index for an array of good triples with a maximum of:
    /// - 3 for 16-bit,
    /// - 80 for 32-bit
    /// - 274 for 64-bit
    ///
    /// ## Usage:
    /// `xorshift_with![bits: 32, basis: 1, triplet: 40, seed: 5334];`
    ///
    /// Valid argument values:
    /// - `$bits`:    `16`, `32` or `64`.
    /// - `$basis`:   in range `0..=7`.
    /// - `$triplet`: `0..=3` for 16-bit; `0..=80` for 32-bit; `0..=274` for 64-bit.
    /// - `$seed`:    any value. If `0` is given the default seed will be used.
    ///
    /// # Panics
    /// If the `$basis` is outside range `0..=7`.
    //
    // The reason for this macro is that is not possible to operate with const-generics,
    // so we can't make a method using an INDEX const-generic to index in a const array.
    // Also: Inner items do not inherit the generic parameters from the items they are embedded in.
    // WAIT: [generic_const_expr](https://github.com/rust-lang/rust/issues/76560)
    #[doc(hidden)]
    #[macro_export]
    #[rustfmt::skip]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "rand")))]
    macro_rules! _xorshift_custom {
        (bits:$bits:literal, basis:$basis:expr, triplet:$triplet:expr, seed:$seed:expr) => {{
            $crate::paste! {
                const T: (u8, u8, u8) = $crate::[<XOROSHIFT_ $bits _TRIPLETS>][{ $triplet }];
                    $crate::[<XorShift $bits>]
                        ::<{$basis}, {T.0 as usize}, {T.1 as usize}, {T.2 as usize}>
                        ::new($seed)
            }
        }};
    }
    #[doc = crate::_tags!(rand)]
    #[doc(inline)]
    pub use _xorshift_custom as xorshift_custom;

    /// Generates a XORSHIFT sequence using the given operation basis and shift triplet.
    ///
    /// # Usage:
    /// `xorshift_basis![<basis>: (<a>, <b>, <c>) <state>];`
    ///
    /// Notes
    /// - The `state` has to be different from 0.
    /// - The `basis` has to be a value between 0 and 7.
    /// - The `(a, b, c)` triplet has to be a valid one for its bit-size.
    /// - When `basis` is a constant the macro is optimized away and has 0 overhead.
    ///
    /// # Panics
    /// If the basis is outside range `0..=7`.
    //
    // - The canonical 32-bit example uses triplet  #40:( 5,17,13) with basis 1.
    // - The canonical 64-bit example uses triplet #155:(13, 7,17) with basis 0.
    macro_rules! xorshift_basis {
        [$state:ident, $basis:expr, ($a:expr, $b:expr, $c:expr)] => {
            match $basis {
                0 => { $state^=$state << $a; $state^=$state >> $b; $state^=$state << $c; },
                1 => { $state^=$state << $c; $state^=$state >> $b; $state^=$state << $a; },
                2 => { $state^=$state >> $a; $state^=$state << $b; $state^=$state >> $c; },
                3 => { $state^=$state >> $c; $state^=$state << $b; $state^=$state >> $a; },
                4 => { $state^=$state << $a; $state^=$state << $c; $state^=$state >> $b; },
                5 => { $state^=$state << $c; $state^=$state << $a; $state^=$state >> $b; },
                6 => { $state^=$state >> $a; $state^=$state >> $c; $state^=$state << $b; },
                7 => { $state^=$state >> $c; $state^=$state >> $a; $state^=$state << $b; },
                _ => panic!("Error: xorshift $basis must be between 0..=7"),
            }
        };
    }
    pub(crate) use xorshift_basis;
}
