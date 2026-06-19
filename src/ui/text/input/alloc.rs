// devela/src/ui/text/alloc.rs

use crate::String;
use crate::{
    TextInput, TextInputAction, TextInputConfig, TextInputOutcome, TextInputReject, TextInputView,
};
use TextInputOutcome::{Accepted, Cancelled, Changed, Rejected, Unchanged};

#[rustfmt::skip]
impl TextInput<String> {
    /// Creates an empty allocated input.
    #[must_use]
    pub fn new_alloc() -> Self { Self::from_storage(String::new()) }

    /// Creates an input from an allocated string.
    #[must_use]
    pub fn from_string(storage: String) -> Self {
        let len = storage.len();
        Self { storage, len, cursor: len, config: TextInputConfig::DEFAULT }
    }

    /// Returns the initialized bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] { self.storage.as_bytes() }
    /// Returns the initialized UTF-8 text.
    #[must_use]
    pub fn as_str(&self) -> &str { self.storage.as_str() }

    /// Tries to set the cursor to a UTF-8 boundary.
    pub fn try_set_cursor(&mut self, cursor: usize) -> bool {
        let valid = Self::valid_cursor(self.storage.as_str(), cursor);
        if valid { self.cursor = cursor; }
        valid
    }
    /// Returns a borrowed input view.
    #[must_use]
    pub fn view(&self) -> TextInputView<'_> {
        TextInputView { text: self.as_str(), cursor: self.cursor }
    }

    /// Applies an editing action.
    pub fn apply(&mut self, action: TextInputAction) -> TextInputOutcome {
        match action {
            TextInputAction::Clear | TextInputAction::Cancel => {}
            _ => {
                if !Self::valid_cursor(self.storage.as_str(), self.cursor) {
                    return Rejected(TextInputReject::InvalidCursor);
                }
            }
        }
        match action {
            TextInputAction::Insert(ch) => {
                let need = ch.len_utf8();
                if Self::exceeds_limit(self.len, need, usize::MAX, self.config) {
                    return Rejected(TextInputReject::Full);
                }
                if !self.storage.is_char_boundary(self.cursor) {
                    return Rejected(TextInputReject::InvalidCursor);
                }
                self.storage.insert(self.cursor, ch);
                self.len += need;
                self.cursor += need;
                Changed
            }
            TextInputAction::Backspace => {
                if self.cursor == 0 { return Unchanged; }
                let prev = Self::prev_char_boundary(self.storage.as_str(), self.cursor);
                self.storage.replace_range(prev..self.cursor, "");
                self.len -= self.cursor - prev;
                self.cursor = prev;
                Changed
            }
            TextInputAction::Delete => {
                if self.cursor == self.len { return Unchanged; }
                let next = Self::next_char_boundary(self.storage.as_str(), self.cursor);
                self.storage.replace_range(self.cursor..next, "");
                self.len -= next - self.cursor;
                Changed
            }
            TextInputAction::MoveLeft => {
                let prev = Self::prev_char_boundary(self.storage.as_str(), self.cursor);
                if prev == self.cursor { Unchanged } else { self.cursor = prev; Changed }
            }
            TextInputAction::MoveRight => {
                let next = Self::next_char_boundary(self.storage.as_str(), self.cursor);
                if next == self.cursor { Unchanged } else { self.cursor = next; Changed }
            }
            TextInputAction::MoveStart => {
                if self.cursor == 0 { Unchanged } else { self.cursor = 0; Changed }
            }
            TextInputAction::MoveEnd => {
                if self.cursor == self.len { Unchanged } else { self.cursor = self.len; Changed }
            }
            TextInputAction::Clear => {
                if self.len == 0 { Unchanged }
                else {
                    self.storage.clear();
                    self.len = 0;
                    self.cursor = 0;
                    Changed
                }
            }
            TextInputAction::Accept => {
                if self.len == 0 && !self.config.can_be_empty { Rejected(TextInputReject::Empty) }
                else { Accepted }
            }
            TextInputAction::Cancel => Cancelled,
        }
    }
}
