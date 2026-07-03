// devela/src/media/audio/format/wav/_test/mod.rs
//
// TOC
// - mod fixture
// - mod expect
// - mod parse
// - mod encode
// - mod decode
// - owned
// - file_io
// - extensible

use super::*;
use crate::{AudioChannels, PcmSample, PcmSpec};

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
    pub(super) const FILE_16K_I16: &[u8] = include_bytes!("16k_i16.wav");
    pub(super) const WAV_EXT_I24_STEREO: &[u8] = b"RIFF\x48\0\0\0\
      WAVE\
      fmt \x28\0\0\0\
      \xFE\xFF\
      \x02\0\
      \x44\xAC\0\0\
      \x98\x09\x04\0\
      \x06\0\
      \x18\0\
      \x16\0\
      \x18\0\
      \x03\0\0\0\
      \x01\0\0\0\0\0\x10\0\x80\0\0\xAA\0\x38\x9B\x71\
      data\x0C\0\0\0\
      \0\0\0\0\0\0\xFF\xFF\x7F\0\0\x80";
}

/* expectations */

mod expect {
    use super::*;

    #[cfg(feature = "alloc")]
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
    #[test]
    fn parses_extensible_i24_stereo() {
        let wav = PcmWav::parse(fixture::WAV_EXT_I24_STEREO).unwrap();
        let fmt = wav.fmt();
        assert_eq!(fmt.format_tag, PcmWav::FORMAT_EXTENSIBLE);
        assert_eq!(fmt.subformat_tag, PcmWav::FORMAT_PCM);
        assert_eq!(fmt.bits_per_sample, 24);
        assert_eq!(fmt.valid_bits_per_sample, 24);
        assert_eq!(fmt.channel_mask, 0x0000_0003);
        assert_eq!(wav.spec().unwrap().sample, PcmSample::I24);
        assert_eq!(wav.frames(), 2);
    }
}

/* encoding */

mod encode {
    use super::*;

    #[test]
    fn encodes_i16_stereo_wav() {
        let fmt = PcmWavFmt::new(PcmWav::FORMAT_PCM, 2, 44_100, 16, 0).unwrap();
        let data = b"\0\0\0\0\xFF\x7F\0\x80";
        let len = PcmWav::write_len(fmt, data.len()).unwrap();
        assert_eq!(len, 44 + data.len());
        let mut out = [0u8; 64];
        let written = PcmWav::write_into(&mut out, fmt, data).unwrap();
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
        let len = PcmWav::write_len(fmt, data.len()).unwrap();
        assert_eq!(len, 46); // 44 bytes + 1 data byte + 1 RIFF pad byte.
        let mut out = [0u8; 64];
        let written = PcmWav::write_into(&mut out, fmt, data).unwrap();
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
        let len = PcmWav::write_len(fmt, data.len()).unwrap();
        assert_eq!(len, 12 + 26 + 12 + 16);
        let mut out = [0u8; 80];
        let written = PcmWav::write_into(&mut out, fmt, data).unwrap();
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
        assert_eq!(PcmWav::write_into(&mut out, fmt, &[0x80]), Err(PcmWavError::NotEnoughSpace),);
    }
}

/* allocation */

#[cfg(feature = "alloc")]
mod owned {
    use super::{expect, fixture, *};

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
    use super::*;
    use crate::{Env, Fs};

    #[test]
    fn reads_temp_file() {
        let path = Env::temp_dir().join("devela_test_pcm_wav_read.wav");
        Fs::write(&path, fixture::I16_STEREO_44K).unwrap();
        let wav = PcmWav::from_file(&path).unwrap();
        expect::i16_stereo_44k(&wav);
        let _ = Fs::remove_file(path);
    }
    #[test]
    fn writes_and_reparses_file() {
        let path = Env::temp_dir().join("devela_test_pcm_wav_write.wav");
        let fmt = PcmWavFmt::new(PcmWav::FORMAT_PCM, 1, 8_000, 8, 0).unwrap();
        PcmWav::to_file(&path, fmt, &[0x80]).unwrap();
        let wav = PcmWav::from_file(&path).unwrap();
        assert_eq!(wav.spec().unwrap().sample, PcmSample::U8);
        assert_eq!(wav.data_bytes(), &[0x80]);
        let _ = Fs::remove_file(path);
    }
}

mod extensible {
    use super::*;

