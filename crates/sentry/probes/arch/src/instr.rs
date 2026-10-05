//

use devela::Arch;

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn probe_nop() {
    Arch::nop();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn probe_relax() {
    Arch::relax();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn probe_nop_loop(mut n: usize) {
    while n != 0 {
        Arch::nop();
        n -= 1;
    }
}
