// devela::sys::os::term::render::tests

use super::*;
use crate::{AnsiColor, AnsiColor3, NotEnoughSpace};

#[test]
fn array_storage_starts_empty() {
    let r = TermRenderer::from_buf([0u8; 16], 80, 24);

    assert_eq!(r.capacity(), 16);
    assert_eq!(r.buffered_len(), 0);
    assert_eq!(r.buffered(), b"");
    assert_eq!(r.cols(), 80);
    assert_eq!(r.rows(), 24);
    assert_eq!(r.bytes_written(), 0);
    assert_eq!(r.frames_presented(), 0);
}

#[test]
fn push_bytes_updates_active_frame() {
    let mut r = TermRenderer::from_buf([0u8; 16], 80, 24);

    r.try_push_bytes(b"abc").unwrap();
    r.try_push_str("def").unwrap();

    assert_eq!(r.buffered(), b"abcdef");
    assert_eq!(r.buffered_len(), 6);
}

#[test]
fn clear_buffer_keeps_storage_reusable() {
    let mut r = TermRenderer::from_buf([0u8; 8], 80, 24);

    r.try_push_bytes(b"abc").unwrap();
    r.clear_buffer();
    r.try_push_bytes(b"xy").unwrap();

    assert_eq!(r.buffered(), b"xy");
}

#[test]
fn push_bytes_reports_not_enough_space() {
    let mut r = TermRenderer::from_buf([0u8; 4], 80, 24);

    assert!(matches!(r.try_push_bytes(b"abcde"), Err(NotEnoughSpace(Some(5)))));

    assert_eq!(r.buffered(), b"");
}

#[test]
fn text_at0_writes_cursor_then_text() {
    let mut r = TermRenderer::from_buf([0u8; 32], 80, 24);

    r.try_text_at0(2, 3, "hi").unwrap();

    assert_eq!(r.buffered(), b"\x1b[4;3Hhi");
}

#[test]
fn colors_write_expected_ansi_sequence() {
    let mut r = TermRenderer::from_buf([0u8; 32], 80, 24);

    r.try_color_fg(AnsiColor::Bright(AnsiColor3::Cyan)).unwrap();

    assert_eq!(r.buffered(), b"\x1b[96m");
}

#[test]
fn hline_repeats_unit() {
    let mut r = TermRenderer::from_buf([0u8; 64], 80, 24);

    r.try_hline_at0(0, 0, "-", 4).unwrap();

    assert_eq!(r.buffered(), b"\x1b[1;1H----");
}

#[test]
fn replace_buf_clears_active_frame_preserves_metrics_and_size() {
    let mut r = TermRenderer::from_buf([0u8; 8], 80, 24);
    r.try_push_bytes(b"abc").unwrap();

    let r = r.replace_buf([0u8; 16]);

    assert_eq!(r.capacity(), 16);
    assert_eq!(r.buffered(), b"");
    assert_eq!(r.cols(), 80);
    assert_eq!(r.rows(), 24);
}

#[test]
fn try_replace_buf_copy_preserves_active_frame() {
    let mut r = TermRenderer::from_buf([0u8; 8], 80, 24);
    r.try_push_bytes(b"abc").unwrap();

    let r = r.try_replace_buf_copy([0u8; 16]).unwrap();

    assert_eq!(r.capacity(), 16);
    assert_eq!(r.buffered(), b"abc");
}

#[test]
fn try_replace_buf_copy_fails_if_new_storage_is_too_small() {
    let mut r = TermRenderer::from_buf([0u8; 8], 80, 24);
    r.try_push_bytes(b"abc").unwrap();

    let Err((r, new_buf)) = r.try_replace_buf_copy([0u8; 2]) else {
        panic!("expected copy replacement to fail");
    };

    assert_eq!(r.buffered(), b"abc");
    assert_eq!(new_buf.len(), 2);
}

#[cfg(feature = "alloc")]
#[test]
fn vec_capacity_is_vec_len_not_spare_capacity() {
    use crate::Vec;

    let r = TermRenderer::from_buf(Vec::<u8>::with_capacity(16), 80, 24);

    assert_eq!(r.capacity(), 0);
}

#[cfg(feature = "alloc")]
#[test]
fn with_buf_len_makes_initialized_vec_workspace() {
    let mut r = TermRenderer::with_buf_len(80, 24, 16);

    assert_eq!(r.capacity(), 16);

    r.try_push_bytes(b"abc").unwrap();
    assert_eq!(r.buffered(), b"abc");
}
