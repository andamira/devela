// devela/src/media/visual/image/raster/grid/interlace.rs
//
//! Classifies bitmap raster interlacing methods.
//

crate::enumset! {
    #[doc = crate::_tags!(image)]
    /// A bitmap raster interlacing method.
    #[doc = crate::_doc_meta!{
        location("media/visual/image/raster/grid", enum Interlace),
        test_size_of(Interlace = 1|8; niche Option),
    }]
    /// Interlacing changes the spatial traversal order of one logical raster,
    /// typically allowing partial data to progressively approximate the
    /// complete image.
    ///
    /// This classification excludes transform-domain, layered, or
    /// quality-progressive image coding, and temporal video interlacing.
    #[must_use]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum Interlace(
        #[doc = crate::_tags!(image)]
        /// A compact set of bitmap [`Interlace`] methods.
        #[doc = crate::_doc_meta!{
            location("media/visual/image/raster/grid", struct InterlaceSet),
            test_size_of(InterlaceSet = 1|8; niche !Option),
        }]
        #[must_use]
        pub InterlaceSet: u8
    ) {
        /// Ordinary raster order, without interlacing.
        None,

        /// Two-way row interlacing: even rows followed by odd rows.
        Row2Way,

        /// Four-way row interlacing, visiting rows in four interleaved phases.
        Row4Way,

        /// GIF's four-pass progressively refining row interlacing.
        Gif4,

        /// Adam7's fixed seven-pass two-dimensional interlacing.
        Adam7,

        /// Recursive Adam∞ interlacing, alternating row and column refinement.
        ///
        /// This scheme was introduced by FLIF.
        AdamInf,
    }

    impl set
    /// Common structural classifications of bitmap interlace methods.
    {
        /// Methods that reorder complete rows without subdividing them horizontally.
        pub const ROW_ONLY: Self =
            Self::Row2Way
                .with(Self::Row4Way)
                .with(Self::Gif4);

        /// Methods that progressively refine both raster axes.
        pub const TWO_DIMENSIONAL: Self =
            Self::Adam7
                .with(Self::AdamInf);

        /// Methods having a fixed number of traversal passes.
        pub const FIXED_PASS: Self =
            Self::None
                .with(Self::Row2Way)
                .with(Self::Row4Way)
                .with(Self::Gif4)
                .with(Self::Adam7);

        /// Methods whose number of refinement levels depends on raster extent.
        pub const RECURSIVE: Self = Self::AdamInf;
    }
}
#[expect(clippy::derivable_impls, reason = "attributes are passed to the set")]
impl Default for Interlace {
    fn default() -> Self {
        Self::None
    }
}
