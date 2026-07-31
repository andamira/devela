// devela/src/text/parse/scanner/_helper.rs

use crate::{TextRange, TextScanner};

impl TextScanner<'_> {
    pub(crate) fn str_at(&self, range: TextRange) -> &str {
        self.slice_str(range).unwrap()
    }
}
