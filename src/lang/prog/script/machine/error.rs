// devela/src/lang/prog/script/machine/error.rs
//
//!
//

#[cfg(doc)]
use crate::ScriptMachine;
use crate::{ConstInit, Debug, Display, Error, FmtResult, Formatter, Infallible, ValueKind};

#[doc = crate::_tags!(lang error)]
/// An error encountered while executing a [`ScriptMachine`].
#[doc = crate::_doc_meta!{
    location("lang/prog/script"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(ScriptError = 16|128),
    #[cfg(target_pointer_width = "64")]
    test_size_of(ScriptError = 32|256),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScriptError<E = Infallible> {
    /// The stack contained fewer values than an operation required.
    StackUnderflow {
        /// Number of values required.
        needed: usize,
        /// Number of values available.
        available: usize,
    },
    /// An operation required more stack capacity.
    StackOverflow {
        /// Maximum number of live stack values.
        capacity: usize,
    },
    /// A value had an unexpected semantic category.
    ExpectedKind {
        /// Required value category.
        expected: ValueKind,
        /// Encountered value category.
        found: ValueKind,
    },
    /// An operation does not support this opaque value category.
    UnsupportedKind {
        /// The unsupported value category.
        found: ValueKind,
    },
    /// Signed integer arithmetic overflowed.
    IntegerOverflow,
    /// A pending host call no longer matches the machine state.
    InvalidCall,
    /// The instruction position lies beyond the program.
    InvalidIp {
        /// Current instruction position.
        ip: usize,
        /// Program length.
        len: usize,
    },
    /// A relative jump would leave the program range.
    InvalidJump {
        /// Instruction position from which the jump was attempted.
        ip: usize,
        /// Requested relative offset.
        offset: isize,
        /// Program length.
        len: usize,
    },
    /// The host rejected or failed a host operation.
    Host(E),
}

impl<E> ConstInit for ScriptError<E> {
    const INIT: Self = Self::IntegerOverflow;
}

impl<E: Display + Debug> Error for ScriptError<E> {}
impl<E: Display> Display for ScriptError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        match self {
            Self::StackUnderflow { needed, available } => {
                write!(f, "Script stack underflow: needed {needed} values, found {available}.")
            }
            Self::StackOverflow { capacity } => {
                write!(f, "Script stack overflow: capacity is {capacity} values.")
            }
            Self::ExpectedKind { expected, found } => {
                write!(f, "Unexpected script value kind: expected {expected:?}, found {found:?}.")
            }
            Self::UnsupportedKind { found } => {
                write!(f, "Unsupported script value kind: found {found:?}.")
            }
            Self::InvalidCall => f.write_str("Script call no longer matches the machine state."),
            Self::IntegerOverflow => f.write_str("Script integer arithmetic overflow."),
            Self::InvalidIp { ip, len } => {
                write!(f, "Invalid script instruction position {ip} for program length {len}.")
            }
            Self::InvalidJump { ip, offset, len } => {
                write!(f, "Invalid script jump from {ip} by {offset} for program length {len}.")
            }
            Self::Host(err) => {
                write!(f, "Script host error: {err}")
            }
        }
    }
}
impl<E> ScriptError<E> {
    pub(crate) const fn expected_kind(expected: ValueKind, found: ValueKind) -> Self {
        Self::ExpectedKind { expected, found }
    }
    pub(crate) const fn unsupported_kind(found: ValueKind) -> Self {
        Self::UnsupportedKind { found }
    }
    pub(crate) const fn invalid_ip(ip: usize, len: usize) -> Self {
        Self::InvalidIp { ip, len }
    }
    pub(crate) const fn invalid_jump(ip: usize, offset: isize, len: usize) -> Self {
        Self::InvalidJump { ip, offset, len }
    }
    pub(crate) const fn stack_overflow(capacity: usize) -> Self {
        Self::StackOverflow { capacity }
    }
    pub(crate) const fn stack_underflow(needed: usize, available: usize) -> Self {
        Self::StackUnderflow { needed, available }
    }
}
