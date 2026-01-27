// devela::num::fin::bit::tests

#[cfg(feature = "std")]
use crate::{Bitwise, Panic};

#[test] #[rustfmt::skip]
#[cfg(feature = "std")]
fn bit_mask_range() {
    debug_assert![Panic::catch(|| { let _ = Bitwise::<u8>::mask_range(8, 8); }) .is_err()];
    debug_assert![Panic::catch(|| { let _ = Bitwise::<u8>::mask_range(0, 8); }) .is_err()];
    debug_assert![Panic::catch(|| { let _ = Bitwise::<u8>::mask_range(4, 1); }) .is_err()];
}
