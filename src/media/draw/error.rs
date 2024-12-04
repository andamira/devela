// devela::media::draw::error
//
//!
//

/// A drawing-related result.
pub type DrawResult<T> = crate::Result<T, DrawError>;

/// A drawing-related error.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DrawError {
    /// TEMP
    DrawError,
}

mod core_impls {
    use crate::{Display, DrawError, FmtResult, Formatter};

    impl crate::Error for DrawError {}
    impl crate::ExtError for DrawError {
        type Kind = ();
        fn error_eq(&self, other: &Self) -> bool {
            self == other
        }
        fn error_kind(&self) -> Self::Kind {}
    }

    impl Display for DrawError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
            write![f, "DrawError"]
        }
    }
}
