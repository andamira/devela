// devela::num::sign
//
//! the sign of a number.
//

/// Represents the sign of a number.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Sign {
    /// A positive sign (+).
    #[default]
    Positive,
    /// A negative sign (-).
    Negative,
}
