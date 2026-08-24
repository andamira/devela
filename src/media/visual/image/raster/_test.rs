// devela/src/media/visual/image/raster/_test.rs

use crate::{Boundary1d, Extent2, ext, pos};
use crate::{RasterBuf, RasterView, RasterViewBytes};
use crate::{RasterByteSlice, RasterElement, RasterFormat, RasterLayout, RasterSlice};

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
    let layout = RasterLayout::interleaved(ext![10_u32, 2], 4, 8, Boundary1d::Upper);
    assert!(!layout.is_valid()); // passed only 8 of the at least 40 needed
    assert_eq!(layout.min_len_bytes(), None);
    assert_eq!(layout.row_offset_bytes(0), None);
    assert_eq!(layout.pixel_offset_bytes(pos![0, 0]), None);
}
#[test]
fn dense_upper_offsets() {
    let layout = RasterLayout::dense_interleaved(ext![3_u32, 2], 4).unwrap();
    assert_eq!(layout.row_offset_bytes(0), Some(0));
    assert_eq!(layout.row_offset_bytes(1), Some(12));
    assert_eq!(layout.pixel_offset_bytes(pos![0, 0]), Some(0));
    assert_eq!(layout.pixel_offset_bytes(pos![2, 0]), Some(8));
    assert_eq!(layout.pixel_offset_bytes(pos![0, 1]), Some(12));
    assert_eq!(layout.pixel_offset_bytes(pos![2, 1]), Some(20));
}
#[test]
fn padded_upper_offsets() {
    let layout = RasterLayout::interleaved(ext![3_u32, 2], 4, 16, Boundary1d::Upper);
    assert!(layout.is_valid());
    assert_eq!(layout.pixel_offset_bytes(pos![2, 1]), Some(24));
    assert_eq!(layout.min_len_bytes(), Some(28));
}
#[test]
fn lower_first_storage_reverses_physical_rows_only() {
    let layout = RasterLayout::interleaved(ext![3_u32, 2], 4, 16, Boundary1d::Lower);
    // Logical upper row is physically second.
    assert_eq!(layout.row_offset_bytes(0), Some(16));
    assert_eq!(layout.row_offset_bytes(1), Some(0));
    assert_eq!(layout.pixel_offset_bytes(pos![2, 0]), Some(24));
    assert_eq!(layout.pixel_offset_bytes(pos![2, 1]), Some(8));
}
#[test]
fn offsets_reject_external_coordinates() {
    let layout = RasterLayout::dense_interleaved(ext![3_u32, 2], 4).unwrap();
    assert_eq!(layout.pixel_offset_bytes(pos![3, 0]), None);
    assert_eq!(layout.pixel_offset_bytes(pos![0, 2]), None);
    assert_eq!(layout.row_offset_bytes(2), None);
}
#[cfg(target_pointer_width = "64")]
#[test]
fn byte_length_can_exceed_u32() {
    let layout = RasterLayout::interleaved(ext![1_u32, 3], 1, u32::MAX, Boundary1d::Upper);
    assert!(layout.is_valid());
    assert_eq!(layout.min_len_bytes(), Some(2 * u32::MAX as usize + 1),);
}
#[test]
fn raster_element_addresses_dense_sample_storage() {
    struct TestRaster {
        extent: Extent2<u32>,
        samples: [u8; 6],
    }
    impl RasterView for TestRaster {
        type Sample = u8;
        fn raster_extent(&self) -> Extent2<u32> {
            self.extent
        }
        fn raster_samples(&self) -> &[u8] {
            &self.samples
        }
    }
    impl RasterBuf for TestRaster {
        fn raster_samples_mut(&mut self) -> &mut [u8] {
            &mut self.samples
        }
    }
    let mut raster = TestRaster { extent: ext![3_u32, 2], samples: [0; 6] };
    let element = RasterElement::full(pos![2_u32, 1]);
    *raster.raster_get_mut(element.coord()).unwrap() = 7;
    assert_eq!(raster.raster_get(pos![2, 1]), Some(&7));
    assert_eq!(raster.samples, [0, 0, 0, 0, 0, 7]);
}
