// devela::media::audio::format::wav::tests

use super::*;
use crate::{AudioChannels, PcmSample};

/* fixtures */

mod fixture {
    pub(super) const I16_STEREO_44K: &[u8] = b"RIFF\x2C\0\0\0\
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
    pub(super) const I16_STEREO_44K_WITH_JUNK: &[u8] = b"RIFF\x36\0\0\0\
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
    pub(super) const U8_MONO_8K: &[u8] = b"RIFF\x26\0\0\0\
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
    pub(super) const NOT_WAVE: &[u8] = b"RIFF\x04\0\0\0AVI ";
    pub(super) const MISSING_FMT: &[u8] = b"RIFF\x0C\0\0\0WAVEdata\0\0\0\0";
    pub(super) const BAD_BLOCK_ALIGN: &[u8] = b"RIFF\x2C\0\0\0\
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
    pub(super) const FILE_16K_I16: &[u8] = include_bytes!("_test_16k_i16.wav");
}

/* expectations */

mod expect {
    use super::*;

    pub(super) fn i16_stereo_44k<B: AsRef<[u8]>>(wav: &PcmWavBuf<B>) {
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
    pub(super) fn file_16k_i16<B: AsRef<[u8]>>(wav: &PcmWavBuf<B>) {
        assert_eq!(wav.fmt().sample_rate, 16_000);
        assert_eq!(wav.spec().unwrap().sample, PcmSample::I16);
        assert_eq!(wav.frames(), 3146); // 0.2 seconds
    }
}

/* parsing */

mod parse {
    use super::{expect, fixture, *};

    #[test]
    fn parses_u8_mono_spec() {
        let wav = PcmWav::parse(fixture::U8_MONO_8K).unwrap();
        let spec = wav.spec().unwrap();
        assert_eq!(spec.sample, PcmSample::U8);
        assert_eq!(spec.channels, AudioChannels::Mono);
        assert_eq!(spec.sample_rate, 8_000);
        assert_eq!(wav.frames(), 1);
    }
    #[test]
    fn parses_embedded_file_fixture() {
        let wav = PcmWav::parse(fixture::FILE_16K_I16).unwrap();
        expect::file_16k_i16(&wav);
    }
    #[test]
    fn skips_unknown_padded_chunks() {
        let wav = PcmWav::parse(fixture::I16_STEREO_44K_WITH_JUNK).unwrap();
        assert_eq!(wav.fmt().channels, 2);
        assert_eq!(wav.data_bytes().len(), 8);
        assert_eq!(wav.frames(), 2);
    }
    #[test]
    fn rejects_non_wave_riff() {
        assert_eq!(PcmWav::parse(fixture::NOT_WAVE), Err(PcmWavError::NotWave));
    }
    #[test]
    fn rejects_missing_fmt() {
        assert_eq!(PcmWav::parse(fixture::MISSING_FMT), Err(PcmWavError::MissingFmt));
    }
    #[test]
    fn rejects_bad_block_align() {
        assert_eq!(PcmWav::parse(fixture::BAD_BLOCK_ALIGN), Err(PcmWavError::InvalidBlockAlign),);
    }
    #[test]
    fn parses_const() {
        const WAV: Result<PcmWavBuf<&'static [u8]>, PcmWavError> =
            PcmWav::parse(fixture::I16_STEREO_44K);
        const FRAMES: usize = match WAV {
            Ok(wav) => wav.frames(),
            Err(_) => 0,
        };
        assert_eq!(FRAMES, 2);
    }
}

/* encoding */

mod encode {
    use super::*;

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
        assert_eq!(len, 46); // 44 bytes + 1 data byte + 1 RIFF pad byte.
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
    fn rejects_small_destination() {
        let fmt = PcmWavFmt::new(PcmWav::FORMAT_PCM, 1, 8_000, 8, 0).unwrap();
        let mut out = [0u8; 8];
        assert_eq!(PcmWav::encode_into(&mut out, fmt, &[0x80]), Err(PcmWavError::NotEnoughSpace),);
    }
}

/* allocation */

#[cfg(feature = "alloc")]
mod owned {
    use super::{expect, fixture, *};
    use crate::PcmSpec;

    #[test]
    fn parses_owned_vec() {
        let wav = PcmWav::from_vec(fixture::I16_STEREO_44K.to_vec()).unwrap();
        expect::i16_stereo_44k(&wav);
        let borrowed = wav.as_borrowed();
        assert_eq!(borrowed.fmt(), wav.fmt());
        assert_eq!(borrowed.data_bytes(), wav.data_bytes());
        assert_eq!(borrowed.data_offset(), wav.data_offset());
    }
    #[test]
    fn encodes_spec_to_vec_and_reparses() {
        let spec = PcmSpec::new(PcmSample::I16, AudioChannels::Stereo, 44_100);
        let data = b"\0\0\0\0\xFF\x7F\0\x80";
        let bytes = PcmWav::spec_to_vec(spec, data).unwrap();
        let wav = PcmWav::parse(&bytes).unwrap();
        assert_eq!(wav.spec().unwrap(), spec);
        assert_eq!(wav.data_bytes(), data);
    }
}

/* file I/O */

#[cfg(feature = "std")]
mod file_io {
    use super::{expect, fixture, *};
    use crate::Fs;

    #[test]
    fn reads_temp_file() {
        let path = std::env::temp_dir().join("devela_test_pcm_wav_read.wav");
        Fs::write(&path, fixture::I16_STEREO_44K).unwrap();
        let wav = PcmWav::from_file(&path).unwrap();
        expect::i16_stereo_44k(&wav);
        let _ = Fs::remove_file(path);
    }
    #[test]
    fn writes_and_reparses_file() {
        let path = std::env::temp_dir().join("devela_test_pcm_wav_write.wav");
        let fmt = PcmWavFmt::new(PcmWav::FORMAT_PCM, 1, 8_000, 8, 0).unwrap();
        PcmWav::to_file(&path, fmt, &[0x80]).unwrap();
        let wav = PcmWav::from_file(&path).unwrap();
        assert_eq!(wav.spec().unwrap().sample, PcmSample::U8);
        assert_eq!(wav.data_bytes(), &[0x80]);
        let _ = Fs::remove_file(path);
    }
}
