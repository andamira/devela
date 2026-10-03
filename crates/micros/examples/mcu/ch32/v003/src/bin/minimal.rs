//
//! Minimal example for the CH32.
//

#![no_std]
#![no_main]

use devela_micros::devela::{PanicInfo, global_asm};

global_asm!(
    r#"
    .section .init, "ax"
    .global _start
_start:
    la sp, _stack_top
    j main
"#
);

static mut RESULT: u32 = 0;

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    let result = mix6(1, 2, 3, 4, 5, 6);
    unsafe {
        core::ptr::write_volatile(&raw mut RESULT, result);
    }
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn mix6(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32) -> u32 {
    let x = a.wrapping_add(b);
    let y = c.wrapping_add(c).wrapping_add(c);
    let z = d.rotate_left(5);
    x ^ y ^ z ^ e.wrapping_add(f)
}
