// devela::sys::io::impls

use crate::{ExtError, IoError, IoErrorKind};

impl ExtError for IoError {
    type Kind = IoErrorKind;

    fn error_eq(&self, other: &Self) -> bool {
        self.error_kind() == other.error_kind()
    }

    fn error_kind(&self) -> Self::Kind {
        self.kind()
    }
}
