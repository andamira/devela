// devela/src/data/access/cursor/byte/_test.rs

use crate::{ByteCursor, NotEnoughSpace, UnexpectedEof};

/* read */

#[test]
fn read_fixed_width() {
    let bytes = b"RIFF\x24\x00\x00\x00WAVE";
    let mut cur = ByteCursor::reader(bytes);
    assert_eq!(cur.take_4(), Some(*b"RIFF"));
    assert_eq!(cur.take_u32_le(), Some(36));
    assert_eq!(cur.take_4(), Some(*b"WAVE"));
    assert_eq!(cur.pos(), 12);
    assert!(cur.is_eof());
}
#[test]
fn read_slice_payload() {
    let bytes = b"abcdPAYLOADxyz";
    let mut cur = ByteCursor::reader(bytes);
    assert_eq!(cur.take_4(), Some(*b"abcd"));
    assert_eq!(cur.take(7), Some(&b"PAYLOAD"[..]));
    assert_eq!(cur.rest(), b"xyz");
}
#[test]
fn read_bounds() {
    let bytes = [1, 2, 3];
    let mut cur = ByteCursor::reader(&bytes);
    assert_eq!(cur.take_4(), None);
    assert_eq!(cur.pos(), 0);
    assert_eq!(cur.skip_exact(4), Err(UnexpectedEof(Some(4))));
    assert_eq!(cur.pos(), 0);
    assert_eq!(cur.advance(99), 3);
    assert!(cur.is_eof());
}
#[test]
fn generic_array_read_is_available() {
    let bytes = [1, 2, 3, 4, 5];
    let mut cur = ByteCursor::reader(&bytes);
    assert_eq!(cur.take_array::<3>(), Some([1, 2, 3]));
    assert_eq!(cur.take_array::<2>(), Some([4, 5]));
    assert_eq!(cur.take_array::<1>(), None);
}

/* write */

#[test]
fn write_fixed_width() {
    let mut bytes = [0u8; 12];
    let mut cur = ByteCursor::writer(&mut bytes);
    assert_eq!(cur.write_4(*b"RIFF"), Ok(()));
    assert_eq!(cur.write_u32_le(36), Ok(()));
    assert_eq!(cur.write_4(*b"WAVE"), Ok(()));
    assert_eq!(cur.pos(), 12);
    assert_eq!(&bytes, b"RIFF\x24\x00\x00\x00WAVE");
}
#[test]
fn write_patch_size() {
    let mut bytes = [0u8; 12];
    let mut cur = ByteCursor::writer(&mut bytes);
    assert_eq!(cur.write_4(*b"RIFF"), Ok(()));
    let size_pos = cur.pos();
    assert_eq!(cur.write_u32_le(0), Ok(()));
    assert_eq!(cur.write_4(*b"WAVE"), Ok(()));
    assert_eq!(cur.write_at_u32_le(size_pos, 36), Ok(()));
    assert_eq!(&bytes, b"RIFF\x24\x00\x00\x00WAVE");
}
#[test]
fn write_dynamic_bytes() {
    let mut bytes = [0u8; 8];
    let mut cur = ByteCursor::writer(&mut bytes);
    assert_eq!(cur.write(b"abc"), Ok(()));
    assert_eq!(cur.write(b"defgh"), Ok(()));
    assert_eq!(&bytes, b"abcdefgh");
}
#[test]
fn write_bounds() {
    let mut bytes = [0u8; 3];
    let mut cur = ByteCursor::writer(&mut bytes);
    assert_eq!(cur.write_4(*b"RIFF"), Err(NotEnoughSpace(Some(4))));
    assert_eq!(cur.pos(), 0);
    assert_eq!(cur.write(b"abc"), Ok(()));
    assert_eq!(cur.write_u8(1), Err(NotEnoughSpace(Some(1))));
}
