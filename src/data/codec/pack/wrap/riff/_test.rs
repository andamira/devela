// devela/src/data/codec/pack/wrap/riff/_test.rs

use crate::{BinTag4, Riff, RiffError};

const fn tag(bytes: [u8; 4]) -> BinTag4 {
    BinTag4::new(bytes)
}

#[test]
fn tag4_helpers() {
    let tag = BinTag4::new(*b"fmt ");

    assert_eq![tag.bytes(), *b"fmt "];
    assert_eq![tag.as_bytes(), b"fmt "];
    assert_eq![tag.trimmed_len(), 3];
    assert![tag.is_ascii()];
    assert![tag.is_ascii_graphic_or_space()];
    assert![tag.is_fourcc()];

    let raw = tag.to_u32_le();
    assert_eq![BinTag4::from_u32_le(raw), tag];
}

#[test]
fn reads_root_riff_chunk() -> Result<(), RiffError> {
    let bytes = b"RIFF\x04\0\0\0WAVE";

    let root = Riff::root(bytes)?;

    assert_eq![root.id(), Riff::RIFF];
    assert_eq![root.size(), 4];
    assert_eq![root.data(), b"WAVE"];
    assert_eq![root.container_type(), Some(Riff::WAVE)];
    assert_eq![root.subchunk_data(), Some(&[][..])];

    Ok(())
}

#[test]
fn iterates_wave_subchunks() -> Result<(), RiffError> {
    let bytes = b"RIFF\x18\0\0\0WAVEfmt \x04\0\0\0\x01\0\x02\0data\x00\0\0\0";

    let root = Riff::root(bytes)?;
    assert_eq![root.container_type(), Some(Riff::WAVE)];

    let mut chunks = root.subchunks()?;

    let fmt = chunks.next_chunk().unwrap()?;
    assert_eq![fmt.id(), tag(*b"fmt ")];
    assert_eq![fmt.size(), 4];
    assert_eq![fmt.data(), &[1, 0, 2, 0]];

    let data = chunks.next_chunk().unwrap()?;
    assert_eq![data.id(), Riff::DATA];
    assert_eq![data.size(), 0];
    assert_eq![data.data(), &[]];

    assert![chunks.next_chunk().is_none()];

    Ok(())
}

#[test]
fn skips_odd_chunk_padding() -> Result<(), RiffError> {
    // One `abc` chunk with 3 bytes of data and one pad byte,
    // followed by one empty `def` chunk.
    let bytes = b"abc \x03\0\0\0xyz\0def \x00\0\0\0";

    let mut chunks = Riff::chunks(bytes);

    let first = chunks.next_chunk().unwrap()?;
    assert_eq![first.id(), tag(*b"abc ")];
    assert_eq![first.data(), b"xyz"];

    let second = chunks.next_chunk().unwrap()?;
    assert_eq![second.id(), tag(*b"def ")];
    assert_eq![second.data(), b""];

    assert![chunks.next_chunk().is_none()];

    Ok(())
}

#[test]
fn rejects_non_riff_root() {
    let bytes = b"LIST\x04\0\0\0INFO";
    assert_eq![Riff::root(bytes), Err(RiffError::NotRiff)];
}

#[test]
fn rejects_rifx_for_now() {
    let bytes = b"RIFX\x04\0\0\0WAVE";
    assert_eq![Riff::root(bytes), Err(RiffError::UnsupportedRifx)];
}

#[test]
fn detects_truncated_header() {
    assert_eq![Riff::chunk(b"RIFF"), Err(RiffError::TruncatedHeader)];
}

#[test]
fn detects_truncated_data() {
    let bytes = b"abcd\x04\0\0\0xy";
    assert_eq![Riff::chunk(bytes), Err(RiffError::TruncatedData)];
}

#[test]
fn detects_truncated_padding() {
    let bytes = b"abcd\x03\0\0\0xyz";
    assert_eq![Riff::chunk(bytes), Err(RiffError::TruncatedPad)];
}

#[test]
fn count_chunks_reports_errors() {
    let good = b"abcd\x00\0\0\0efgh\x00\0\0\0";
    assert_eq![Riff::chunks(good).count_chunks(), Ok(2)];

    let bad = b"abcd\x04\0\0\0xy";
    assert_eq![Riff::chunks(bad).count_chunks(), Err(RiffError::TruncatedData)];
}
