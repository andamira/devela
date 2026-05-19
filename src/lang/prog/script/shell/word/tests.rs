// devela::lang::prog::script::shell::tests

use crate::{ShellLex, ShellQuote, ShellWordError};

fn assert_split(input: &[u8], expected: &[&[u8]]) {
    let mut lex = ShellLex::from_bytes(input);
    let mut buf = [0u8; 128];

    for &word in expected {
        let len = lex.next_word_to(&mut buf).unwrap().unwrap();
        assert_eq!(&buf[..len], word);
    }

    assert_eq!(lex.next_word_to(&mut buf).unwrap(), None);
}

fn assert_lex_err(input: &[u8], expected: ShellWordError) {
    let mut lex = ShellLex::from_bytes(input);
    let mut buf = [0u8; 128];

    assert_eq!(lex.next_word_to(&mut buf), Err(expected));
}

fn assert_quote(input: &[u8], expected: &[u8]) {
    let quote = ShellQuote::new();
    let mut out = [0u8; 128];

    let len = quote.quote_to(input, &mut out).unwrap();

    assert_eq!(quote.quoted_len(input).unwrap(), expected.len());
    assert_eq!(len, expected.len());
    assert_eq!(&out[..len], expected);
}

fn assert_roundtrip(input: &[u8]) {
    let quote = ShellQuote::new();
    let mut quoted = [0u8; 128];

    let quoted_len = quote.quote_to(input, &mut quoted).unwrap();

    let mut lex = ShellLex::from_bytes(&quoted[..quoted_len]);
    let mut decoded = [0u8; 128];

    let decoded_len = lex.next_word_to(&mut decoded).unwrap().unwrap();

    assert_eq!(&decoded[..decoded_len], input);
    assert_eq!(lex.next_word_to(&mut decoded).unwrap(), None);
}

#[test]
fn lex_splits_shell_words() {
    assert_split(b"foo$baz", &[b"foo$baz".as_slice()]);
    assert_split(b"foo baz", &[b"foo".as_slice(), b"baz".as_slice()]);
    assert_split(b"foo\"bar\"baz", &[b"foobarbaz".as_slice()]);
    assert_split(b"foo \"bar\"baz", &[b"foo".as_slice(), b"barbaz".as_slice()]);
    assert_split(b"   foo \nbar", &[b"foo".as_slice(), b"bar".as_slice()]);
    assert_split(b"foo\\\nbar", &[b"foobar".as_slice()]);
    assert_split(b"\"foo\\\nbar\"", &[b"foobar".as_slice()]);
    assert_split(b"'baz\\$b'", &[b"baz\\$b".as_slice()]);
    assert_split(b"foo #bar\nbaz", &[b"foo".as_slice(), b"baz".as_slice()]);
    assert_split(b"foo #bar", &[b"foo".as_slice()]);
    assert_split(b"foo#bar", &[b"foo#bar".as_slice()]);
    assert_split(b"'\\n'", &[b"\\n".as_slice()]);
    assert_split(b"'\\\\n'", &[b"\\\\n".as_slice()]);
}

#[test]
fn lex_handles_double_quote_escapes() {
    assert_split(b"\"a\\$b \\` \\\" \\\\ \\q\"", &[b"a$b ` \" \\ \\q".as_slice()]);
}

#[test]
fn lex_reports_errors() {
    assert_lex_err(b"\\", ShellWordError::TrailingEscape);
    assert_lex_err(b"\"\\", ShellWordError::UnterminatedDoubleQuote);
    assert_lex_err(b"'\\", ShellWordError::UnterminatedSingleQuote);
    assert_lex_err(b"\"", ShellWordError::UnterminatedDoubleQuote);
    assert_lex_err(b"'", ShellWordError::UnterminatedSingleQuote);
    assert_lex_err(b"foo\"#bar", ShellWordError::UnterminatedDoubleQuote);
}

#[test]
fn lex_reports_output_too_small() {
    let mut lex = ShellLex::from_bytes(b"abcd");
    let mut buf = [0u8; 3];

    assert_eq!(lex.next_word_to(&mut buf), Err(ShellWordError::OutputTooSmall { needed: 4 }),);
}

#[test]
fn lex_tracks_line_numbers() {
    let mut lex = ShellLex::from_bytes(b"\nfoo\nbar");
    let mut buf = [0u8; 16];

    let len = lex.next_word_to(&mut buf).unwrap().unwrap();
    assert_eq!(&buf[..len], b"foo");
    assert_eq!(lex.line_no(), 2);

    let len = lex.next_word_to(&mut buf).unwrap().unwrap();
    assert_eq!(&buf[..len], b"bar");
    assert_eq!(lex.line_no(), 3);
}

#[test]
fn quote_writes_shell_words() {
    assert_quote(b"", b"''");
    assert_quote(b"abcXYZ_09+./:@-]", b"abcXYZ_09+./:@-]");
    assert_quote(b"a b", b"'a b'");
    assert_quote(b"$HOME", b"'$HOME'");
    assert_quote(b"*.rs", b"'*.rs'");
    assert_quote(b"a'b", b"'a'\\''b'");
    assert_quote(b"a\nb", b"'a\nb'");
    assert_quote(&[b'a', 0x80, b'b'], &[b'\'', b'a', 0x80, b'b', b'\'']);
}

#[test]
fn quote_reports_errors() {
    let mut out = [0u8; 128];

    assert_eq!(ShellQuote::new().quote_to(b"a\0b", &mut out), Err(ShellWordError::Nul),);

    assert_eq!(
        ShellQuote::new().reject_control(true).quote_to(b"a\nb", &mut out),
        Err(ShellWordError::Control { byte: b'\n' }),
    );

    assert_eq!(
        ShellQuote::new().quote_to(b"a b", &mut [0u8; 4]),
        Err(ShellWordError::OutputTooSmall { needed: 5 }),
    );
}

#[test]
fn quote_roundtrips_through_lex() {
    assert_roundtrip(b"");
    assert_roundtrip(b"abc");
    assert_roundtrip(b"a b");
    assert_roundtrip(b"a'b");
    assert_roundtrip(b"$HOME");
    assert_roundtrip(b"*.rs");
    assert_roundtrip(b"a\nb");
    assert_roundtrip(&[b'a', 0x80, b'b']);
}
