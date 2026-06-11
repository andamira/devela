// devela::text::parse::tests
//
// TOC
// - construction, views, and cursor state
// - byte inspection and exact consumption
// - ASCII scanning and range-taking
// - quoted strings
// - predicate-driven adapters
// - UTF-8 scanning

use crate::{AsciiSet, TextParseErrorKind, TextRange, TextScanner, assert_matches, charu};

fn str_at<'a>(scanner: &TextScanner<'a>, range: TextRange) -> &'a str {
    scanner.slice_str(range).unwrap()
}

/* construction, views, and cursor state */

#[test]
fn construction_views_and_state() {
    let mut s = TextScanner::new("abc");
    assert_eq!(s.pos().as_usize(), 0);
    assert!(!s.is_eof());
    assert_eq!(s.rest(), b"abc");

    let start = s.mark();
    assert_eq!(s.next_byte(), Some(b'a'));
    assert_eq!(s.next_byte(), Some(b'b'));
    let range = s.range_from(start);

    assert_eq!(s.slice(range), b"ab");
    assert_eq!(s.slice_str(range), Some("ab"));
    assert_eq!(s.rest(), b"c");

    assert_eq!(s.next_byte(), Some(b'c'));
    assert_eq!(s.next_byte(), None);
    assert!(s.is_eof());
    assert_eq!(s.rest(), b"");
}

/* byte inspection and exact consumption */

#[test]
fn byte_ops() {
    let mut s = TextScanner::new("abc\r\ndef");

    assert_eq!(s.peek_byte(), Some(b'a'));
    assert_eq!(s.eat_byte(b'x'), false);
    assert_eq!(s.pos().as_usize(), 0);

    assert_eq!(s.eat_byte(b'a'), true);
    assert_eq!(s.pos().as_usize(), 1);

    assert_eq!(s.eat_bytes(b"bc"), true);
    assert_eq!(s.pos().as_usize(), 3);

    assert_eq!(s.eat_eol(), true);
    assert_eq!(s.pos().as_usize(), 5);

    assert_eq!(s.expect_bytes(b"def"), Ok(()));
    assert!(s.is_eof());
}

#[test]
fn scanner_byte_consumption_and_position() {
    let mut s = TextScanner::new("abc");

    assert_eq!(s.pos().as_usize(), 0);
    assert_eq!(s.peek_byte(), Some(b'a'));

    assert!(s.skip_byte());
    assert_eq!(s.pos().as_usize(), 1);
    assert_eq!(s.peek_byte(), Some(b'b'));

    assert_eq!(s.next_byte(), Some(b'b'));
    assert_eq!(s.pos().as_usize(), 2);

    assert!(s.eat_byte(b'c'));
    assert!(s.is_eof());

    assert!(!s.skip_byte());
    assert_eq!(s.next_byte(), None);
    assert_eq!(s.pos().as_usize(), 3);
}

#[test]
fn expect_ops_do_not_advance_on_failure() {
    let mut s = TextScanner::new("abc");

    assert!(s.expect_byte(b'x').is_err());
    assert_eq!(s.pos().as_usize(), 0);

    assert!(s.expect_bytes(b"ax").is_err());
    assert_eq!(s.pos().as_usize(), 0);

    assert!(s.expect_bytes(b"abcd").is_err());
    assert_eq!(s.pos().as_usize(), 0);
}

#[test]
fn take_until_variants() {
    let mut s = TextScanner::new("abc:def,ghi\r\nrest");

    let r = s.take_until_byte(b':');
    assert_eq!(s.slice_str(r), Some("abc"));
    assert_eq!(s.peek_byte(), Some(b':'));
    let _ = s.eat_byte(b':');

    let r = s.take_until_any2(b',', b';');
    assert_eq!(s.slice_str(r), Some("def"));
    assert_eq!(s.peek_byte(), Some(b','));
    let _ = s.eat_byte(b',');

    let r = s.take_until_any3(b'X', b'Y', b'\r');
    assert_eq!(s.slice_str(r), Some("ghi"));
    assert_eq!(s.peek_byte(), Some(b'\r'));

    let r = s.take_until_eol();
    assert_eq!(s.slice_str(r), Some(""));
    assert_eq!(s.peek_byte(), Some(b'\r'));

    assert_eq!(s.eat_eol(), true);
    assert_eq!(s.rest(), b"rest");
}

