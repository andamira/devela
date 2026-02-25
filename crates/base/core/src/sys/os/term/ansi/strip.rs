// devela_base_core::sys::os::term::ansi::strip
//
//! Defines the [`Ansi::strip_codes`] method.
//

use crate::Ansi;

/// The state of an Ansi code for [`Ansi::strip_codes`].
#[derive(Clone, Copy, PartialEq)]
enum State {
    Normal, // Copying text to output
    Escape, // Saw ESC, determining sequence type
    Csi,    // In CSI sequence (ESC[)
    Osc,    // In OSC sequence (ESC])
}

impl Ansi {
    /// Removes ANSI escape sequences from a byte slice and writes the clean text to a buffer.
    ///
    /// This function processes well-formed ANSI escape sequences as defined by ECMA-48,
    /// including CSI (Control Sequence Introducer) sequences like `ESC [ ... m` and
    /// OSC (Operating System Command) sequences like `ESC ] ... BEL`.
    ///
    /// # Behavior
    ///
    /// - **Strips valid sequences**: Complete, well-formed ANSI sequences are removed.
    /// - **Preserves malformed content**: Invalid sequences or partial sequences are
    ///   passed through as-is to avoid data loss.
    /// - **Handles partial buffers**: Works correctly if buffers end mid-sequence.
    /// - **No allocations**: Uses only the provided buffers.
    ///
    /// # Supported Sequences
    ///
    /// - CSI sequences: `ESC [ P... I... F` where:
    ///   - `P...` = parameters (digits, semicolons)
    ///   - `I...` = intermediate bytes (rare)
    ///   - `F` = final byte (command: 0x40-0x7E)
    /// - OSC sequences: `ESC ] ... (BEL or ESC \)`
    /// - Single-character escapes (stripped)
    ///
    /// # Returns
    /// The number of bytes written to `dest`. This will always be ≤ the original text length.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Ansi;
    /// let input = b"Hello \x1B[32mWorld\x1B[0m!";
    /// let mut output = [0u8; 20];
    /// let len = Ansi::strip_codes(input, &mut output);
    /// assert_eq!(&output[0..len], b"Hello World!");
    /// ```
    ///
    /// # Notes
    /// - Malformed sequences (e.g., `ESC` in middle of parameters) are preserved as text.
    /// - Lone `ESC` bytes (0x1B) without valid sequence syntax are stripped.
    /// - If `dest` is too small, output is truncated (no panic).
    pub const fn strip_codes(src: &[u8], dest: &mut [u8]) -> usize {
        let mut src_pos = 0;
        let mut dest_pos = 0;
        let mut state = State::Normal;

        while src_pos < src.len() && dest_pos < dest.len() {
            let byte = src[src_pos];

            match state {
                State::Normal => {
                    if byte == 0x1B {
                        // ESC character
                        state = State::Escape;
                    } else {
                        dest[dest_pos] = byte;
                        dest_pos += 1;
                    }
                    src_pos += 1;
                }
                State::Escape => {
                    if byte == b'[' {
                        state = State::Csi;
                        src_pos += 1;
                    } else if byte == b']' {
                        state = State::Osc;
                        src_pos += 1;
                    } else if (byte >= b'0' && byte <= b'9') || byte == b';' {
                        // Parameter in progress, keep consuming
                        src_pos += 1;
                    } else {
                        // Single-character escape or invalid sequence.
                        // Strip the ESC and process current byte.
                        state = State::Normal;
                    }
                }
                State::Csi => {
                    // Consume until command character
                    if byte >= 0x40 && byte <= 0x7E {
                        // Command character
                        state = State::Normal;
                    }
                    src_pos += 1;
                }
                State::Osc => {
                    // OSC sequences end with BEL or ESC\
                    if byte == 0x07 {
                        // BEL
                        state = State::Normal;
                    } else if byte == 0x1B && src_pos + 1 < src.len() && src[src_pos + 1] == b'\\' {
                        state = State::Normal;
                        src_pos += 1; // Skip the backslash
                    }
                    src_pos += 1;
                }
            }
        }
        dest_pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_codes() {
        // Test 1: Basic sequence
        let input = b"Hello \x1B[32mWorld\x1B[0m!";
        let mut output = [0u8; 50];
        let len = Ansi::strip_codes(input, &mut output);
        assert_eq!(&output[0..len], b"Hello World!");

        // Test 2: Consecutive escape sequences
        let input = b"\x1B[1m\x1B[32mBold Green\x1B[0mText";
        let len = Ansi::strip_codes(input, &mut output);
        assert_eq!(&output[0..len], b"Bold GreenText");

        // Test 3: CSI sequences with parameters
        let input = b"Normal\x1B[1;33;44mText\x1B[0mEnd";
        let len = Ansi::strip_codes(input, &mut output);
        assert_eq!(&output[0..len], b"NormalTextEnd");

        // Test 4: OSC sequences (less common but important)
        let input = b"Start\x1B]0;Window Title\x07End";
        let len = Ansi::strip_codes(input, &mut output);
        assert_eq!(&output[0..len], b"StartEnd");

        // Test 5: Incomplete escape sequence at end
        let input = b"Text\x1B[";
        let len = Ansi::strip_codes(input, &mut output);
        assert_eq!(&output[0..len], b"Text");

        // Test 6: Real ESC character (0x1B) in normal text (should be stripped)
        let input = b"Has\x1B ESC char";
        let len = Ansi::strip_codes(input, &mut output);
        assert_eq!(&output[0..len], b"Has ESC char");

        // Test 7: Empty input
        let input = b"";
        let len = Ansi::strip_codes(input, &mut output);
        assert_eq!(&output[0..len], b"");

        // Test 8: Only escape sequences
        let input = b"\x1B[1m\x1B[32m\x1B[0m";
        let len = Ansi::strip_codes(input, &mut output);
        assert_eq!(&output[0..len], b"");

        // Test 9: Mixed content with various sequence types
        let input = b"Start\x1B[32mGreen\x1B]0;Title\x07\x1B[0mEnd";
        let len = Ansi::strip_codes(input, &mut output);
        assert_eq!(&output[0..len], b"StartGreenEnd");

        // Test 10: Buffer too small for output
        let input = b"Hello World";
        let mut small_output = [0u8; 5];
        let len = Ansi::strip_codes(input, &mut small_output);
        assert_eq!(&small_output[0..len], b"Hello");

        // Test 11: Escape sequences that look like text
        let input = b"Text\x1B[0;1;mMore[Text]";
        let len = Ansi::strip_codes(input, &mut output);
        assert_eq!(&output[0..len], b"TextMore[Text]");

        // Test 12: Binary data with ESC characters
        let input = &[b'A', 0x1B, b'[', b'1', b'm', b'B', 0x00, b'C'];
        let len = Ansi::strip_codes(input, &mut output);
        assert_eq!(&output[0..len], &[b'A', b'B', 0x00, b'C']);

        // Test 13: Multiple ESC characters in a row
        let input = b"\x1B\x1B\x1B[32mText";
        let len = Ansi::strip_codes(input, &mut output);
        assert_eq!(&output[0..len], b"Text");
    }
}
