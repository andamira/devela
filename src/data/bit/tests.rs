// devela::data::bit::tests
//
// TOC
// - bit_mask_range
// - bit_ops

#[cfg(feature = "std")]
use crate::{Bitwise, Panic, PanicAssertUnwindSafe, bitfield};

#[test] #[rustfmt::skip]
#[cfg(feature = "std")]
fn bit_mask_range() {
    debug_assert![Panic::catch(|| { let _ = Bitwise::<u8>::mask_range(8, 8); }) .is_err()];
    debug_assert![Panic::catch(|| { let _ = Bitwise::<u8>::mask_range(0, 8); }) .is_err()];
    debug_assert![Panic::catch(|| { let _ = Bitwise::<u8>::mask_range(4, 1); }) .is_err()];
}

// unchecked panics in debug mode
#[test] #[rustfmt::skip]
#[cfg(feature = "std")]
fn bitfield_bits_unchecked_panic() {
    bitfield! { (extra) struct Bf(u8) {} }
    let mut b0 = Bf::new_zeroed();

    // immutable:
    debug_assert![Panic::catch(|| { let _ = b0.set_bit(9); }).is_err()];
    debug_assert![Panic::catch(|| { let _ = b0.unset_bit(9); }).is_err()];
    debug_assert![Panic::catch(|| { let _ = b0.flip_bit(9); }).is_err()];
    // mutable:
    debug_assert![Panic::catch(PanicAssertUnwindSafe(|| { b0.mut_set_bit(9); })).is_err()];
    debug_assert![Panic::catch(PanicAssertUnwindSafe(|| { b0.mut_unset_bit(9); })).is_err()];
    debug_assert![Panic::catch(PanicAssertUnwindSafe(|| { b0.mut_flip_bit(9); })).is_err()];
}