#[test]
fn take_until_bytes_regression_nonzero_cursor() {
    let mut s = TextScanner::new("xx--yy--zz");

    assert_eq!(s.expect_bytes(b"xx"), Ok(()));
    assert_eq!(s.pos().as_usize(), 2);

    let r = s.take_until_bytes(b"--");
    assert_eq!(s.slice_str(r), Some(""));
    assert_eq!(s.pos().as_usize(), 2);

    assert_eq!(s.eat_bytes(b"--"), true);
    assert_eq!(s.pos().as_usize(), 4);

    let r = s.take_until_bytes(b"--");
    assert_eq!(s.slice_str(r), Some("yy"));
    assert_eq!(s.pos().as_usize(), 6);

    assert_eq!(s.eat_bytes(b"--"), true);
    assert_eq!(s.pos().as_usize(), 8);

    let r = s.take_until_bytes(b"??");
    assert_eq!(s.slice_str(r), Some("zz"));
    assert_eq!(s.pos().as_usize(), 10);
    assert!(s.is_eof());
}

/* ASCII scanning and range-taking */

#[test]
fn ascii_scanners() {
    let mut s = TextScanner::new(" \t\r\nfoo_12 123bar");
    s.skip_ascii_ws();
    assert_eq!(s.pos().as_usize(), 4);

    let ident = s.take_ascii_ident().unwrap();
    assert_eq!(s.slice_str(ident), Some("foo_12"));
    assert_eq!(s.peek_byte(), Some(b' '));

    s.skip_ascii_ws();
    assert_eq!(s.expect_ascii_u64(), Ok(123));
    assert_eq!(s.rest(), b"bar");
}

#[test]
fn ascii_u64_edge_cases() {
    let mut s = TextScanner::new("abc");
    assert_eq!(s.take_ascii_u64(), Ok(None));
    assert!(s.expect_ascii_u64().is_err());
    assert_eq!(s.pos().as_usize(), 0);

    let mut s = TextScanner::new("18446744073709551615!");
    assert_eq!(s.take_ascii_u64(), Ok(Some(u64::MAX)));
    assert_eq!(s.peek_byte(), Some(b'!'));

    let mut s = TextScanner::new("18446744073709551616!");
    assert!(s.take_ascii_u64().is_err());
    assert_eq!(s.pos().as_usize(), 19); // stops before the offending digit
    assert_eq!(s.peek_byte(), Some(b'6'));
}

#[test]
fn scanner_ascii_set_consumes_runs_and_delimiters() {
    let mut s = TextScanner::from_bytes(b"abc=123;\xC3\xB1!");

    let alpha = s.take_ascii_set(AsciiSet::ALPHA).unwrap();
    assert_eq!(str_at(&s, alpha), "abc");

    assert!(s.eat_ascii_set(AsciiSet::PUNCT));
    assert_eq!(s.peek_byte(), Some(b'1'));

    let digits = s.take_ascii_set(AsciiSet::DIGIT).unwrap();
    assert_eq!(s.slice(digits), b"123");

    assert_eq!(s.skip_until_ascii_set(AsciiSet::PUNCT), 0);
    assert!(s.eat_ascii_set(AsciiSet::PUNCT));

    // Non-ASCII bytes do not match the ASCII set, but are still skipped over.
    assert_eq!(s.skip_until_ascii_set(AsciiSet::PUNCT), 2);
    assert_eq!(s.peek_byte(), Some(b'!'));
    assert!(s.eat_ascii_set(AsciiSet::PUNCT));
    assert!(s.is_eof());
}

