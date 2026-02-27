// devela_base_core::media::visual::color::rgb::definitions
//
//! Defines the [`Rgb`] and [`Rgba`] types.
//

#[doc = crate::_tags!(color)]
/// RGB color with 3 channels.
#[doc = crate::_doc_location!("media/visual/color")]
#[repr(C)]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rgb<T, const LINEAR: bool = false> {
    /// Color channels in order: [red, green, blue].
    ///
    /// - **Red/Green/Blue**: Gamma-encoded luminosity (0..=255).
    pub c: [T; 3],
}

#[doc = crate::_tags!(color)]
/// RGB+A color with 4 channels.
#[doc = crate::_doc_location!("media/visual/color")]
#[repr(C)]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rgba<T, const LINEAR: bool = false, const PREMUL: bool = false> {
    /// Color channels in order: [red, green, blue, alpha].
    ///
    /// - **Red/Green/Blue**: Gamma-encoded luminosity (0..=255).
    /// - **Alpha**: Linear opacity (0 = transparent, 255 = opaque).
    pub c: [T; 4],
}

/* aliases */

#[doc = crate::_tags!(color)]
/// RGB color with 8-bit integer channels (sRGB gamma space).
#[doc = crate::_doc_location!("media/visual/color")]
pub type Rgb8 = Rgb<u8>;

#[doc = crate::_tags!(color)]
/// RGB+A color with 8-bit integer channels (sRGB gamma space, straight alpha).
#[doc = crate::_doc_location!("media/visual/color")]
pub type Rgba8 = Rgba<u8>;

#[doc = crate::_tags!(color)]
/// RGB+A color with 8-bit integer channels (sRGB gamma space, premultiplied alpha).
#[doc = crate::_doc_location!("media/visual/color")]
pub type RgbaPre8 = Rgba<u8, false, true>;

#[doc = crate::_tags!(color)]
/// RGB color with 16-bit integer channels (sRGB gamma space).
#[doc = crate::_doc_location!("media/visual/color")]
pub type Rgb16 = Rgb<u16>;

#[doc = crate::_tags!(color)]
/// RGB+A color with 16-bit integer channels (sRGB gamma space, straight alpha).
#[doc = crate::_doc_location!("media/visual/color")]
pub type Rgba16 = Rgba<u16>;

#[doc = crate::_tags!(color)]
/// RGB+A color with 16-bit integer channels (sRGB gamma space, premultiplied alpha).
#[doc = crate::_doc_location!("media/visual/color")]
pub type RgbaPre16 = Rgba<u16, false, true>;

crate::items! {
    #[doc = crate::_tags!(color)]
    /// RGB color with 32-bit float channels (sRGB gamma space).
    #[doc = crate::_doc_location!("media/visual/color")]
    pub type RgbF32 = Rgb<f32>;

    #[doc = crate::_tags!(color)]
    /// RGB+A color with 32-bit float channels (sRGB gamma space, straight alpha).
    #[doc = crate::_doc_location!("media/visual/color")]
    pub type RgbaF32 = Rgba<f32>;

    #[doc = crate::_tags!(color)]
    /// RGB+A color with 32-bit float channels (sRGB gamma space, premultiplied alpha).
    #[doc = crate::_doc_location!("media/visual/color")]
    pub type RgbaPreF32 = Rgba<f32, false, true>;


    #[doc = crate::_tags!(color)]
    /// RGB color with 32-bit float channels (linear space).
    #[doc = crate::_doc_location!("media/visual/color")]
    pub type RgbLinF32 = Rgb<f32, true>;

    #[doc = crate::_tags!(color)]
    /// RGB+A color with 32-bit float channels (linear space, straight alpha).
    #[doc = crate::_doc_location!("media/visual/color")]
    pub type RgbaLinF32 = Rgba<f32, true>;

    #[doc = crate::_tags!(color)]
    /// RGB+A color with 32-bit float channels (linear space, premultiplied alpha).
    #[doc = crate::_doc_location!("media/visual/color")]
    pub type RgbaLinPreF32 = Rgba<f32, true, true>;
}
crate::items! {
    #[doc = crate::_tags!(color)]
    /// RGB color with 64-bit float channels (sRGB gamma space).
    #[doc = crate::_doc_location!("media/visual/color")]
    pub type RgbF64 = Rgb<f64>;

    #[doc = crate::_tags!(color)]
    /// RGB+A color with 64-bit float channels (sRGB gamma space, straight alpha).
    #[doc = crate::_doc_location!("media/visual/color")]
    pub type RgbaF64 = Rgba<f64>;

    #[doc = crate::_tags!(color)]
    /// RGB+A color with 64-bit float channels (sRGB gamma space, premultiplied alpha).
    #[doc = crate::_doc_location!("media/visual/color")]
    pub type RgbaPreF64 = Rgba<f64, false, true>;


    #[doc = crate::_tags!(color)]
    /// RGB color with 64-bit float channels (linear space).
    #[doc = crate::_doc_location!("media/visual/color")]
    pub type RgbLinF64 = Rgb<f64, true>;

    #[doc = crate::_tags!(color)]
    /// RGB+A color with 64-bit float channels (linear space, straight alpha).
    #[doc = crate::_doc_location!("media/visual/color")]
    pub type RgbaLinF64 = Rgba<f64, true>;

    #[doc = crate::_tags!(color)]
    /// RGB+A color with 64-bit float channels (linear space, premultiplied alpha).
    #[doc = crate::_doc_location!("media/visual/color")]
    pub type RgbaLinPreF64 = Rgba<f64, true, true>;
}

#[cfg(test)]
const _TEST_RGB_SIZES: () = {
    assert![size_of::<Rgb<u8>>() == 3];
    assert![size_of::<Rgba<u8>>() == 4];
    assert![size_of::<Rgb<u16>>() == 6];
    assert![size_of::<Rgba<u16>>() == 8];

    assert![size_of::<Rgb<f32>>() == 12];
    assert![size_of::<Rgba<f32>>() == 16];
    assert![size_of::<Rgb<f64>>() == 24];
    assert![size_of::<Rgba<f64>>() == 32];

    assert![align_of::<Rgb<u8>>() == 1];
    assert![align_of::<Rgba<u8>>() == 1];
    assert![align_of::<Rgb<u16>>() == 2];
    assert![align_of::<Rgba<u16>>() == 2];

    assert![align_of::<Rgb<f32>>() == 4];
    assert![align_of::<Rgba<f32>>() == 4];
    assert![align_of::<Rgb<f64>>() == 8];
    assert![align_of::<Rgba<f64>>() == 8];
};
