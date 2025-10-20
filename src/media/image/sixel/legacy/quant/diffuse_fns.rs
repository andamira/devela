// devela::media::image::sixel::quant::diffuse_fns

#![allow(clippy::erasing_op, clippy::identity_op, reason = "symmetry")]

/// Diffuses error energy to surround pixels.
fn error_diffuse(
    data: &mut [u8],  /* base address of pixel buffer */
    pos: i32,         /* address of the destination pixel */
    depth: i32,       /* color depth in bytes */
    error: i32,       /* error energy */
    numerator: i32,   /* numerator of diffusion coefficient */
    denominator: i32, /* denominator of diffusion coefficient */
) {
    let offset = (pos * depth) as usize;

    let mut c = data[offset] as i32 + error * numerator / denominator;
    if c < 0 {
        c = 0;
    }
    if c >= 1 << 8 {
        c = (1 << 8) - 1;
    }
    data[offset] = c as u8;
}

/// Floyd Steinberg diffuse
///
/// ```txt
///          curr    7/16
///  3/16    5/48    1/16
/// ```
pub(super) fn diffuse_fs(
    data: &mut [u8],
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    depth: i32,
    error: i32,
) {
    let pos = y * width + x;
    if x < width - 1 && y < height - 1 {
        // add error to the right cell
        error_diffuse(data, pos + width * 0 + 1, depth, error, 7, 16);
        // add error to the left-bottom cell
        error_diffuse(data, pos + width * 1 - 1, depth, error, 3, 16);
        // add error to the bottom cell
        error_diffuse(data, pos + width * 1 + 0, depth, error, 5, 16);
        // add error to the right-bottom cell
        error_diffuse(data, pos + width * 1 + 1, depth, error, 1, 16);
    }
}

/// Atkinson's diffuse
///
/// ```txt
///          curr    1/8    1/8
///   1/8     1/8    1/8
///           1/8
/// ```
pub(super) fn diffuse_atkinson(
    data: &mut [u8],
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    depth: i32,
    error: i32,
) {
    let pos = y * width + x;
    if y < height - 2 {
        // add error to the right cell
        error_diffuse(data, pos + width * 0 + 1, depth, error, 1, 8);
        // add error to the 2th right cell
        error_diffuse(data, pos + width * 0 + 2, depth, error, 1, 8);
        // add error to the left-bottom cell
        error_diffuse(data, pos + width * 1 - 1, depth, error, 1, 8);
        // add error to the bottom cell
        error_diffuse(data, pos + width * 1 + 0, depth, error, 1, 8);
        // add error to the right-bottom cell
        error_diffuse(data, pos + width * 1 + 1, depth, error, 1, 8);
        // add error to the 2th bottom cell
        error_diffuse(data, pos + width * 2 + 0, depth, error, 1, 8);
    }
}

/// Jarvis, Judice & Ninke diffusion
///
/// ```txt
///                  curr    7/48    5/48
///  3/48    5/48    7/48    5/48    3/48
///  1/48    3/48    5/48    3/48    1/48
/// ```
pub(super) fn diffuse_jajuni(
    data: &mut [u8],
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    depth: i32,
    error: i32,
) {
    let pos = y * width + x;
    if pos < (height - 2) * width - 2 {
        error_diffuse(data, pos + width * 0 + 1, depth, error, 7, 48);
        error_diffuse(data, pos + width * 0 + 2, depth, error, 5, 48);
        error_diffuse(data, pos + width * 1 - 2, depth, error, 3, 48);
        error_diffuse(data, pos + width * 1 - 1, depth, error, 5, 48);
        error_diffuse(data, pos + width * 1 + 0, depth, error, 7, 48);
        error_diffuse(data, pos + width * 1 + 1, depth, error, 5, 48);
        error_diffuse(data, pos + width * 1 + 2, depth, error, 3, 48);
        error_diffuse(data, pos + width * 2 - 2, depth, error, 1, 48);
        error_diffuse(data, pos + width * 2 - 1, depth, error, 3, 48);
        error_diffuse(data, pos + width * 2 + 0, depth, error, 5, 48);
        error_diffuse(data, pos + width * 2 + 1, depth, error, 3, 48);
        error_diffuse(data, pos + width * 2 + 2, depth, error, 1, 48);
    }
}

/// Stucki's diffusion
///
/// ```txt
///                  curr    8/48    4/48
///  2/48    4/48    8/48    4/48    2/48
///  1/48    2/48    4/48    2/48    1/48
/// ```
pub(super) fn diffuse_stucki(
    data: &mut [u8],
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    depth: i32,
    error: i32,
) {
    let pos = y * width + x;
    if pos < (height - 2) * width - 2 {
        error_diffuse(data, pos + width * 0 + 1, depth, error, 1, 6);
        error_diffuse(data, pos + width * 0 + 2, depth, error, 1, 12);
        error_diffuse(data, pos + width * 1 - 2, depth, error, 1, 24);
        error_diffuse(data, pos + width * 1 - 1, depth, error, 1, 12);
        error_diffuse(data, pos + width * 1 + 0, depth, error, 1, 6);
        error_diffuse(data, pos + width * 1 + 1, depth, error, 1, 12);
        error_diffuse(data, pos + width * 1 + 2, depth, error, 1, 24);
        error_diffuse(data, pos + width * 2 - 2, depth, error, 1, 48);
        error_diffuse(data, pos + width * 2 - 1, depth, error, 1, 24);
        error_diffuse(data, pos + width * 2 + 0, depth, error, 1, 12);
        error_diffuse(data, pos + width * 2 + 1, depth, error, 1, 24);
        error_diffuse(data, pos + width * 2 + 2, depth, error, 1, 48);
    }
}

/// Burkes' diffusion
///
/// ```txt
///                  curr    4/16    2/16
///  1/16    2/16    4/16    2/16    1/16
/// ```
pub(super) fn diffuse_burkes(
    data: &mut [u8],
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    depth: i32,
    error: i32,
) {
    let pos = y * width + x;
    if pos < (height - 1) * width - 2 {
        error_diffuse(data, pos + width * 0 + 1, depth, error, 1, 4);
        error_diffuse(data, pos + width * 0 + 2, depth, error, 1, 8);
        error_diffuse(data, pos + width * 1 - 2, depth, error, 1, 16);
        error_diffuse(data, pos + width * 1 - 1, depth, error, 1, 8);
        error_diffuse(data, pos + width * 1 + 0, depth, error, 1, 4);
        error_diffuse(data, pos + width * 1 + 1, depth, error, 1, 8);
        error_diffuse(data, pos + width * 1 + 2, depth, error, 1, 16);
    }
}
