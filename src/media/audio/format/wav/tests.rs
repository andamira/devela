// devela::media::audio::format::wav::tests

use super::*;
#[cfg(feature = "std")]
use crate::Fs;
use crate::{AudioChannels, PcmSample, PcmSpec};

/* helpers */

const WAV_I16_STEREO: &[u8] = b"RIFF\x2C\0\0\0\
      WAVE\
      fmt \x10\0\0\0\
      \x01\0\
      \x02\0\
      \x44\xAC\0\0\
      \x10\xB1\x02\0\
      \x04\0\
      \x10\0\
      data\x08\0\0\0\
      \0\0\0\0\xFF\x7F\0\x80";

const WAV_WITH_JUNK: &[u8] = b"RIFF\x36\0\0\0\
      WAVE\
      JUNK\x01\0\0\0x\0\
      fmt \x10\0\0\0\
      \x01\0\
      \x02\0\
      \x44\xAC\0\0\
      \x10\xB1\x02\0\
      \x04\0\
      \x10\0\
      data\x08\0\0\0\
      \0\0\0\0\xFF\x7F\0\x80";

const NOT_WAVE: &[u8] = b"RIFF\x04\0\0\0AVI ";

const MISSING_FMT: &[u8] = b"RIFF\x0C\0\0\0WAVEdata\0\0\0\0";

const BAD_BLOCK_ALIGN: &[u8] = b"RIFF\x2C\0\0\0\
      WAVE\
      fmt \x10\0\0\0\
      \x01\0\
      \x02\0\
      \x44\xAC\0\0\
      \x10\xB1\x02\0\
      \x02\0\
      \x10\0\
      data\x08\0\0\0\
      \0\0\0\0\xFF\x7F\0\x80";

const WAV_16K_I16_FILE: &[u8] = include_bytes!("_test_16k_i16.wav");

#[cfg(feature = "std")]
fn test_file_neighbor(name: &str) -> std::path::PathBuf {
    use std::path::{Path, PathBuf};
    let file = Path::new(file!());
    let source = if file.is_absolute() {
        PathBuf::from(file)
    } else {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(file)
    };
    source.parent().unwrap().join(name)
}

/* reading */

