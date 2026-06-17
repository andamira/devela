// devela/src/media/visual/image/format/netpbm/tests.rs

use crate::{ImageError, ImageResult, Pnm};

/* common mechanics*/

type EncodeFn = fn(&[u8], usize, usize, &mut [u8]) -> ImageResult<usize>;

fn assert_decode<const N: usize>(
    pnm: &[u8],
    expected_extent: [usize; 2],
    expected_channels: u8,
    expected: [u8; N],
) {
    let mut out = [0u8; N];
    let (extent, channels) = Pnm::decode_u8(pnm, &mut out).expect("PNM decoded");
    assert_eq![extent.dim, expected_extent];
    assert_eq![channels, expected_channels];
    assert_eq![out, expected];
}
fn assert_encode_decode<const OUT: usize, const N: usize>(
    encode: EncodeFn,
    samples: [u8; N],
    width: usize,
    height: usize,
    expected_encoded: &[u8],
    expected_channels: u8,
) {
    let mut encoded = [0u8; OUT];
    let len = encode(&samples, width, height, &mut encoded).expect("PNM encoded");
    assert_eq![&encoded[..len], expected_encoded];
    assert_decode(&encoded[..len], [width, height], expected_channels, samples);
}
fn assert_encode_insufficient<const OUT: usize>(
    encode: EncodeFn,
    samples: &[u8],
    width: usize,
    height: usize,
    needed: usize,
    available: usize,
) {
    let mut out = [0u8; OUT];
    let err = encode(samples, width, height, &mut out).expect_err("buffer must be too small");
    assert_eq![err, ImageError::InsufficientBuffer { needed, available }];
}
fn assert_encode_invalid_pixel<const OUT: usize>(
    encode: EncodeFn,
    samples: &[u8],
    width: usize,
    height: usize,
) {
    let mut out = [0u8; OUT];
    let err = encode(samples, width, height, &mut out).expect_err("bitmap samples must be 0 or 1");
    assert_eq![err, ImageError::InvalidPixel];
}

/* P1 */

#[test]
fn encode_decode_p1_bitmap() {
    assert_encode_decode::<32, 4>(
        Pnm::encode_p1_bitmap,
        [0, 1, 1, 0],
        2,
        2,
        b"P1\n2 2\n0 1\n1 0\n",
        1,
    );
}
#[test]
fn encode_p1_reports_insufficient_output_buffer() {
    assert_encode_insufficient::<8>(Pnm::encode_p1_bitmap, &[0, 1, 1, 0], 2, 2, 15, 8);
}
#[test]
fn encode_p1_rejects_non_binary_sample() {
    assert_encode_invalid_pixel::<32>(Pnm::encode_p1_bitmap, &[0, 1, 2, 0], 2, 2);
}
#[test]
fn decode_p1_ascii_bitmap() {
    assert_decode(b"P1\n# two by two\n2 2\n0 1\n1 0\n", [2, 2], 1, [0, 1, 1, 0]);
}

/* P2 */

#[test]
fn encode_decode_p2_gray8() {
    assert_encode_decode::<32, 3>(
        Pnm::encode_p2_gray8,
        [0, 127, 255],
        3,
        1,
        b"P2\n3 1\n255\n0 127 255\n",
        1,
    );
}
#[test]
fn encode_p2_reports_insufficient_output_buffer() {
    assert_encode_insufficient::<8>(Pnm::encode_p2_gray8, &[0, 127, 255], 3, 1, 21, 8);
}
#[test]
fn decode_p2_ascii_gray_preserves_values() {
    assert_decode(b"P2\n3 1\n15\n0 7 15\n", [3, 1], 1, [0, 7, 15]);
}

/* P3 */

