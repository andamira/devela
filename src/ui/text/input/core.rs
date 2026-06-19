// devela/src/ui/text/mod.rs

use crate::{TextInput, TextInputAction, TextInputConfig, TextInputOutcome, TextInputView};

/* generic storage */

#[rustfmt::skip]
impl<B> TextInput<B> {
    /// Creates an empty text input over `storage`.
    #[must_use]
    pub const fn from_storage(storage: B) -> Self {
        Self { storage, len: 0, cursor: 0, config: TextInputConfig::DEFAULT }
    }
    /// Creates an empty text input over `storage`, with `config`.
    #[must_use]
    pub const fn from_storage_config(storage: B, config: TextInputConfig) -> Self {
        Self { storage, len: 0, cursor: 0, config }
    }
    /// Returns the initialized length, in bytes.
    #[must_use]
    pub const fn len(&self) -> usize { self.len }
    /// Returns whether the input is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.len == 0 }

    /// Returns the cursor byte position.
    #[must_use]
    pub const fn cursor(&self) -> usize { self.cursor }
    /// Returns whether the cursor is at the beginning.
    #[must_use]
    pub const fn is_cursor_start(&self) -> bool { self.cursor == 0 }
    /// Returns whether the cursor is at the end.
    #[must_use]
    pub const fn is_cursor_end(&self) -> bool { self.cursor == self.len }

    /// Returns the input configuration.
    #[must_use]
    pub const fn config(&self) -> TextInputConfig { self.config }
    /// Sets the input configuration.
    pub const fn set_config(&mut self, config: TextInputConfig) { self.config = config; }

    /// Returns the underlying storage.
    #[must_use]
    pub const fn storage(&self) -> &B { &self.storage }
    /// Returns mutable access to the underlying storage.
    pub const fn storage_mut(&mut self) -> &mut B { &mut self.storage }
    /// Consumes the input and returns its storage.
    #[must_use]
    pub fn into_storage(self) -> B { self.storage }
}

/* owned inline bytes */

#[rustfmt::skip]
impl<const CAP: usize> TextInput<[u8; CAP]> {
    /// Creates an empty input with inline storage.
    #[must_use]
    pub const fn new() -> Self { Self::from_storage([0; CAP]) }

    /// Returns the storage capacity, in bytes.
    #[must_use]
    pub const fn capacity(&self) -> usize { CAP }
    /// Returns the remaining storage capacity, in bytes.
    #[must_use]
    pub const fn remaining_capacity(&self) -> usize { CAP - self.len }

    /// Returns the initialized bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] { &self.storage[..self.len] }
    /// Returns the initialized UTF-8 text.
    #[must_use]
    pub fn as_str(&self) -> &str { Self::valid_str(&self.storage, self.len) }

    /// Tries to set the cursor to a UTF-8 boundary.
    pub fn try_set_cursor(&mut self, cursor: usize) -> bool {
        let valid = Self::valid_cursor(self.as_str(), cursor);
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
        Self::apply_bytes(&mut self.storage, &mut self.len, &mut self.cursor, self.config, action)
    }
}
impl<const CAP: usize> Default for TextInput<[u8; CAP]> {
    fn default() -> Self {
        Self::new()
    }
}

/* borrowed bytes */

#[rustfmt::skip]
impl<'a> TextInput<&'a mut [u8]> {
    /// Creates an empty input over caller-provided storage.
    #[must_use]
    pub const fn from_buf(buf: &'a mut [u8]) -> Self { Self::from_storage(buf) }

    /// Creates an empty input over caller-provided storage, with `config`.
    #[must_use]
    pub const fn from_buf_config(buf: &'a mut [u8], config: TextInputConfig) -> Self {
        Self::from_storage_config(buf, config)
    }
    /// Returns the storage capacity, in bytes.
    #[must_use]
    pub const fn capacity(&self) -> usize { self.storage.len() }
    /// Returns the remaining storage capacity, in bytes.
    #[must_use]
    pub const fn remaining_capacity(&self) -> usize {
        self.storage.len() - self.len
    }

    /// Returns the initialized bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] { &self.storage[..self.len] }
    /// Returns the initialized UTF-8 text.
    #[must_use]
    pub fn as_str(&self) -> &str { Self::valid_str(self.storage, self.len) }

    /// Tries to set the cursor to a UTF-8 boundary.
    pub fn try_set_cursor(&mut self, cursor: usize) -> bool {
        let valid = Self::valid_cursor(self.as_str(), cursor);
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
        Self::apply_bytes(self.storage, &mut self.len, &mut self.cursor, self.config, action)
    }
}