#[test]
fn parses_i16_stereo() {
    let wav = PcmWav::parse(WAV_I16_STEREO).unwrap();
    assert_eq!(wav.fmt().format_tag, PcmWav::FORMAT_PCM);
    assert_eq!(wav.fmt().channels, 2);
    assert_eq!(wav.fmt().sample_rate, 44_100);
    assert_eq!(wav.fmt().block_align, 4);
    assert_eq!(wav.fmt().bits_per_sample, 16);
    assert_eq!(wav.data_bytes().len(), 8);
    assert_eq!(wav.frames(), 2);
    let spec = wav.spec().unwrap();
    assert_eq!(spec.sample, PcmSample::I16);
    assert_eq!(spec.channels, AudioChannels::Stereo);
    assert_eq!(spec.sample_rate, 44_100);
}
#[test]
fn skips_unknown_padded_chunks() {
    let wav = PcmWav::parse(WAV_WITH_JUNK).unwrap();
    assert_eq!(wav.fmt().channels, 2);
    assert_eq!(wav.data_bytes().len(), 8);
    assert_eq!(wav.frames(), 2);
}
#[test]
fn rejects_non_wave_riff() {
    assert_eq!(PcmWav::parse(NOT_WAVE), Err(PcmWavError::NotWave));
}
#[test]
fn rejects_missing_fmt() {
    assert_eq!(PcmWav::parse(MISSING_FMT), Err(PcmWavError::MissingFmt));
}
#[test]
fn rejects_bad_block_align() {
    assert_eq!(PcmWav::parse(BAD_BLOCK_ALIGN), Err(PcmWavError::InvalidBlockAlign));
}
#[test]
fn parses_const() {
    const WAV: Result<PcmWavBuf<&'static [u8]>, PcmWavError> = PcmWav::parse(WAV_I16_STEREO);
    const FRAMES: usize = match WAV {
        Ok(wav) => wav.frames(),
        Err(_) => 0,
    };
    assert_eq!(FRAMES, 2);
}
#[test]
#[cfg(feature = "alloc")]
fn parses_wav_alloc() {
    let wav = PcmWav::from_vec(WAV_I16_STEREO.to_vec()).unwrap();
    assert_eq!(wav.fmt().channels, 2);
    assert_eq!(wav.data_bytes().len(), 8);
    assert_eq!(wav.frames(), 2);
    let borrowed = wav.as_borrowed();
    assert_eq!(borrowed.fmt(), wav.fmt());
    assert_eq!(borrowed.data_bytes(), wav.data_bytes());
    assert_eq!(borrowed.data_offset(), wav.data_offset());
}
#[test]
#[cfg(feature = "alloc")]
fn parses_u8_pcm_spec() {
    const WAV_U8_MONO: &[u8] = b"RIFF\x26\0\0\0\
      WAVE\
      fmt \x10\0\0\0\
      \x01\0\
      \x01\0\
      \x40\x1F\0\0\
      \x40\x1F\0\0\
      \x01\0\
      \x08\0\
      data\x01\0\0\0\
      \x80\0";
    let wav = PcmWav::parse(WAV_U8_MONO).unwrap();
    let spec = wav.spec().unwrap();
    assert_eq!(spec.sample, PcmSample::U8);
    assert_eq!(spec.channels, AudioChannels::Mono);
    assert_eq!(spec.sample_rate, 8_000);
    assert_eq!(wav.frames(), 1);
}
#[test]
#[cfg(feature = "std")]
fn reads_wav_file() {
    let path = std::env::temp_dir().join("_test_16k_i16.wav");
    Fs::write(&path, WAV_I16_STEREO).unwrap();
    let wav = PcmWav::from_file(&path).unwrap();
    assert_eq!(wav.fmt().sample_rate, 44_100);
    assert_eq!(wav.frames(), 2);
    assert_eq!(wav.spec().unwrap().sample, PcmSample::I16);
    let _ = Fs::remove_file(path);
}

/* writing */

#[test]
fn parses_embedded_wav_file_fixture() {
    let wav = PcmWav::parse(WAV_16K_I16_FILE).unwrap();
    assert_eq!(wav.fmt().sample_rate, 16_000);
    assert_eq!(wav.spec().unwrap().sample, PcmSample::I16);
    assert_eq!(wav.frames(), 3146); // 0.2 seconds
}

