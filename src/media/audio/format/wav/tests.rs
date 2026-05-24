// devela::media::audio::format::wav::tests

use super::*;
#[cfg(feature = "std")]
use crate::Fs;
use crate::{AudioChannels, PcmSample};

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

#[cfg(feature = "alloc")]
#[test]
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

#[cfg(feature = "alloc")]
#[test]
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

#[cfg(feature = "std")]
#[test]
fn reads_wav_file() {
    let path = std::env::temp_dir().join("_test_16k_i16.wav");
    Fs::write(&path, WAV_I16_STEREO).unwrap();
    let wav = PcmWav::from_file(&path).unwrap();
    assert_eq!(wav.fmt().sample_rate, 44_100);
    assert_eq!(wav.frames(), 2);
    assert_eq!(wav.spec().unwrap().sample, PcmSample::I16);
    let _ = Fs::remove_file(path);
}
