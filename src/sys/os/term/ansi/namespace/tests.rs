// devela::sys::os::term::ansi::namespace::definition

use crate::{Ansi, AnsiColor8, AnsiLink};

#[test]
fn write_ansi_code_n() {
    let mut buffer = [0u8; 16];
    let result = Ansi::write_ansi_code_n(&mut buffer, 0, b'J');
    assert_eq!(result, b"\x1b[0J");
    let result = Ansi::write_ansi_code_n(&mut buffer, 5, b'm');
    assert_eq!(result, b"\x1b[5m");
    let result = Ansi::write_ansi_code_n(&mut buffer, 255, b'B');
    assert_eq!(result, b"\x1b[255B");
    let result = Ansi::write_ansi_code_n(&mut buffer, 15000, b'C');
    assert_eq!(result, b"\x1b[15000C");
}

#[test]
fn cursor_move_n() {
    let mut buffer = [0u8; 32];
    let result = Ansi::CURSOR_MOVE_N_B(&mut buffer, 0, 0);
    assert_eq!(result, b"\x1b[0;0H");
    let result = Ansi::CURSOR_MOVE_N_B(&mut buffer, 1, 2);
    assert_eq!(result, b"\x1b[2;1H");
    let result = Ansi::CURSOR_MOVE_N_B(&mut buffer, 5, 10);
    assert_eq!(result, b"\x1b[10;5H");
    let result = Ansi::CURSOR_MOVE_N_B(&mut buffer, 123, 456);
    assert_eq!(result, b"\x1b[456;123H");
    let result = Ansi::CURSOR_MOVE_N_B(&mut buffer, 1999, 10999);
    assert_eq!(result, b"\x1b[10999;1999H");
    let result = Ansi::CURSOR_MOVE(30_000, 20_000); // StringNonul
    assert_eq!(result, "\x1b[20000;30000H");
}

/* OSC */
#[test]
fn ansi_link_write_to() {
    let link = Ansi::link("docs", "https://example.com");
    let mut buf = [0u8; 64];
    let len = link.write_to(&mut buf).unwrap();
    assert_eq!(&buf[..len], b"\x1b]8;;https://example.com\x1b\\docs\x1b]8;;\x1b\\",);
    assert_eq!(len, link.len());
}
#[test]
fn ansi_link_write_to_with_id() {
    let link = AnsiLink::with_id("docs", "https://example.com", "x");
    let mut buf = [0u8; 72];
    let len = link.write_to(&mut buf).unwrap();
    assert_eq!(&buf[..len], b"\x1b]8;id=x;https://example.com\x1b\\docs\x1b]8;;\x1b\\",);
    assert_eq!(len, link.len());
}
#[test]
fn ansi_link_write_to_too_small() {
    let link = Ansi::link("docs", "https://example.com");
    let mut buf = [0u8; 8];
    assert_eq!(link.write_to(&mut buf), Err(link.len()));
}
#[test]
fn ansi_window_title_write_to() {
    let title = Ansi::title_window("devela");
    let mut buf = [0u8; 32];
    let len = title.write_to(&mut buf).unwrap();
    assert_eq!(&buf[..len], b"\x1b]2;devela\x1b\\");
}
#[test]
fn ansi_clipboard_base64_write_to() {
    let clip = Ansi::clipboard_base64("c", "aGVsbG8=");
    let mut buf = [0u8; 32];
    let len = clip.write_to(&mut buf).unwrap();
    assert_eq!(&buf[..len], b"\x1b]52;c;aGVsbG8=\x1b\\");
}
#[test]
fn underline_colors() {
    assert_eq!(Ansi::UNDERLINE_COLOR8(AnsiColor8(7)), *b"\x1b[58;5;007m",);
    assert_eq!(Ansi::UNDERLINE_RGB([1, 23, 255]), *b"\x1b[58;2;001;023;255m",);
}
