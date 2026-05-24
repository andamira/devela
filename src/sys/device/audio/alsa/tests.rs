// devela::sys::device::audio::alsa::tests

use crate::{Alsa, AlsaError, AudioChannels, PcmSample, PcmSpec};

#[test]
#[cfg(ffi_alsa··)]
fn alsa_default_playback_configure_drain() -> Result<(), AlsaError> {
    let mut pcm = Alsa::open_default_playback()?;
    let spec =
        pcm.configure_interleaved(PcmSpec::new(PcmSample::I16, AudioChannels::Stereo, 48_000))?;
    assert_eq!(spec.sample, PcmSample::I16);
    assert_eq!(spec.channels, AudioChannels::Stereo);
    assert!(spec.sample_rate > 0);
    pcm.drain()?;
    Ok(())
}
#[test]
#[cfg(ffi_alsa··)]
fn alsa_list_pcm_devices() -> Result<(), AlsaError> {
    let mut count = 0;
    Alsa::for_each_pcm_device(|dev| {
        assert!(!dev.id.is_empty());
        count += 1;
        Ok(())
    })?;
    assert!(count > 0);
    Ok(())
}
