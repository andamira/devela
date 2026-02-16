// devela::data::codec::bit::tests

#[cfg(feature = "std")]
use crate::{Panic, PanicAssertUnwindSafe, bitfield};

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
