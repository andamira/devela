// devela::media::image::sixel::quant

#![allow(clippy::identity_op, reason = "symmetry")]

mod diffuse_fns;
use diffuse_fns::*;

use super::{
    PixelFormat, SixelError, SixelMean, SixelQuality, SixelResult, SixelSplit, SIXEL_PALETTE_MAX,
};
use crate::{vec_ as vec, Dither, HashMap, Ordering, Vec};

/// TODO
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct BBox {
    pub ind: i32,
    pub colors: i32,
    pub sum: i32,
}
impl BBox {
    /// Comparing function to be used for ordering
    const fn sum_cmp(b1: &BBox, b2: &BBox) -> Ordering {
        if b2.sum > b1.sum {
            Ordering::Greater
        } else if b2.sum < b1.sum {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

/*
 ** Here is the fun part, the median-cut colormap generator.  This is based
 ** on Paul Heckbert's paper "Color Image Quantization for Frame Buffer
 ** Display", SIGGRAPH '82 Proceedings, page 297.
 */

/// TODO
#[must_use]
fn new_color_map(newcolors: i32, depth: i32) -> HashMap<i32, Tuple> {
    let mut colormap = HashMap::new();
    for i in 0..newcolors {
        colormap.insert(i, Tuple { value: 0, tuple: vec![0; depth as usize] });
    }
    colormap
}

/// TODO
#[must_use]
fn new_box_vector(colors: i32, sum: i32, newcolors: i32) -> Vec<BBox> {
    let mut result = vec![BBox { ind: 0, colors: 0, sum: 0 }; newcolors as usize];

    /* Set up the initial box. */
    result[0].ind = 0;
    result[0].colors = colors;
    result[0].sum = sum;

    result
}

/// Go through the box finding the minimum and maximum of each
/// component - the boundaries of the box.
fn find_box_boundaries(
    colorfreqtable: &mut HashMap<i32, Tuple>,
    depth: i32,
    box_start: i32,
    box_size: i32,
    minval: &mut [i32],
    maxval: &mut [i32],
) {
    for plane in 0..depth {
        minval[plane as usize] = colorfreqtable.get(&(box_start)).unwrap().tuple[plane as usize];
        maxval[plane as usize] = minval[plane as usize];
    }
    for i in 1..box_size {
        for plane in 0..depth {
            let v = colorfreqtable.get(&(box_start + i)).unwrap().tuple[plane as usize];
            minval[plane as usize] = minval[plane as usize].min(v);
            maxval[plane as usize] = maxval[plane as usize].max(v);
        }
    }
}

/// TODO
#[must_use]
#[expect(unused, reason = "WIP")]
fn largest_by_norm(minval: &[i32], maxval: &[i32], depth: i32) -> i32 {
    let mut largest_spread_so_far = 0;
    let mut largest_dimension = 0;
    for plane in 0..depth as usize {
        let spread = maxval[plane] - minval[plane];
        if spread > largest_spread_so_far {
            largest_dimension = plane;
            largest_spread_so_far = spread;
        }
    }
    largest_dimension as i32
}

/// This function presumes that the tuple type is either
/// BLACKANDWHITE, GRAYSCALE, or RGB (which implies pamP->depth is 1 or 3).
/// To save time, we don't actually check it.
#[must_use]
#[expect(unused, reason = "WIP")]
fn largest_by_luminosity(minval: &[i32], maxval: &[i32], depth: i32) -> i32 {
    let retval;
    let lumin_factor = [0.2989, 0.5866, 0.1145];

    if depth == 1 {
        retval = 0;
    } else {
        /* An RGB tuple */
        let mut largest_spread_so_far = 0.0;
        let mut largest_dimension = 0;

        for plane in 0..3 {
            let spread = lumin_factor[plane] * (maxval[plane] - minval[plane]) as f32;
            if spread > largest_spread_so_far {
                largest_dimension = plane;
                largest_spread_so_far = spread;
            }
        }
        retval = largest_dimension;
    }
    retval as i32
}

/// TODO
fn center_box(
    box_start: i32,
    box_size: i32,
    colorfreqtable: &mut HashMap<i32, Tuple>,
    depth: i32,
    new_tuple: &mut [i32],
) {
    for plane in 0..depth {
        let mut maxval = colorfreqtable.get(&(box_start)).unwrap().tuple[plane as usize];
        let mut minval = maxval;

        for i in 1..box_size {
            let v = colorfreqtable.get(&(box_start + i)).unwrap().tuple[plane as usize];
            minval = minval.min(v);
            maxval = maxval.max(v);
        }
        new_tuple[plane as usize] = (minval + maxval) / 2;
    }
}

/// TODO
fn average_colors(
    box_start: i32,
    box_size: i32,
    colorfreqtable: &mut HashMap<i32, Tuple>,
    depth: i32,
    new_tuple: &mut [i32],
) {
    for plane in 0..depth {
        let mut sum = 0;

        for i in 0..box_size {
            sum += colorfreqtable.get(&(box_start + i)).unwrap().tuple[plane as usize];
        }

        new_tuple[plane as usize] = sum / box_size;
    }
}

/// Number of tuples represented by the box
/// Count the tuples in question
fn average_pixels(
    box_start: i32,
    box_size: i32,
    colorfreqtable: &mut HashMap<i32, Tuple>,
    depth: i32,
    new_tuple: &mut [i32],
) {
    let mut n = 0; /* initial value */
    for i in 0..box_size {
        n += colorfreqtable.get(&(box_start + i)).unwrap().value;
    }

    for plane in 0..depth {
        let mut sum = 0;
        for i in 0..box_size {
            sum += colorfreqtable.get(&(box_start + i)).unwrap().tuple[plane as usize]
                * colorfreqtable.get(&(box_start + i)).unwrap().value;
        }
        new_tuple[plane as usize] = sum / n;
    }
}

/// Ok, we've got enough boxes. Now choose a representative color for each box.
///
/// There are a number of possible ways to make this choice.
/// - One would be to choose the center of the box; this ignores any structure
///   within the boxes.
/// - Another method would be to average all the colors in the box.
///   This is the method specified in Heckbert's paper.
/// - A third method is to average all the pixels in the box.
fn color_map_from_bv(
    newcolors: i32,
    bv: &[BBox],
    boxes: i32,
    colorfreqtable: &mut HashMap<i32, Tuple>,
    depth: i32,
    rep: SixelMean,
) -> HashMap<i32, Tuple> {
    let mut colormap = new_color_map(newcolors, depth);

    for bi in 0..boxes {
        match rep {
            SixelMean::Center => {
                center_box(
                    bv[bi as usize].ind,
                    bv[bi as usize].colors,
                    colorfreqtable,
                    depth,
                    &mut colormap.get_mut(&bi).unwrap().tuple,
                );
            }
            SixelMean::Colors => {
                average_colors(
                    bv[bi as usize].ind,
                    bv[bi as usize].colors,
                    colorfreqtable,
                    depth,
                    &mut colormap.get_mut(&bi).unwrap().tuple,
                );
            }
            SixelMean::Auto | SixelMean::Pixels => {
                average_pixels(
                    bv[bi as usize].ind,
                    bv[bi as usize].colors,
                    colorfreqtable,
                    depth,
                    &mut colormap.get_mut(&bi).unwrap().tuple,
                );
            }
        }
    }
    colormap
}

/// Split Box 'bi' in the box vector bv (so that bv contains one more box
/// than it did as input).  Split it so that each new box represents about
/// half of the pixels in the distribution given by 'colorfreqtable' for
/// the colors in the original box, but with distinct colors in each of the
/// two new boxes.
///
/// Assume the box contains at least two colors.
fn split_box(
    bv: &mut [BBox],
    boxes: &mut i32,
    bi: usize,
    colorfreqtable: &mut HashMap<i32, Tuple>,
    depth: i32,
    _largest: SixelSplit,
) -> SixelResult<()> {
    let box_start = bv[bi].ind;
    let box_size = bv[bi].colors;
    let sm = bv[bi].sum;

    let max_depth = 16;
    let mut minval = vec![0; max_depth];
    let mut maxval = vec![0; max_depth];

    /* assert(max_depth >= depth); */

    find_box_boundaries(colorfreqtable, depth, box_start, box_size, &mut minval, &mut maxval);

    /* Find the largest dimension, and sort by that component.  I have
       included two methods for determining the "largest" dimension;
       first by simply comparing the range in RGB space, and second by
       transforming into luminosities before the comparison.
    */
    // let _largest_dimension = match _largest {
    //     SixelSplit::Auto | SixelSplit::Norm => largest_by_norm(&minval, &maxval, depth),
    //     SixelSplit::Lum => largest_by_luminosity(&minval, &maxval, depth),
    // };

    /* TODO: I think this sort should go after creating a box,
       not before splitting.  Because you need the sort to use
       the SIXEL_REP_CENTER_BOX method of choosing a color to
       represent the final boxes
    */

    /* Set the gross global variable 'compareplanePlane' as a
       parameter to compareplane(), which is called by qsort().
    */

    /* Sholdn't be needed - I use a stupid hasmap - should be refactored.
    compareplanePlane = largest_dimension;
    qsort((char*) &colorfreqtable.table[box_start], box_size,
          sizeof(colorfreqtable.table[box_start]),
          compareplane);*/

    /* Now find the median based on the counts, so that about half
    the pixels (not colors, pixels) are in each subdivision.  */
    let mut lowersum = colorfreqtable.get(&box_start).unwrap().value; /* initial value */
    let mut i = 1;
    while i < box_size - 1 && lowersum < sm / 2 {
        lowersum += colorfreqtable.get(&(box_start + i)).unwrap().value;
        i += 1;
    }
    let median_idx = i;
    /* Split the box, and sort to bring the biggest boxes to the top.  */

    bv[bi].colors = median_idx;
    bv[bi].sum = lowersum;
    bv[*boxes as usize].ind = box_start + median_idx;
    bv[*boxes as usize].colors = box_size - median_idx;
    bv[*boxes as usize].sum = sm - lowersum;
    (*boxes) += 1;

    bv[0..*boxes as usize].sort_by(BBox::sum_cmp);
    Ok(())
}

/// Compute a set of only 'newcolors' colors that best represent an
/// image whose pixels are summarized by the histogram
/// 'colorfreqtable'.  Each tuple in that table has depth 'depth'.
/// colorfreqtable.table\[i\] tells the number of pixels in the subject image
/// have a particular color.
///
/// As a side effect, sort 'colorfreqtable'.
fn mediancut(
    colorfreqtable: &mut HashMap<i32, Tuple>,
    depth: i32,
    newcolors: i32,
    largest: SixelSplit,
    rep: SixelMean,
    colormap: &mut HashMap<i32, Tuple>,
) -> SixelResult<()> {
    let mut sum = 0;

    for i in 0..colorfreqtable.len() {
        sum += colorfreqtable.get(&(i as i32)).unwrap().value;
    }

    // There is at least one box that contains at least 2 colors; ergo,
    // there is more splitting we can do.
    let mut bv = new_box_vector(colorfreqtable.len() as i32, sum, newcolors);
    let mut boxes = 1;
    let mut multi_color_boxes_exist = colorfreqtable.len() > 1;

    // Main loop: split boxes until we have enough.
    while boxes < newcolors && multi_color_boxes_exist {
        // Find the first splittable box.
        let mut bi = 0;
        while bi < boxes && bv[bi as usize].colors < 2 {
            bi += 1;
        }

        if bi >= boxes {
            multi_color_boxes_exist = false;
        } else {
            split_box(&mut bv, &mut boxes, bi as usize, colorfreqtable, depth, largest)?;
        }
    }
    *colormap = color_map_from_bv(newcolors, &bv, boxes, colorfreqtable, depth, rep);

    Ok(())
}

/// TODO
fn compute_hash(data: &[u8], i: usize, depth: i32) -> i32 {
    let mut hash = 0;
    for n in 0..depth {
        hash |= (data[i + depth as usize - 1 - n as usize] as i32 >> 3) << (n * 5);
    }
    hash
}

/// TODO
#[derive(Clone)]
struct Tuple {
    pub value: i32,
    pub tuple: Vec<i32>,
}

/// TODO
fn compute_histogram(
    data: &[u8],
    length: i32,
    depth: i32,
    quality: SixelQuality,
) -> SixelResult<HashMap<i32, Tuple>> {
    let (max_sample, mut step) = match quality {
        SixelQuality::Low => (18_383, length / depth / 18_383 * depth),
        SixelQuality::High => (18_383, length / depth / 18_383 * depth),
        SixelQuality::Auto | SixelQuality::HighColor | SixelQuality::Full => {
            (4_003_079, length / depth / 4_003_079 * depth)
        }
    };

    if length < max_sample * depth {
        step = 6 * depth;
    }

    if step <= 0 {
        step = depth;
    }

    let mut histogram = vec![0; 1 << (depth * 5)];

    let mut memory = vec![0; 1 << (depth * 5)];
    let mut it = 0;
    let mut refe = 0;
    let _refmap = 0;

    let mut i = 0;
    while i < length {
        let bucket_index = compute_hash(data, i as usize, 3) as usize;
        if histogram[bucket_index] == 0 {
            memory[refe] = bucket_index;
            refe += 1;
        }
        if histogram[bucket_index] < (1 << (2 * 8)) - 1 {
            histogram[bucket_index] += 1;
        }

        i += step;
    }
    let mut colorfreqtable = HashMap::new();

    for i in 0..refe {
        if histogram[memory[i]] > 0 {
            let mut tuple: Vec<i32> = vec![0; depth as usize];
            for n in 0..depth {
                tuple[(depth - 1 - n) as usize] = ((memory[it] >> (n * 5) & 0x1f) << 3) as i32;
            }
            colorfreqtable.insert(i as i32, Tuple { value: histogram[memory[i]], tuple });
        }
        it += 1;
    }
    Ok(colorfreqtable)
}

/// Produce a colormap containing the best colors to represent the
/// image stream in file 'ifP'.  Figure it out using the median cut
/// technique.
///
/// The colormap will have 'req_colors' or fewer colors in it, unless
/// 'allcolors' is true, in which case it will have all the colors that
/// are in the input.
///
/// The colormap has the same maxval as the input.
///
/// Put the colormap in newly allocated storage as a tupletable2
/// and return its address as *colormap.  Return the number of colors in
/// it as *colorsP and its maxval as *colormapMaxvalP.
///
/// Return the characteristics of the input file as
/// *formatP and *freqPamP.  (This information is not really
/// relevant to our colormap mission; just a fringe benefit).
#[expect(clippy::too_many_arguments)]
fn compute_color_map_from_input(
    data: &[u8],
    length: i32,
    depth: i32,
    req_colors: i32,
    largest: SixelSplit,
    rep: SixelMean,
    quality: SixelQuality,
    colormap: &mut HashMap<i32, Tuple>,
    origcolors: &mut i32,
) -> SixelResult<()> {
    let mut colorfreqtable = compute_histogram(data, length, depth, quality)?;
    *origcolors = colorfreqtable.len() as i32;

    if colorfreqtable.len() as i32 <= req_colors {
        /*
        for i in colorfreqtable.len() as i32..=req_colors {
            let mut tuple: Vec<i32> = vec![0; depth as usize];
            for n in 0..depth {
                tuple[n as usize] = (i * depth) + n;
            }
            colorfreqtable.insert(i, Tuple { value: i, tuple });
        }*/

        for i in 0..colorfreqtable.len() as i32 {
            colormap.insert(i, colorfreqtable.get(&i).unwrap().clone());
        }
    } else {
        mediancut(&mut colorfreqtable, depth, req_colors, largest, rep, colormap)?;
    }
    Ok(())
}

/// No diffusion.
#[rustfmt::skip]
fn diffuse_none(_: &mut [u8], _: i32, _h: i32, _x: i32, _y: i32, _: i32, _: i32) {}

/// TODO
#[must_use]
const fn mask_a(x: i32, y: i32, c: i32) -> f32 {
    ((((x + c * 67) + y * 236) * 119) & 255) as f32 / 128.0 - 1.0
}

/// TODO
#[must_use]
const fn mask_x(x: i32, y: i32, c: i32) -> f32 {
    ((((x + c * 29) ^ (y * 149)) * 1234) & 511) as f32 / 256.0 - 1.0
}

/// Lookup closest color from palette with "normal" strategy.
#[must_use]
fn lookup_normal(
    pixel: &[u8],
    depth: i32,
    palette: &[u8],
    reqcolor: i32,
    _cachetable: &mut [u16],
    complexion: i32,
) -> i32 {
    let mut result = -1;
    let mut diff = i32::MAX;

    /* don't use cachetable in 'normal' strategy */

    for i in 0..reqcolor {
        let mut distant = 0;
        let mut r = pixel[0] as i32 - palette[(i * depth + 0) as usize] as i32;
        distant += r * r * complexion;
        for n in 1..depth {
            r = pixel[n as usize] as i32 - palette[(i * depth + n) as usize] as i32;
            distant += r * r;
        }
        if distant < diff {
            diff = distant;
            result = i;
        }
    }

    result
}

/// lookup closest color from palette with "fast" strategy.
fn lookup_fast(
    pixel: &[u8],
    _depth: i32,
    palette: &[u8],
    reqcolor: i32,
    cachetable: &mut [u16],
    complexion: i32,
) -> i32 {
    let mut result: i32 = -1;
    let mut diff = i32::MAX;
    let hash = compute_hash(pixel, 0, 3);

    let cache = cachetable[hash as usize];
    if cache != 0 {
        /* fast lookup */
        return cache as i32 - 1;
    }
    /* collision */
    for i in 0..reqcolor {
        /*          distant = 0;
         #if 0
                for (n = 0; n < 3; ++n) {
                    r = pixel[n] - palette[i * 3 + n];
                    distant += r * r;
                }
        #elif 1*/
        /* complexion correction */
        let i = i as usize;
        let distant = (pixel[0] as i32 - palette[i * 3 + 0] as i32)
            * (pixel[0] as i32 - palette[i * 3 + 0] as i32)
            * complexion
            + (pixel[1] as i32 - palette[i * 3 + 1] as i32)
                * (pixel[1] as i32 - palette[i * 3 + 1] as i32)
            + (pixel[2] as i32 - palette[i * 3 + 2] as i32)
                * (pixel[2] as i32 - palette[i * 3 + 2] as i32);
        //  #endif
        if distant < diff {
            diff = distant;
            result = i as i32;
        }
    }
    cachetable[hash as usize] = (result + 1) as u16;

    result
}

/// TODO
fn lookup_mono_darkbg(
    pixel: &[u8],
    depth: i32,
    _palette: &[u8],
    reqcolor: i32,
    _cachetable: &mut [u16],
    _complexion: i32,
) -> i32 {
    let mut distant = 0;
    for n in 0..depth {
        distant += pixel[n as usize] as i32;
    }
    i32::from(distant >= 128 * reqcolor)
}

/// TODO
fn lookup_mono_lightbg(
    pixel: &[u8],
    depth: i32,
    _palette: &[u8],
    reqcolor: i32,
    _cachetable: &mut [u16],
    _complexion: i32,
) -> i32 {
    let mut distant = 0;
    for n in 0..depth {
        distant += pixel[n as usize] as i32;
    }
    i32::from(distant < 128 * reqcolor)
}

/// Choose colors using median-cut method.
//
// Called from DitherConf::initialize
#[expect(clippy::too_many_arguments)]
pub(crate) fn sixel_quant_make_palette(
    data: &[u8],
    length: i32,
    pixelformat: PixelFormat,
    req_colors: i32,
    ncolors: &mut i32,
    origcolors: &mut i32,
    largest: SixelSplit,
    rep: SixelMean,
    quality: SixelQuality,
) -> SixelResult<Vec<u8>> {
    let depth = pixelformat.bytes_per_pixel();
    // if (result_depth <= 0) { *result = NULL; goto end; }

    let mut colormap = HashMap::new();
    let _ = compute_color_map_from_input(
        data,
        length,
        depth as i32,
        req_colors,
        largest,
        rep,
        quality,
        &mut colormap,
        origcolors,
    );
    *ncolors = colormap.len() as i32;
    let mut result = vec![0; colormap.len() * depth];
    for i in 0..colormap.len() {
        for n in 0..depth {
            result[i * depth + n] = colormap.get(&(i as i32)).unwrap().tuple[n] as u8;
        }
    }
    Ok(result)
}

/// Apply color palette to the given pixel buffer.
//
// Called from DitherConf::initialize
#[expect(clippy::too_many_arguments)]
pub(crate) fn sixel_quant_apply_palette(
    result: &mut [u8],
    data: &mut [u8],
    width: i32,
    height: i32,
    depth: i32,
    palette: &mut Vec<u8>,
    reqcolor: i32,
    diffuse: Dither,
    foptimize: bool,
    foptimize_palette: bool,
    complexion: i32,
    cachetable: Option<&mut [u16]>,
) -> SixelResult<i32> {
    let mut ncolors: i32;
    // check bad reqcolor
    if reqcolor < 1 {
        // sixel_helper_set_additional_message(
        // "sixel_quant_apply_palette: "
        // "a bad argument is detected, reqcolor < 0.");
        return Err(SixelError::BadArgument);
    }

    let mut f_mask = false;

    let f_diffuse = if depth != 3 {
        diffuse_none
    } else {
        match diffuse {
            Dither::Auto | Dither::None => diffuse_none,
            Dither::Atkinson => diffuse_atkinson,
            Dither::FS => diffuse_fs,
            Dither::JaJuNi => diffuse_jajuni,
            Dither::Stucki => diffuse_stucki,
            Dither::Burkes => diffuse_burkes,
            Dither::ADither => {
                f_mask = true;
                diffuse_none
            }
            Dither::XDither => {
                f_mask = true;
                diffuse_none
            }
        }
    };
    type LookupFunc = fn(&[u8], i32, &[u8], i32, &mut [u16], i32) -> i32;
    let mut f_lookup: Option<LookupFunc> = None;
    if reqcolor == 2 {
        let mut sum1 = 0;
        let mut sum2 = 0;
        for n in 0..depth {
            sum1 += palette[n as usize] as i32;
        }
        for n in depth..(depth + depth) {
            sum2 += palette[n as usize] as i32;
        }
        if sum1 == 0 && sum2 == 255 * 3 {
            f_lookup = Some(lookup_mono_darkbg);
        } else if sum1 == 255 * 3 && sum2 == 0 {
            f_lookup = Some(lookup_mono_lightbg);
        }
    }
    if f_lookup.is_none() {
        if foptimize && depth == 3 {
            f_lookup = Some(lookup_fast);
        } else {
            f_lookup = Some(lookup_normal);
        }
    }

    let mut cc = vec![0u16, 1 << (depth * 5)];
    let indextable = match cachetable {
        Some(table) => table,
        None => &mut cc,
    };

    if foptimize_palette {
        ncolors = 0;
        let mut new_palette = vec![0; SIXEL_PALETTE_MAX * depth as usize];
        let mut migration_map = vec![0; SIXEL_PALETTE_MAX];

        if f_mask {
            for y in 0..height {
                for x in 0..width {
                    let mut copy: Vec<u8> = Vec::new();

                    let pos = y * width + x;
                    for d in 0..depth {
                        let mut val = data[(pos * depth + d) as usize] as i32;
                        if matches!(diffuse, Dither::ADither) {
                            val += (mask_a(x, y, d) * 32.0) as i32;
                        } else {
                            val += (mask_x(x, y, d) * 32.0) as i32;
                        }
                        copy.push(val.clamp(0, 255) as u8);
                    }
                    //                    &[u8],  i32,   &[u8],    i32,    &mut [u16],    i32
                    let color_index =
                        f_lookup.unwrap()(&copy, depth, palette, reqcolor, indextable, complexion)
                            as usize;
                    if migration_map[color_index] == 0 {
                        result[pos as usize] = ncolors as u8;
                        for n in 0..depth {
                            new_palette[(ncolors * depth + n) as usize] =
                                palette[color_index * depth as usize + n as usize];
                        }
                        ncolors += 1;
                        migration_map[color_index] = ncolors;
                    } else {
                        result[pos as usize] = migration_map[color_index] as u8 - 1;
                    }
                }
            }
            *palette = new_palette;
        } else {
            for y in 0..height {
                for x in 0..width {
                    let pos = y * width + x;
                    let color_index = f_lookup.unwrap()(
                        &data[(pos * depth) as usize..],
                        depth,
                        palette,
                        reqcolor,
                        indextable,
                        complexion,
                    ) as usize;
                    if migration_map[color_index] == 0 {
                        result[pos as usize] = ncolors as u8;
                        for n in 0..depth {
                            new_palette[(ncolors * depth + n) as usize] =
                                palette[color_index * depth as usize + n as usize];
                        }
                        ncolors += 1;
                        migration_map[color_index] = ncolors;
                    } else {
                        result[pos as usize] = migration_map[color_index] as u8 - 1;
                    }
                    for n in 0..depth {
                        let offset = data[(pos * depth + n) as usize] as i32
                            - palette[color_index * depth as usize + n as usize] as i32;
                        f_diffuse(&mut data[n as usize..], width, height, x, y, depth, offset);
                    }
                }
            }
            *palette = new_palette;
        }
    } else {
        if f_mask {
            for y in 0..height {
                for x in 0..width {
                    let mut copy = Vec::new();
                    let pos = y * width + x;
                    for d in 0..depth {
                        let mut val = data[(pos * depth + d) as usize] as i32;
                        if matches!(diffuse, Dither::ADither) {
                            val += (mask_a(x, y, d) * 32.0) as i32;
                        } else {
                            val += (mask_x(x, y, d) * 32.0) as i32;
                        }

                        copy.push(val.clamp(0, 255) as u8);
                    }
                    result[pos as usize] = f_lookup.unwrap()(
                        &mut copy, depth, palette, reqcolor, indextable, complexion,
                    ) as u8;
                }
            }
        } else {
            for y in 0..height {
                for x in 0..width {
                    let pos = y * width + x;
                    let color_index = f_lookup.unwrap()(
                        &mut data[(pos * depth) as usize..],
                        depth,
                        palette,
                        reqcolor,
                        indextable,
                        complexion,
                    ) as usize;
                    result[pos as usize] = color_index as u8;
                    for n in 0..depth {
                        let offset = data[(pos * depth + n) as usize] as i32
                            - palette[color_index * depth as usize + n as usize] as i32;
                        f_diffuse(&mut data[n as usize..], width, height, x, y, depth, offset);
                    }
                }
            }
        }
        ncolors = reqcolor;
    }

    Ok(ncolors)
}
