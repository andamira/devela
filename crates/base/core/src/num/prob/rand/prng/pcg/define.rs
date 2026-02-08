// devela_base_core::num::prob::rand::pcg::define
//
//! Defines [`define_pcg!`].
//

#[doc = crate::_tags!(construction rand)]
#[doc = concat!["Defines a canonical ", crate::_ABBR_PCG!(), " (Permuted Congruential Generator) ",
crate::_ABBR_PRNG!(), " type."]]
#[doc = crate::_doc_location!("num/prob/rand")]
///
/// This macro generates a concrete, fixed-width PCG PRNG
/// with a canonical state transition and output permutation.
///
/// The output width is selected by specifying one of the supported integer
/// sizes (`u8`, `u16`, `u32`, `u64`). The appropriate state size, multiplier,
/// increment, and output permutation are chosen automatically.
///
/// The generator stores two internal state values (state and inc), each twice
/// the output width, for a total internal state of four times the output width.
///
/// # Generated items
/// For the defined PRNG type, this macro generates:
/// - the PRNG struct with internal state.
/// - constructors and state accessors.
/// - step, peek, and jump-ahead operations.
/// - bounded generation helpers.
/// - an implementation of [`Rand`][crate::Rand] for generic usage.
///
/// # Example
/// ```
// WAIT: [cfg(doctest)](https://github.com/rust-lang/rust/issues/67295)
/// # #[macro_export]macro_rules!__crate_name{()=>{""};} // doctest stub for _doc_location
/// # use devela_base_core::define_pcg;
/// define_pcg! {
///     /// Custom attributes
///     pub(crate) Pcg8: (u8)
/// }
/// ```
/// See also [`Pcg32`][crate::Pcg32].
///
/// # Reference
/// PCG family reference: <https://www.pcg-random.org>
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! define_pcg {
    (
    /* public API */

      // Defines an 8-bit PCG PRNG (`u8` output, 16-bit state)
      $(#[$attrs:meta])* $vis:vis $name:ident : (u8)) => {
        $crate::define_pcg!(% $(#[$attrs])* $vis $name, 32+u16, 8+u8, 1,
            output_xsh_rr_16_8, 0xDEFA, 12829, 47989);
    };
    ( // Defines a 16-bit PCG PRNG (`u16` output, 32-bit state)
      $(#[$attrs:meta])* $vis:vis $name:ident : (u16)) => {
        $crate::define_pcg!(% $(#[$attrs])* $vis $name, 64+u32, 16+u16, 2,
            output_xsh_rr_32_16, 0xDEFA_0017, 747796405, 2891336453);
    };
    ( // Defines a 32-bit PCG PRNG (`u32` output, 64-bit state)
      $(#[$attrs:meta])* $vis:vis $name:ident : (u32)) => {
        $crate::define_pcg!(% $(#[$attrs])* $vis $name, 128+u64, 32+u32, 4,
            output_xsh_rr_64_32, 0xDEFA_0017_DEFA_0017, 6364136223846793005, 1442695040888963407);
    };
    ( // Defines a 64-bit PCG PRNG (`u64` output, 128-bit state)
      $(#[$attrs:meta])* $vis:vis $name:ident : (u64)

    /* internal arms */
    ) => {
        $crate::define_pcg!(% $(#[$attrs])* $vis $name, 256+u128, 64+u64, 8,
            output_xsl_rr_128_64, 0xDEFA_0017_DEFA_0017_DEFA_0017_DEFA_0017,
            47026247687942121848144207491837523525, 117397592171526113268558934119004209487);
    };

    // NOTE: This intermediate module exists to attach compile-time doc-routing attributes
    // (`#[compile(...)]`) to an item. Macro invocations themselves cannot carry such
    // attributes in statement position (e.g. rustdoc examples), but modules can.
    // The generated type is re-exported unchanged.
    (% $(#[$attrs:meta])*
     $vis:vis $name:ident, $sbits:literal+$state:ty, $obits:literal+$output:ty,
     $obytes:literal, $output_fn:ident,
     $SEED:literal, $MUL:literal, $INC:literal
     ) => { $crate::paste! {
        $crate::_devela_policy! { $vis mod [<__pcg_def_ $name >],
            // devela-specific doc conventions
            devela { $crate::define_pcg!(%define
                $(#[$attrs])*
                #[doc = $crate::_tags!(rand)]
                #[doc = concat!["A ", $obits, "-bit ", $crate::_ABBR_PCG!(), " ",
                $crate::_ABBR_PRNG!(), "."]]
                #[doc = $crate::_doc_location!("num/prob/rand")]
                $vis $name, $sbits+$state, $obits+$output, $obytes, $output_fn, $SEED, $MUL, $INC);
            }
            // simpler docs when invoked externally
            extern { $crate::define_pcg!(%define
                $(#[$attrs])*
                #[doc = concat![
                "A ", $obits, "-bit ", $crate::_ABBR_PCG!(), " ", $crate::_ABBR_PRNG!(), ".\n\n"]]
                $vis $name, $sbits+$state, $obits+$output, $obytes, $output_fn, $SEED, $MUL, $INC);
            }
        }
    }};

    /* private arms */
    (%define $(#[$attrs:meta])*
     $vis:vis $name:ident, $sbits:literal+$state:ty, $obits:literal+$output:ty,
     $obytes:literal, $output_fn:ident,
     $SEED:literal, $MUL:literal, $INC:literal
     ) => { $crate::paste! {
        $crate::define_pcg!(%impls $output $name);

        $(#[$attrs])*
        #[doc = "It uses a " $sbits
        "-bit linear congruential state with an independent stream selector, and generates "
        $obits "-bit values."]
        ///
        /// Compared to simple xorshift-style generators of similar size, it
        /// has stronger statistical properties while remaining small and fast.
        ///
        /// This implementation follows a canonical PCG output permutation
        /// (XSH RR or XSL RR, depending on size), by Melissa O’Neill.
        ///
        /// This generator is deterministic and non-cryptographic.
        ///
        /// Reference: <https://www.pcg-random.org>
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $vis struct $name {
            state: $state,
            inc: $state, // must always be odd
        }
        /// Creates a new PRNG initialized with the default fixed seed.
        impl Default for $name {
            fn default() -> Self {
                use $crate::ConstInitCore;
                Self::INIT
            }
        }
        /// Creates a new PRNG initialized with the default fixed seed.
        impl $crate::ConstInitCore for $name {
            const INIT: Self = Self::new(Self::DEFAULT_SEED, Self::DEFAULT_INC);
        }

        // Private associated item
        impl $name {
            #[doc(hidden)]
            pub const DEFAULT_SEED: $state = $SEED;
            #[doc(hidden)]
            pub const DEFAULT_INC: $state = $INC;

            #[inline]
            const fn advance_lcg(state: $state,
                mut delta: $state,
                mut cur_mult: $state,
                mut cur_plus: $state,
            ) -> $state {
                let mut acc_mult = 1 as $state;
                let mut acc_plus = 0 as $state;
                while delta > 0 {
                    if (delta & 1) != 0 {
                        acc_mult = acc_mult.wrapping_mul(cur_mult);
                        acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
                    }
                    cur_plus = cur_mult.wrapping_add(1).wrapping_mul(cur_plus);
                    cur_mult = cur_mult.wrapping_mul(cur_mult);
                    delta >>= 1;
                }
                acc_mult.wrapping_mul(state).wrapping_add(acc_plus)
            }
        }

        impl $name {
            #[doc = "Creates a new " $name " using the canonical PCG seeding scheme."]
            ///
            /// This performs a small warm-up so that closely related seeds produce
            /// well-scrambled initial outputs.
            pub const fn new(seed: $state, stream: $state) -> Self {
                let mut rng = Self::new_fast(0, stream);
                let _ = rng.[<next_ $output>]();
                rng.state = rng.state.wrapping_add(seed);
                let _ = rng.[<next_ $output>]();
                rng
            }

            #[doc = "Creates a new " $name " with the given initial state and stream."]
            ///
            /// The stream value selects an independent random sequence.
            pub const fn new_fast(state: $state, stream: $state) -> Self {
                Self { state, inc: (stream << 1) | 1 }
            }

            #[doc = "Creates a new " $name " without enforcing invariants."]
            ///
            /// # Panics
            /// Panics in debug if `inc` is not odd.
            pub const fn new_unchecked(state: $state, inc: $state) -> Self {
                debug_assert![!inc.is_multiple_of(2), "`inc` must be odd"];
                Self { state, inc }
            }

            /// Returns the PRNG's inner state as a raw snapshot.
            #[must_use]
            pub const fn inner_state(self) -> ($state, $state) {
                (self.state, self.inc)
            }

            /// Restores the PRNG from the given state.
            #[must_use]
            pub const fn from_state(state: ($state, $state)) -> Self {
                Self { state: state.0, inc: state.1 }
            }

            #[doc = "Returns the current `" $output "` value."]
            #[must_use]
            pub const fn [<current_ $output>](&self) -> $output {
                Self::$output_fn(self.state)
            }

            #[doc = "Advances to the next random `" $output "` value."]
            #[must_use]
            pub const fn [<next_ $output>](&mut self) -> $output {
                let old = self.state;
                self.state = old.wrapping_mul($MUL).wrapping_add(self.inc | 1);
                Self::$output_fn(old)
            }

            /// Returns a copy of the next PRNG state.
            #[must_use]
            pub const fn peek_next_state(&self) -> Self {
                let next_state = self.state.wrapping_mul($MUL).wrapping_add(self.inc | 1);
                Self { state: next_state, inc: self.inc }
            }
            #[doc = "Returns the next random `" $output "` without advancing the state."]
            #[must_use]
            pub const fn [<peek_next_ $output>](&self) -> $output {
                Self::$output_fn(self.state)
            }

            #[doc = "Returns both the next random state and the `" $output "` value."]
            pub const fn [<own_next_ $output>](self) -> $crate::Own<Self, $output> {
                let value = Self::$output_fn(self.state);
                let next = self.peek_next_state();
                $crate::Own::new(next, value)
            }

            /// Fills the given buffer
            pub const fn fill_bytes(&mut self, buffer: &mut [u8]) {
                use $crate::{Slice, slice};
                let mut i = 0;
                while i < buffer.len() {
                    let r = self.[<next_ $output>]();
                    let bytes = r.to_le_bytes();
                    let remaining = buffer.len() - i;
                    if remaining >= $obytes {
                        // dest[i..i + $obytes].copy_from_slice(&bytes);
                        Slice::copy(slice!(mut buffer, i, ..i + $obytes), &bytes);
                        i += $obytes;
                    } else {
                        // dest[i..].copy_from_slice(&bytes[..remaining]);
                        Slice::copy(slice!(mut buffer, i, ..), slice!(&bytes, ..remaining));
                        break;
                    }
                }
            }

            /// Advances the generator by `delta` steps.
            pub const fn advance(&mut self, delta: $state) {
                self.state = Self::advance_lcg(self.state, delta, $MUL, self.inc | 1);
            }

            /// Returns a copy of the state advanced by `delta` steps.
            pub const fn copy_advance(self, delta: $state) -> Self {
                let mut s = self;
                s.advance(delta);
                s
            }

            /// Returns a uniformly distributed value in `0..bound`.
            ///
            /// Uses rejection sampling to avoid modulo bias.
            #[must_use]
            pub fn next_bounded(&mut self, bound: $output) -> $output {
                debug_assert!(bound > 0);
                let threshold = bound.wrapping_neg() % bound;
                loop {
                    let r = self.[<next_ $output>]();
                    if r >= threshold { return r % bound; }
                }
            }
        }
    }};

    /* specific impls for the given output bit-size */

    (%impls u8 $name:ident) => {
        impl $name {
            #[inline(always)]
            const fn output_xsh_rr_16_8(state: u16) -> u8 {
                let xorshifted = (((state >> 5) ^ state) >> 3) as u8;
                let rot = (state >> 13) as u32;
                xorshifted.rotate_right(rot)
            }
        }
        impl $crate::Rand for $name {
            const RAND_OUTPUT_BITS: u32 = 8;
            const RAND_STATE_BITS: u32 = 32;

            fn rand_next_u32(&mut self) -> u32 {
                u32::from_le_bytes([self.next_u8(), self.next_u8(), self.next_u8(), self.next_u8()])
            }
            fn rand_next_u64(&mut self) -> u64 {
                u64::from_le_bytes([
                    self.next_u8(), self.next_u8(), self.next_u8(), self.next_u8(),
                    self.next_u8(), self.next_u8(), self.next_u8(), self.next_u8(),
                ])
            }
            fn rand_fill_bytes(&mut self, dest: &mut [u8]) {
                self.fill_bytes(dest)
            }
            fn rand_below(&mut self, upper: u64) -> u64 {
                debug_assert!(upper > 0);
                if upper <= u8::MAX as u64 {
                    self.next_bounded(upper as u8) as u64
                } else {
                    let zone = u64::MAX - (u64::MAX % upper);
                    loop {
                        let v = self.rand_next_u64();
                        if v < zone { return v % upper; }
                    }
                }
            }
        }
    };
    (%impls u16 $name:ident) => {
        impl $name {
            #[inline(always)]
            const fn output_xsh_rr_32_16(state: u32) -> u16 {
                let xorshifted = (((state >> 10) ^ state) >> 12) as u16;
                let rot = (state >> 28) as u32;
                xorshifted.rotate_right(rot)
            }
        }
        impl $crate::Rand for $name {
            const RAND_OUTPUT_BITS: u32 = 16;
            const RAND_STATE_BITS: u32 = 64;

            fn rand_next_u32(&mut self) -> u32 {
                $crate::Cast::<u32>::from_u16_le([self.next_u16(), self.next_u16()])
            }
            fn rand_next_u64(&mut self) -> u64 {
                $crate::Cast::<u64>::from_u16_le([
                    self.next_u16(), self.next_u16(), self.next_u16(), self.next_u16(),
                ])
            }
            fn rand_fill_bytes(&mut self, dest: &mut [u8]) {
                self.fill_bytes(dest)
            }
            fn rand_below(&mut self, upper: u64) -> u64 {
                debug_assert!(upper > 0);
                if upper <= u16::MAX as u64 {
                    self.next_bounded(upper as u16) as u64
                } else {
                    let zone = u64::MAX - (u64::MAX % upper);
                    loop {
                        let v = self.rand_next_u64();
                        if v < zone { return v % upper; }
                    }
                }
            }
        }
    };
    (%impls u32 $name:ident) => {
        impl $name {
            #[inline(always)]
            const fn output_xsh_rr_64_32(state: u64) -> u32 {
                let xorshifted = (((state >> 18) ^ state) >> 27) as u32;
                let rot = (state >> 59) as u32;
                xorshifted.rotate_right(rot)
            }
        }
        impl $crate::Rand for $name {
            const RAND_OUTPUT_BITS: u32 = 32;
            const RAND_STATE_BITS: u32 = 128;

            fn rand_next_u32(&mut self) -> u32 { self.next_u32() }
            fn rand_next_u64(&mut self) -> u64 {
                $crate::Cast::<u64>::from_u32_le([self.next_u32(), self.next_u32()])
            }
            fn rand_fill_bytes(&mut self, dest: &mut [u8]) { self.fill_bytes(dest) }
            fn rand_below(&mut self, upper: u64) -> u64 {
                debug_assert!(upper > 0);
                if upper <= u32::MAX as u64 {
                    self.next_bounded(upper as u32) as u64
                } else {
                    // fallback to generic rejection on u64
                    let zone = u64::MAX - (u64::MAX % upper);
                    loop {
                        let v = self.rand_next_u64();
                        if v < zone { return v % upper; }
                    }
                }
            }
        }
    };
    (%impls u64 $name:ident) => {
        impl $name {
            #[inline(always)]
            const fn output_xsl_rr_128_64(state: u128) -> u64 {
                let xorshifted = (((state >> 64) ^ state) >> 64) as u64;
                let rot = (state >> 122) as u32;
                xorshifted.rotate_right(rot)
            }
        }
        impl $crate::Rand for $name {
            const RAND_OUTPUT_BITS: u32 = 64;
            const RAND_STATE_BITS: u32 = 256;

            fn rand_next_u32(&mut self) -> u32 { self.next_u64() as u32 }
            fn rand_next_u64(&mut self) -> u64 { self.next_u64() }
            fn rand_fill_bytes(&mut self, dest: &mut [u8]) { self.fill_bytes(dest) }
            fn rand_below(&mut self, upper: u64) -> u64 {
                debug_assert!(upper > 0);
                let zone = u64::MAX - (u64::MAX % upper);
                loop {
                    let v = self.rand_next_u64();
                    if v < zone { return v % upper; }
                }
            }
        }
    };
}
#[doc(inline)]
pub use define_pcg;
