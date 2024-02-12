// devela::num::sign
//
//! the sign of a number.
//

use crate::code::ConstDefault;

/// Represents the sign of a number.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Sign {
    /// A positive sign (+). This is the default value.
    #[default]
    Positive,
    /// A negative sign (-).
    Negative,
}

impl ConstDefault for Sign {
    const DEFAULT: Self = Sign::Positive;
}