#[test]
fn extent_and_decoded_len_p3() {
    let pnm = b"P3\n2 1\n255\n255 0 0 0 255 0\n";

    let extent = Pnm::extent(pnm).expect("PNM extent");
    let len = Pnm::decoded_len_u8(pnm).expect("PNM decoded length");

    assert_eq![extent.dim, [2, 1]];
    assert_eq![len, 6];
}
#[test]
fn encode_decode_p3_rgb8() {
    assert_encode_decode::<48, 6>(
        Pnm::encode_p3_rgb8,
        [255, 0, 0, 0, 255, 0],
        2,
        1,
        b"P3\n2 1\n255\n255 0 0 0 255 0\n",
        3,
    );
}
#[test]
fn encode_p3_reports_insufficient_output_buffer() {
    assert_encode_insufficient::<8>(Pnm::encode_p3_rgb8, &[255, 0, 0, 0, 255, 0], 2, 1, 27, 8);
}
#[test]
fn decode_p3_ascii_rgb_with_comments() {
    assert_decode(
        b"P3\n# comment\n2 1\n255\n255 0 0  0 255 0\n",
        [2, 1],
        3,
        [255, 0, 0, 0, 255, 0],
    );
}

/* P4 */

const P4_BITS_10X2: [u8; 20] = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];

#[test]
fn encode_decode_p4_bitmap() {
    assert_encode_decode::<32, 20>(
        Pnm::encode_p4_bitmap,
        P4_BITS_10X2,
        10,
        2,
        b"P4\n10 2\n\xaa\x80\x55\x40",
        1,
    );
}
#[test]
fn encode_p4_reports_insufficient_output_buffer() {
    assert_encode_insufficient::<8>(Pnm::encode_p4_bitmap, &P4_BITS_10X2, 10, 2, 12, 8);
}
#[test]
fn encode_p4_rejects_non_binary_sample() {
    assert_encode_invalid_pixel::<32>(Pnm::encode_p4_bitmap, &[0, 1, 2, 0], 2, 2);
}
#[test]
fn decode_p4_raw_bitmap_rows_are_padded() {
    assert_decode(b"P4\n10 2\n\xaa\x80\x55\x40", [10, 2], 1, P4_BITS_10X2);
}

/* P5 */

#[test]
fn encode_decode_p5_gray8() {
    assert_encode_decode::<32, 3>(
        Pnm::encode_p5_gray8,
        [0, 127, 255],
        3,
        1,
        b"P5\n3 1\n255\n\0\x7f\xff",
        1,
    );
}
#[test]
fn encode_p5_reports_insufficient_output_buffer() {
    assert_encode_insufficient::<8>(Pnm::encode_p5_gray8, &[0, 127, 255], 3, 1, 14, 8);
}

/* P6 */

#[test]
fn encode_decode_p6_rgb8() {
    assert_encode_decode::<64, 6>(
        Pnm::encode_p6_rgb8,
        [255, 0, 0, 0, 255, 0],
        2,
        1,
        b"P6\n2 1\n255\n\xff\0\0\0\xff\0",
        3,
    );
}

/* misc. */

#[test]
fn decode_reports_insufficient_output_buffer() {
    let pnm = b"P6\n2 1\n255\n\xff\0\0\0\xff\0";
    let mut out = [0u8; 5];
    let err = Pnm::decode_u8(pnm, &mut out).expect_err("buffer must be too small");
    assert_eq![err, ImageError::InsufficientBuffer { needed: 6, available: 5 }];
}
#[test]
fn invalid_magic_number() {
    let pnm = b"P9\n1 1\n255\n0";
    let err = Pnm::decoded_len_u8(pnm).expect_err("magic number must be invalid");
    assert_eq![err, ImageError::InvalidMagicNumber];
}
#[test]
fn decode_rejects_u16_samples_for_u8_output() {
    let pnm = b"P2\n1 1\n256\n0\n";
    let mut out = [0u8; 1];
    let err = Pnm::decode_u8(pnm, &mut out).expect_err("max value must be unsupported");
    assert_eq![err, ImageError::InvalidPixel];
}
