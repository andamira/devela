// devela::media::layout::error
//
//!
//

/// A layout-related result.
pub type LayoutResult<T> = crate::Result<T, LayoutError>;

/// A layout-related error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LayoutError {
    /// TEMP
    LayoutError,
}

mod core_impls {
    use crate::{Display, FmtResult, Formatter, LayoutError};

    impl crate::Error for LayoutError {}
    impl crate::ExtError for LayoutError {
        type Kind = ();
        fn error_eq(&self, other: &Self) -> bool {
            self == other
        }
        fn error_kind(&self) -> Self::Kind {}
    }

    impl Display for LayoutError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            write![f, "LayoutError"]
        }
    }
}
