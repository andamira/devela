// devela/src/media/visual/image/raster/_test.rs

use crate::{Boundary1d, ext};
use crate::{
    RasterByteSlice, RasterFormat, RasterLayout, RasterSlice, RasterView, RasterViewBytes,
};

#[test]
fn typed_raster_rejects_zst() {
    let extent = ext![2_u32, 2];
    let samples = [(); 4];
    assert!(RasterSlice::dense(RasterFormat::default(), extent, &samples).is_none());
}
#[test]
fn typed_raster_rejects_format_width_mismatch() {
    let extent = ext![2_u32, 2];
    let layout = RasterLayout::dense_interleaved(extent, 4).unwrap();
    let samples = [0_u32; 4];
    // A three-byte stored format must not accept four-byte samples.
    assert!(RasterSlice::new(RasterFormat::RGB8, layout, &samples).is_none());
}
#[test]
fn typed_raster_rejects_lower_first_layout() {
    let extent = ext![2_u32, 2];
    let layout = RasterLayout::interleaved(extent, 4, 8, Boundary1d::Lower);
    let samples = [0_u32; 4];
    assert!(RasterSlice::new(RasterFormat::XRGB8888, layout, &samples).is_none());
}
#[test]
fn typed_raster_hides_trailing_samples() {
    let extent = ext![2_u32, 2];
    let samples = [0_u32; 8];
    let raster = RasterSlice::dense(RasterFormat::XRGB8888, extent, &samples).unwrap();
    assert_eq!(raster.samples().len(), 4);
    assert_eq!(raster.raster_samples().len(), 4);
}
#[test]
fn byte_raster_reports_padding_separately() {
    let extent = ext![3_u32, 2];
    // Three 3-byte pixels use 9 bytes; rows are padded to 12.
    let layout = RasterLayout::interleaved(extent, 3, 12, Boundary1d::Lower);
    let bytes = [0_u8; 24];
    let raster = RasterByteSlice::new(RasterFormat::RGB8, layout, &bytes).unwrap();
    assert_eq!(raster.raster_bytes_per_pixel_bytes(), 3);
    assert_eq!(raster.raster_bits_per_pixel_bytes(), Some(24));
    assert_eq!(raster.raster_bytes_per_line(), 12);
    assert_eq!(raster.raster_row_start_bytes(), Boundary1d::Lower,);
}
#[test]
fn layout_rejects_overlapping_rows() {
    let layout = RasterLayout::interleaved(
        ext![10_u32, 2],
        4,
        8, // Needs at least 40
        Boundary1d::Upper,
    );
    assert!(!layout.is_valid());
    assert_eq!(layout.min_len_bytes(), None);
}