#[test]
#[cfg(feature = "std")]
fn reads_wav_file_fixture() {
    let path = test_file_neighbor("_test_16k_i16.wav");
    let wav = PcmWav::from_file(&path).unwrap();
    assert_eq!(wav.fmt().sample_rate, 16_000);
    assert_eq!(wav.spec().unwrap().sample, PcmSample::I16);
    assert_eq!(wav.frames(), 3146); // 0.2 seconds
}
#[test]
fn encodes_i16_stereo_wav() {
    let fmt = PcmWavFmt::new(PcmWav::FORMAT_PCM, 2, 44_100, 16, 0).unwrap();
    let data = b"\0\0\0\0\xFF\x7F\0\x80";
    let len = PcmWav::encode_len(fmt, data.len()).unwrap();
    assert_eq!(len, 44 + data.len());
    let mut out = [0u8; 64];
    let written = PcmWav::encode_into(&mut out, fmt, data).unwrap();
    assert_eq!(written, len);
    assert_eq!(&out[0..4], b"RIFF");
    assert_eq!(&out[8..12], b"WAVE");
    assert_eq!(&out[12..16], b"fmt ");
    assert_eq!(&out[36..40], b"data");
    let parsed = PcmWav::parse(&out[..written]).unwrap();
    assert_eq!(parsed.fmt(), fmt);
    assert_eq!(parsed.data_bytes(), data);
    assert_eq!(parsed.frames(), 2);
}
#[test]
fn encodes_u8_mono_with_pad() {
    let fmt = PcmWavFmt::new(PcmWav::FORMAT_PCM, 1, 8_000, 8, 0).unwrap();
    let data = &[0x80];
    let len = PcmWav::encode_len(fmt, data.len()).unwrap();
    // 44 bytes base header + 1 data byte + 1 RIFF pad byte.
    assert_eq!(len, 46);
    let mut out = [0u8; 64];
    let written = PcmWav::encode_into(&mut out, fmt, data).unwrap();
    assert_eq!(written, len);
    assert_eq!(out[written - 1], 0);
    let parsed = PcmWav::parse(&out[..written]).unwrap();
    assert_eq!(parsed.spec().unwrap().sample, PcmSample::U8);
    assert_eq!(parsed.data_bytes(), data);
    assert_eq!(parsed.frames(), 1);
}
#[test]
fn encodes_f32_with_fact_chunk() {
    let fmt = PcmWavFmt::new(PcmWav::FORMAT_IEEE_FLOAT, 1, 48_000, 32, 0).unwrap();
    let data = &[0u8; 8]; // 2 mono f32 frames
    let len = PcmWav::encode_len(fmt, data.len()).unwrap();
    // RIFF/WAVE 12, fmt chunk 8 + 18, fact chunk 8 + 4, data chunk 8 + 8
    assert_eq!(len, 12 + 26 + 12 + 16);
    let mut out = [0u8; 80];
    let written = PcmWav::encode_into(&mut out, fmt, data).unwrap();
    assert_eq!(written, len);
    assert_eq!(&out[12..16], b"fmt ");
    assert_eq!(&out[38..42], b"fact");
    assert_eq!(&out[50..54], b"data");
    let parsed = PcmWav::parse(&out[..written]).unwrap();
    assert_eq!(parsed.fmt().format_tag, PcmWav::FORMAT_IEEE_FLOAT);
    assert_eq!(parsed.fmt().extra_len, 0);
    assert_eq!(parsed.spec().unwrap().sample, PcmSample::F32);
    assert_eq!(parsed.data_bytes(), data);
}
#[test]
fn encode_rejects_small_destination() {
    let fmt = PcmWavFmt::new(PcmWav::FORMAT_PCM, 1, 8_000, 8, 0).unwrap();
    let mut out = [0u8; 8];
    assert_eq!(PcmWav::encode_into(&mut out, fmt, &[0x80]), Err(PcmWavError::NotEnoughSpace),);
}
#[test]
#[cfg(feature = "alloc")]
fn encodes_to_vec_and_reparses() {
    let spec = PcmSpec::new(PcmSample::I16, AudioChannels::Stereo, 44_100);
    let data = b"\0\0\0\0\xFF\x7F\0\x80";
    let bytes = PcmWav::spec_to_vec(spec, data).unwrap();
    let wav = PcmWav::parse(&bytes).unwrap();
    assert_eq!(wav.spec().unwrap(), spec);
    assert_eq!(wav.data_bytes(), data);
}
#[test]
#[cfg(feature = "std")]
fn writes_wav_file() {
    let path = std::env::temp_dir().join("devela_test_pcm_wav_write.wav");
    let fmt = PcmWavFmt::new(PcmWav::FORMAT_PCM, 1, 8_000, 8, 0).unwrap();
    PcmWav::to_file(&path, fmt, &[0x80]).unwrap();
    let wav = PcmWav::from_file(&path).unwrap();
    assert_eq!(wav.spec().unwrap().sample, PcmSample::U8);
    assert_eq!(wav.data_bytes(), &[0x80]);
    let _ = Fs::remove_file(path);
}
