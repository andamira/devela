// devela/src/num/prob/rand/qual.rs
//
//! Defines [`RandQualities`].
//

crate::set! {
    #[doc = crate::_tags!(rand set)]
    /// Semantic qualities of a random source.
    #[doc = crate::_doc_meta!{
        location("num/prob/rand"),
        test_size_of(RandQualities = 1),
    }]
    /// These qualities describe source behavior and suitability independently
    /// of its output width, state size, and fallibility.
    ///
    /// The absence of a quality means that property is not claimed.
    /// In particular, a source may be neither cryptographic nor known to be weak.
    pub struct RandQualities(u8) {
        /// The output stream is determined by the source's current state.
        DETERMINISTIC = 0;

        /// The same explicit seed material can recreate the same stream.
        REPRODUCIBLE = 1;

        /// The source is intended for cryptographic use.
        CRYPTOGRAPHIC = 2;

        /// The source depends on state outside its own value.
        EXTERNAL = 3;

        /// The source may block while producing random data.
        MAY_BLOCK = 4;

        /// The source has known generator-side weaknesses, such as
        /// poor statistical behavior, short period, or limited state capacity.
        GENERATOR_WEAK = 5;

        /// The source draws unpredictability from weak or low-entropy material.
        ENTROPY_WEAK = 6;
    }
    /// # Common profiles
    impl {
        /// Common qualities of reproducible deterministic PRNGs.
        pub const PRNG: Self = Self::DETERMINISTIC.with_reproducible();

        /// Common qualities of reproducible deterministic PRNGs with known
        /// generator-side weaknesses.
        pub const WEAK_PRNG: Self = Self::PRNG.with_generator_weak();

        /// Set containing both broad classes of known weakness:
        /// generator-side and entropy-source.
        pub const WEAK: Self = Self::GENERATOR_WEAK.with_entropy_weak();

        /// Returns whether either known form of weakness is present.
        #[must_use]
        pub const fn is_weak(self) -> bool {
            self.intersects(Self::WEAK)
        }
    }
}
