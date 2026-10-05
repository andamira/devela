//

use devela::Scale;

#[unsafe(no_mangle)]
#[inline(never)]
pub fn raw_u8_runtime(x: u8, n: u8, d: u8) -> u8 {
    x * n / d
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn wide_u8_runtime(x: u8, n: u8, d: u8) -> u8 {
    ((x as u16 * n as u16) / d as u16) as u8
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn scale_u8_runtime(x: u8, n: u8, d: u8) -> u8 {
    Scale(x).mul_div_trunc(n, d).unwrap()
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn raw_u8_31_255(x: u8) -> u8 {
    x * 31 / 255
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn wide_u8_31_255(x: u8) -> u8 {
    ((x as u16 * 31) / 255) as u8
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn scale_u8_31_255(x: u8) -> u8 {
    Scale(x).mul_div_round(31, 255).unwrap()
}
