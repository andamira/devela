// devela::rend::layout::error
//
//!
//

/// A layout rendering result.
pub type LayoutResult<T> = crate::Result<T, LayoutError>;

/// A layout rendering error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LayoutError {
    /// TEMP
    LayoutError,
}

impl crate::Error for LayoutError {}

mod core_impls {
    use crate::{Display, FmtResult, Formatter, LayoutError};

    impl Display for LayoutError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            write![f, "LayoutError"]
        }
    }
}
