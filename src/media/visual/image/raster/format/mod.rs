// devela::media::visual::image::raster::format
//
//! Raster image formats.
//
/* maintenance invariants
   ---------------------------------------------------------------------------
   - `RasterChannels` describes stored channel topology.
   - `RasterSampleFormat` describes the scalar/sample representation.
   - `RasterTransfer` describes how color values are interpreted.
   - `RasterAlpha` describes semantic alpha behavior.

   Packed formats:
   - `RasterChannels::Packed(p)` means one scalar element contains several channel bit fields.
   - `RasterPackedFormat` variant names describe scalar bit-field order from
     most-significant field to least-significant field.
   - Packed names do not describe native byte order. Byte slices still follow
     the target endianness of the scalar representation.

   Alpha consistency:
   - Formats with alpha-bearing channel topology should use either
     `RasterAlpha::Straight` or `RasterAlpha::Premultiplied`.
   - Formats with `X` padding fields should use `RasterAlpha::Opaque`, not
     `RasterAlpha::Straight`.
   - Formats with no alpha field should normally use `RasterAlpha::None`.
   - `RasterFormat::has_alpha()` reports semantic alpha.
   - `RasterFormat::has_alpha_field()` reports stored alpha topology.

   Depth and storage:
   - `depth_bits()` excludes padding bits but includes alpha bits.
   - `bits_per_pixel()` includes padding bits.
   - For `Xrgb8888`, depth is 24 but storage is 32 bits.
   - For `Rgb555`, depth is 15 but storage is usually 16 bits.
*/

mod impl_const;

mod base; // RasterFormat, (RasterTransfer, RasterAlpha)
mod channels; // (RasterChannels)
mod packed; // (RasterPackedFormat)
mod sample; // (RasterSampleFormat)

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            base::*,
            channels::*,
            packed::*,
            sample::*,
        };
    }
}
