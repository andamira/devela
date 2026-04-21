// devela::ui::event::key::pad
//
//! Defines [`KeyPad`].
//

use crate::ConstInit;

/* definitions */

#[doc = crate::_tags!(interaction)]
/// Keypad keys.
#[doc = crate::_doc_location!("ui/event")]
#[repr(u8)]
#[non_exhaustive]
#[allow(missing_docs)] #[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyPad {
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    Multiply, Add, Subtract, Divide, Decimal,
    Enter, Equal, Comma,
}

/* impls */

impl ConstInit for KeyPad {
    const INIT: Self = Self::Num0;
}