#[test]
fn scanner_ascii_set_takes_until_and_runs() {
    let mut s = TextScanner::new("name:value rest");

    let head = s.take_until_ascii_set(AsciiSet::PUNCT);
    assert_eq!(str_at(&s, head), "name");
    assert_eq!(s.peek_byte(), Some(b':'));

    assert!(s.eat_ascii_set(AsciiSet::PUNCT));

    let tail = s.take_until_ascii_set(AsciiSet::PUNCT);
    assert_eq!(str_at(&s, tail), "value rest");
    assert!(s.is_eof());

    let mut id = TextScanner::new("9bad _good");
    assert!(id.take_ascii_run(AsciiSet::IDENT_HEAD, AsciiSet::IDENT_TAIL).is_none());
    assert_eq!(id.pos().as_usize(), 0);

    let tail = id.take_ascii_set(AsciiSet::IDENT_TAIL).unwrap();
    assert_eq!(str_at(&id, tail), "9bad");

    id.skip_ascii_hws();

    let ident = id.take_ascii_run(AsciiSet::IDENT_HEAD, AsciiSet::IDENT_TAIL).unwrap();
    assert_eq!(str_at(&id, ident), "_good");
    assert!(id.is_eof());
}

/* quoted strings */

#[test]
fn quoted_scanning_and_decoding() {
    let mut s = TextScanner::new(r#""a\n\"b" 'x\y'"#);

    let basic = s.take_quoted_basic().unwrap().unwrap();
    assert_eq!(s.slice(basic), br#"a\n\"b"#);

    let mut out = [0u8; 16];
    let len = s.decode_quoted_basic_into(basic, &mut out).unwrap();
    assert_eq!(&out[..len], b"a\n\"b");

    let text = s.decode_quoted_basic_str_into(basic, &mut out).unwrap();
    assert_eq!(text, "a\n\"b");

    s.skip_ascii_ws();

    let lit = s.take_quoted_literal().unwrap().unwrap();
    assert_eq!(s.slice(lit), br#"x\y"#);
}

#[test]
fn quoted_decode_errors() {
    // trailing backslash
    let mut s = TextScanner::from_bytes(b"a\\");
    let start = s.mark();
    let _ = s.next_byte();
    let _ = s.next_byte();
    let range = s.range_from(start);

    let mut out = [0u8; 8];
    assert!(s.decode_quoted_basic_into(range, &mut out).is_err());

    // invalid escape
    let mut s = TextScanner::from_bytes(b"\\x");
    let start = s.mark();
    let _ = s.next_byte();
    let _ = s.next_byte();
    let range = s.range_from(start);

    let mut out = [0u8; 8];
    assert!(s.decode_quoted_basic_into(range, &mut out).is_err());
}

#[test]
fn quoted_decode_invalid_utf8() {
    let mut s = TextScanner::from_bytes(&[0xFF]);
    let start = s.mark();
    let _ = s.next_byte();
    let range = s.range_from(start);

    let mut out = [0u8; 8];
    let err = s.decode_quoted_basic_str_into(range, &mut out).unwrap_err();

    assert_eq!(err.at.index.as_usize(), 0);
    match err.kind {
        TextParseErrorKind::InvalidUtf8(e) => {
            assert_eq!(e.valid_up_to, 0);
            assert_eq!(e.error_len, Some(1));
        }
        other => panic!("unexpected error kind: {other:?}"),
    }
}

#[test]
fn quoted_decode_invalid_escape() {
    let mut s = TextScanner::from_bytes(b"\\x");
    let start = s.mark();
    let _ = s.next_byte();
    let _ = s.next_byte();
    let range = s.range_from(start);

    let mut out = [0u8; 8];
    let err = s.decode_quoted_basic_into(range, &mut out).unwrap_err();

    assert_eq!(err.at.index.as_usize(), 1);
    assert_matches!(err.kind, TextParseErrorKind::InvalidEscape);
}

#[test]
fn quoted_decode_trailing_backslash() {
    let mut s = TextScanner::from_bytes(b"a\\");
    let start = s.mark();
    let _ = s.next_byte();
    let _ = s.next_byte();
    let range = s.range_from(start);

    let mut out = [0u8; 8];
    let err = s.decode_quoted_basic_into(range, &mut out).unwrap_err();

    assert_eq!(err.at.index.as_usize(), 2);
    assert_matches!(err.kind, TextParseErrorKind::UnterminatedQuote);
}

#[test]
fn quoted_decode_buffer_too_small() {
    let mut s = TextScanner::new(r#""abcd""#);
    let range = s.take_quoted_basic().unwrap().unwrap();

    let mut out = [0u8; 2];
    assert!(s.decode_quoted_basic_into(range, &mut out).is_err());
}

#[test]
fn scanner_line_and_quoted_helpers() {
    let mut s = TextScanner::new("  alpha # comment\r\n\"a\\n b\" tail\nplain tail");

    let line = s.next_line_trimmed_before(b'#').unwrap();
    assert_eq!(str_at(&s, line), "alpha");

    let quoted = s.take_quoted_basic_or_rest().unwrap();
    assert_eq!(str_at(&s, quoted), "a\\n b");

    s.skip_ascii_hws();

    let tail = s.next_line_trimmed().unwrap();
    assert_eq!(str_at(&s, tail), "tail");

    let rest = s.take_quoted_basic_or_rest().unwrap();
    assert_eq!(str_at(&s, rest), "plain tail");
    assert!(s.is_eof());

    // Pin the documented byte-blind behavior.
    let mut comments = TextScanner::new(r#"  "x # y" # z"#);
    let before_hash = comments.next_line_trimmed_before(b'#').unwrap();
    assert_eq!(str_at(&comments, before_hash), r#""x"#);
}

/* predicate-driven adapters */

#[test]
fn predicate_adapters() {
    let mut s = TextScanner::new("123abc456");

    let n = s.skip_while(|b| b.is_ascii_digit());
    assert_eq!(n, 3);
    assert_eq!(s.rest(), b"abc456");

    let r = s.take_while(|b| b.is_ascii_alphabetic());
    assert_eq!(s.slice_str(r), Some("abc"));
    assert_eq!(s.rest(), b"456");

    let r = s.take_while(|b| b.is_ascii_digit());
    assert_eq!(s.slice_str(r), Some("456"));
    assert!(s.is_eof());
}

/* Unicode scalar scanning */

#[test]
fn scanner_utf8_char_peek_next_take_and_eat() {
    let mut s = TextScanner::new("aé€🐛z");

    assert_eq!(s.pos().as_usize(), 0);
    assert_eq!(s.peek_char(), Some('a'));
    assert_eq!(s.peek_char(), Some('a'));
    assert_eq!(s.next_char(), Some('a'));
    assert_eq!(s.pos().as_usize(), 1);

    let r = s.take_char().unwrap();
    assert_eq!(str_at(&s, r), "é");
    assert_eq!(s.pos().as_usize(), 3);

    assert!(s.eat_char('€'));
    assert_eq!(s.pos().as_usize(), 6);

    let r = s.take_char().unwrap();
    assert_eq!(str_at(&s, r), "🐛");
    assert_eq!(s.pos().as_usize(), 10);

    assert_eq!(s.next_char(), Some('z'));
    assert_eq!(s.next_char(), None);
    assert!(s.is_eof());
}

#[test]
fn scanner_utf8_char_predicates() {
    let mut s = TextScanner::new("αβγ123");

    let r = s.take_char_if(|ch| ch == 'α').unwrap();
    assert_eq!(str_at(&s, r), "α");

    assert!(s.take_char_if(|ch| ch.is_ascii_digit()).is_none());
    assert_eq!(s.peek_char(), Some('β'));

    let letters = s.take_char_while(|ch| ch.is_alphabetic());
    assert_eq!(str_at(&s, letters), "βγ");

    assert_eq!(s.skip_char_while(|ch| ch.is_ascii_digit()), 3);
    assert!(s.is_eof());
}

#[test]
fn scanner_utf8_invalid_input_does_not_advance() {
    let mut s = TextScanner::from_bytes(b"a\xFFz");

    assert_eq!(s.next_char(), Some('a'));
    assert_eq!(s.pos().as_usize(), 1);

    assert_eq!(s.peek_char(), None);
    assert_eq!(s.next_char(), None);
    assert!(s.take_char().is_none());
    assert!(!s.eat_char('z'));
    assert!(s.take_char_if(|_| true).is_none());

    // Strict UTF-8 scalar methods stop before the invalid byte.
    assert_eq!(s.pos().as_usize(), 1);

    // Byte-level methods remain available for recovery.
    assert_eq!(s.next_byte(), Some(0xFF));
    assert_eq!(s.next_char(), Some('z'));
    assert!(s.is_eof());
}

/* Unicode scalar scanning, with UTF-8 representation */

#[test]
fn scanner_utf8_charu_peek_next_and_eat() {
    let mut s = TextScanner::new("aé€🐛z");

    assert_eq!(s.peek_charu(), Some(charu::from_char('a')));
    assert_eq!(s.peek_charu(), Some(charu::from_char('a')));

    assert_eq!(s.next_charu(), Some(charu::from_char('a')));
    assert_eq!(s.pos().as_usize(), 1);

    assert!(s.eat_charu(charu::from_char('é')));
    assert_eq!(s.pos().as_usize(), 3);

    assert!(s.eat_charu(charu::from_char('€')));
    assert_eq!(s.pos().as_usize(), 6);

    assert_eq!(s.next_charu(), Some(charu::from_char('🐛')));
    assert_eq!(s.pos().as_usize(), 10);

    assert!(!s.eat_charu(charu::from_char('x')));
    assert_eq!(s.next_charu(), Some(charu::from_char('z')));
    assert_eq!(s.next_charu(), None);
    assert!(s.is_eof());
}

#[test]
fn scanner_utf8_charu_predicates() {
    let mut s = TextScanner::new("αβγ123");

    let r = s.take_charu_if(|ch| ch == charu::from_char('α')).unwrap();
    assert_eq!(str_at(&s, r), "α");

    assert!(s.take_charu_if(|ch| ch == charu::from_char('1')).is_none());
    assert_eq!(s.peek_charu(), Some(charu::from_char('β')));

    let letters = s.take_charu_while(|ch| !ch.to_char().is_ascii_digit());
    assert_eq!(str_at(&s, letters), "βγ");

    assert_eq!(s.skip_charu_while(|ch| ch.to_char().is_ascii_digit()), 3);
    assert!(s.is_eof());
}

#[test]
fn scanner_utf8_charu_invalid_input_does_not_advance() {
    let mut s = TextScanner::from_bytes(b"a\xFFz");

    assert_eq!(s.next_charu(), Some(charu::from_char('a')));
    assert_eq!(s.pos().as_usize(), 1);

    assert_eq!(s.peek_charu(), None);
    assert_eq!(s.next_charu(), None);
    assert!(s.take_charu_if(|_| true).is_none());
    assert_eq!(s.skip_charu_while(|_| true), 0);

    // Strict UTF-8 scalar methods stop before the invalid byte.
    assert_eq!(s.pos().as_usize(), 1);

    // Byte-level recovery remains possible.
    assert_eq!(s.next_byte(), Some(0xFF));
    assert_eq!(s.next_charu(), Some(charu::from_char('z')));
    assert!(s.is_eof());
}
