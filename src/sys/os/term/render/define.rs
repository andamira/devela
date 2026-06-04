// devela::sys::os::term::render::define
//
//! Defines [`TermRenderer`].
//

use crate::TermSize;

#[doc = crate::_tags!(term linux)]
/// Terminal renderer over caller-provided byte-frame storage.
#[doc = crate::_doc_meta!{
    location("sys/os/term/backend"),
    test_size_of(__: TermRenderer<&mut[u8]> = 48|384),
}]
/// `TermRenderer<B>` treats `B` as initialized, contiguous byte storage through
/// `AsRef<[u8]>` and `AsMut<[u8]>`. The renderer owns the active byte length,
/// so the same type works with arrays, borrowed slices, and initialized vectors.
///
/// It builds terminal output in memory first, then [`present`][Self::present]
/// writes the active bytes and clears the active frame.
///
/// # Storage model
///
/// The storage capacity is `buf.as_ref().len()`.
///
/// For `Vec<u8>`, this means the vector **length**, not spare capacity. Use
/// [`with_buf_len`][Self::with_buf_len] or `vec![0; N]` when using allocated storage.
///
/// # Coordinates
///
/// Methods ending in `0` use zero-based coordinates.
/// Methods without `0` use terminal one-based coordinates.
///
/// # Scope
///
/// This renderer does not manage terminal modes, input events, clipping,
/// wrapping, Unicode width, dirty regions, or terminal-size probing.
///
/// # Methods
///
/// - Core
///   - from_buf
///   - into_buf
///   - buf, _mut
///   - replace_buf
///   - size
///   - set_size
///   - cols
///   - rows
///   - bytes_written
///   - frames_presented
/// - Io
///   - present
/// - Owned Vector
///   - with_buf_len
///   - replace_with_vec_len
///   - try_replace_with_vec_len_copy
/// - Shared reference
///   - capacity
///   - buffered, _len
///   - is_empty
///   - is_full
///   - try_replace_buf_copy
/// - Exclusive reference
///   - clear_buffer
///   - try_push_bytes
///   - try_push_str
///   - try_finish_frame
///   - try_clear_screen
///   - try_clear_line
///   - try_cursor_home
///   - try_cursor_hide
///   - try_cursor_show
///   - try_cursor_move_to
///   - try_cursor_move_to0
///   - try_style_reset
///   - try_style_bold
///   - try_color_fg
///   - try_color_bg
///   - try_colors
///   - try_text_at
///   - try_text_at0
///   - try_hline_at0
///   - try_vline_at0
///
/// # Examples
/// ```no_run
/// # use devela::{AnsiColor, AnsiColor3, IoResult, TermRenderer, ansi};
/// fn main() -> IoResult<()> {
///     let mut r = TermRenderer::from_buf([0u8; 4096], 80, 24);
///
///     begin_fullscreen(&mut r);
///
///     r.try_color_fg(AnsiColor::Bright(AnsiColor3::Cyan)).unwrap();
///     r.try_text_at0(2, 1, "devela terminal renderer").unwrap();
///
///     r.try_color_fg(AnsiColor::Bright(AnsiColor3::Green)).unwrap();
///     r.try_hline_at0(2, 3, "─", 32).unwrap();
///
///     finish(&mut r);
///     r.present()
/// }
///
/// fn begin_fullscreen<B>(r: &mut TermRenderer<B>)
/// where
///     B: AsRef<[u8]> + AsMut<[u8]>,
/// {
///     r.clear_buffer();
///     r.try_push_bytes(ansi![b: cursor_invisible, erase_screen, cursor_home]).unwrap();
/// }
///
/// fn finish<B>(r: &mut TermRenderer<B>)
/// where
///     B: AsRef<[u8]> + AsMut<[u8]>,
/// {
///     r.try_push_bytes(ansi![b: reset, cursor_visible]).unwrap();
/// }
/// ```
#[must_use]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TermRenderer<B> {
    pub(super) buf: B,
    pub(super) len: usize,
    pub(super) size: TermSize,
    pub(super) bytes_written: u64,
    pub(super) frames_presented: u64,
}
