// devela::rend::draw::error
//
//!
//

/// A drawing result.
pub type DrawResult<T> = crate::Result<T, DrawError>;

/// A drawing error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DrawError {
    /// TEMP
    DrawError,
}

impl crate::Error for DrawError {}

mod core_impls {
    use crate::{Display, DrawError, FmtResult, Formatter};

    impl Display for DrawError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            write![f, "DrawError"]
        }
    }
}
