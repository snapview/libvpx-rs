//! Contains the structures which wrap libvpx main encoder configuration.

use std::marker::PhantomData;

use ffi;

use encoder::VpxEncoder;
use error::{VPXResult as Result, check_err};

/// An encoder configuration for a specific codec `Encoder`.
pub struct EncoderConfig<Encoder> {
    inner: ffi::vpx_codec_enc_cfg_t,
    _phantom: PhantomData<Encoder>,
}

impl<Encoder: VpxEncoder> EncoderConfig<Encoder> {
    pub fn new() -> Result<Self> {
        let mut cfg: ffi::vpx_codec_enc_cfg_t = Default::default();
        check_err(unsafe { ffi::vpx_codec_enc_config_default(Encoder::interface(), &mut cfg, 0) })?;
        Ok(EncoderConfig {
            inner: cfg,
            _phantom: PhantomData { }
        })
    }

    pub fn set_frame_dimensions(&mut self, width: u32, height: u32) {
        self.inner.g_w = width;
        self.inner.g_h = height;
    }

    /// Set the timebase.
    ///
    /// > **EXAMPLE**: if you want to write a simple encoder which encodes frames
    /// > and writes them to the file and the FPS is 30, the `numerator` could be `1` and the
    /// > denominator should be `30` in this case. The `pts` (presentation timestamp) during the
    /// > encoding process should be in timebase units (so each frame would have a monotonic pts
    /// > equal to the frame number in case of our example).
    pub fn set_timebase(&mut self, numerator: u32, denominator: u32) {
        self.inner.g_timebase.num = numerator as i32;
        self.inner.g_timebase.den = denominator as i32;
    }

    pub fn set_target_bitrate(&mut self, bitrate: u32) {
        self.inner.rc_target_bitrate = bitrate;
    }
}

impl<Encoder> Into<ffi::vpx_codec_enc_cfg_t> for EncoderConfig<Encoder> {
    fn into(self) -> ffi::vpx_codec_enc_cfg_t {
        self.inner
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct CodecFlags {
    use_psnr: bool,
    use_output_partition: bool,
    use_highbitdepth: bool,
    use_postproc: bool,
    use_error_concealment: bool,
    use_input_fragments: bool,
    use_frame_threading: bool,
}

impl Into<ffi::vpx_codec_flags_t> for CodecFlags {
    fn into(self) -> ffi::vpx_codec_flags_t {
        let mut flags: ffi::vpx_codec_flags_t = 0;
        if self.use_psnr {
            flags |= ffi::VPX_CODEC_USE_PSNR as i64;
        }
        if self.use_output_partition {
            flags |= ffi::VPX_CODEC_USE_OUTPUT_PARTITION as i64;
        }
        if self.use_highbitdepth {
            flags |= ffi::VPX_CODEC_USE_HIGHBITDEPTH as i64;
        }
        if self.use_postproc {
            flags |= ffi::VPX_CODEC_USE_POSTPROC as i64;
        }
        if self.use_error_concealment {
            flags |= ffi::VPX_CODEC_USE_ERROR_CONCEALMENT as i64;
        }
        if self.use_input_fragments {
            flags |= ffi::VPX_CODEC_USE_INPUT_FRAGMENTS as i64;
        }
        if self.use_frame_threading {
            flags |= ffi::VPX_CODEC_USE_FRAME_THREADING as i64;
        }
        flags
    }
}