    const WAV_EXT_I24_STEREO: &[u8] = b"RIFF\x48\0\0\0\
          WAVE\
          fmt \x28\0\0\0\
          \xFE\xFF\
          \x02\0\
          \x44\xAC\0\0\
          \x98\x09\x04\0\
          \x06\0\
          \x18\0\
          \x16\0\
          \x18\0\
          \x03\0\0\0\
          \x01\0\0\0\0\0\x10\0\x80\0\0\xAA\0\x38\x9B\x71\
          data\x0C\0\0\0\
          \0\0\0\0\0\0\xFF\xFF\x7F\0\0\x80";
    const I24_STEREO_DATA: &[u8] = b"\0\0\0\0\0\0\xFF\xFF\x7F\0\0\x80";
    #[test]
    fn parses_extensible_i24_stereo() {
        let wav = PcmWav::parse(WAV_EXT_I24_STEREO).unwrap();
        let fmt = wav.fmt();
        assert_eq!(fmt.format_tag, PcmWav::FORMAT_EXTENSIBLE);
        assert_eq!(fmt.subformat_tag, PcmWav::FORMAT_PCM);
        assert_eq!(fmt.channels, 2);
        assert_eq!(fmt.sample_rate, 44_100);
        assert_eq!(fmt.byte_rate, 264_600);
        assert_eq!(fmt.block_align, 6);
        assert_eq!(fmt.bits_per_sample, 24);
        assert_eq!(fmt.valid_bits_per_sample, 24);
        assert_eq!(fmt.extra_len, PcmWav::EXTENSIBLE_EXTRA_LEN);
        assert_eq!(fmt.channel_mask, 0x0000_0003);
        assert_eq!(wav.data_bytes(), I24_STEREO_DATA);
        assert_eq!(wav.frames(), 2);
        let spec = wav.spec().unwrap();
        assert_eq!(spec, PcmSpec::new(PcmSample::I24, AudioChannels::Stereo, 44_100));
        let raw = wav.raw().unwrap();
        let mut samples = [0i32; 4];
        let written = raw.decode_i24_le_into(&mut samples).unwrap();
        assert_eq!(written, 4);
        assert_eq!(samples, [0, 0, 8_388_607, -8_388_608]);
    }
    #[test]
    fn writes_extensible_i24_stereo_and_reparses() {
        let fmt =
            PcmWavFmt::new_extensible(PcmWav::FORMAT_PCM, 2, 44_100, 24, 24, 0x0000_0003).unwrap();
        let len = PcmWav::write_len(fmt, I24_STEREO_DATA.len()).unwrap();
        assert_eq!(len, WAV_EXT_I24_STEREO.len());
        let mut out = [0u8; 80];
        let written = PcmWav::write_into(&mut out, fmt, I24_STEREO_DATA).unwrap();
        assert_eq!(written, len);
        assert_eq!(&out[..written], WAV_EXT_I24_STEREO);
        let wav = PcmWav::parse(&out[..written]).unwrap();
        assert_eq!(wav.fmt(), fmt);
        assert_eq!(wav.data_bytes(), I24_STEREO_DATA);
    }
    #[test]
    fn writes_extensible_f32_with_fact_chunk() {
        let fmt =
            PcmWavFmt::new_extensible(PcmWav::FORMAT_IEEE_FLOAT, 1, 48_000, 32, 32, 0x0000_0004)
                .unwrap();
        let data = [0u8; 8]; // 2 mono f32 frames
        let len = PcmWav::write_len(fmt, data.len()).unwrap();
        // RIFF/WAVE 12 + fmt 48 + fact 12 + data 16.
        assert_eq!(len, 88);
        let mut out = [0u8; 88];
        let written = PcmWav::write_into(&mut out, fmt, &data).unwrap();
        assert_eq!(written, len);
        assert_eq!(&out[12..16], b"fmt ");
        assert_eq!(&out[60..64], b"fact");
        assert_eq!(&out[72..76], b"data");
        let wav = PcmWav::parse(&out).unwrap();
        assert_eq!(wav.fmt().format_tag, PcmWav::FORMAT_EXTENSIBLE);
        assert_eq!(wav.fmt().subformat_tag, PcmWav::FORMAT_IEEE_FLOAT);
        assert_eq!(wav.spec().unwrap().sample, PcmSample::F32);
        assert_eq!(wav.data_bytes(), data);
    }
    #[test]
    fn rejects_extensible_with_too_small_extra_len() {
        let mut bytes = [0u8; 80];
        bytes.copy_from_slice(WAV_EXT_I24_STEREO);
        // Absolute offset of cbSize inside the `fmt ` payload.
        bytes[36] = 0x14;
        bytes[37] = 0x00;
        assert_eq!(PcmWav::parse(&bytes), Err(PcmWavError::InvalidExtensibleFormat));
    }
    #[test]
    fn rejects_extensible_with_unsupported_subformat_guid() {
        let mut bytes = [0u8; 80];
        bytes.copy_from_slice(WAV_EXT_I24_STEREO);
        // First byte of the subformat GUID.
        bytes[44] = 0x7F;
        assert_eq!(PcmWav::parse(&bytes), Err(PcmWavError::UnsupportedSubformat));
    }
    #[test]
    fn rejects_extensible_with_invalid_valid_bits() {
        assert_eq!(
            PcmWavFmt::new_extensible(PcmWav::FORMAT_PCM, 2, 44_100, 24, 0, 0x3),
            Err(PcmWavError::InvalidExtensibleFormat),
        );
        assert_eq!(
            PcmWavFmt::new_extensible(PcmWav::FORMAT_PCM, 2, 44_100, 24, 25, 0x3),
            Err(PcmWavError::InvalidExtensibleFormat),
        );
    }
    #[test]
    fn helper_roundtrips_subformat_guid() {
        let pcm = PcmWav::subformat_guid(PcmWav::FORMAT_PCM).unwrap();
        let float = PcmWav::subformat_guid(PcmWav::FORMAT_IEEE_FLOAT).unwrap();
        assert_eq!(PcmWav::subformat_tag(pcm), Ok(PcmWav::FORMAT_PCM));
        assert_eq!(PcmWav::subformat_tag(float), Ok(PcmWav::FORMAT_IEEE_FLOAT));
    }
    #[test]
    fn bytes_per_sample_uses_effective_format_tag() {
        assert_eq!(PcmWav::bytes_per_sample(PcmWav::FORMAT_PCM, 24), Ok(3));
        assert_eq!(PcmWav::bytes_per_sample(PcmWav::FORMAT_IEEE_FLOAT, 32), Ok(4));
        // Extensible must resolve to its subformat first.
        assert_eq!(
            PcmWav::bytes_per_sample(PcmWav::FORMAT_EXTENSIBLE, 24),
            Err(PcmWavError::UnsupportedBitsPerSample(24)),
        );
    }
}
