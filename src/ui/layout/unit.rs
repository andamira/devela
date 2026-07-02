// devela/ui/layout/unit.rs
//! Defines [`Lunit`] and metric aliases.
//

#[doc = crate::_tags!(layout)]
/// A 2-dimensional layout extent.
pub type UiExt = crate::Extent2<Lunit>;
#[doc = crate::_tags!(layout)]
/// A 2-dimensional layout position.
pub type UiPos = crate::Position2<Lunit>;
#[doc = crate::_tags!(layout)]
/// A 2-dimensional layout region.
pub type UiRect = crate::RegionS2<Lunit>;
#[doc = crate::_tags!(layout)]
/// A 2-dimensional layout stride.
pub type UiStride = crate::Stride2<Lunit>;

crate::bound_int! {
    #[doc = crate::_tags!(layout)]
    /// Scalar unit of UI layout negotiation.
    #[doc = crate::_doc_meta!{
        location("ui/layout"),
        test_size_of(Lunit = 4|32),
    }]
    pub struct Lunit: repr(i32 => i32);
    value_bits(32-4);
    ops(all);

    impl {
        /// Number of subpixels per pixel.
        pub const SUBPIXELS: i32 = 64;
        /// Creates a layout unit from whole pixels.
        ///
        /// Saturates if the scaled value escapes the payload range.
        pub const fn px(px: i32) -> Self {
            Self::new(px.saturating_mul(Self::SUBPIXELS))
        }
    }
}
