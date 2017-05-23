//! Common video encoder functions for VP8/VP9 codecs.

use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::null;

use ffi;
use libc;
use time::Duration;

use context::CodecContext as Context;
use error::{VPXResult as Result, check_err};
use image::Image;

pub use self::frame::{Frame, FramesIter};
pub use self::config::{CodecFlags, EncoderConfig};

mod config;
mod frame;
pub mod vp8;
pub mod vp9;

/// This trait has to be implemented by every codec which can be used by libvpx library.
pub trait VpxEncoder {
    // These types are commented here, but theoretically VP8/VP9 configs are also different, so it
    // could be that we might uncomment them / write them again when we need it in future.
    // type Config: Deref<Target=ffi::vpx_codec_enc_cfg_t>;
    // type CodecFlags: Deref<Target=ffi::vpx_codec_flags_t>;

    /// Codec specific frame flags. The set of encoding flags differs between VP8 and VP9.
    type FrameFlags: Into<ffi::vpx_enc_frame_flags_t> + Default;

    /// Returns a reference to the codec interface (some sort of opaque data structure inside
    /// libvpx.
    fn interface() -> *mut ffi::vpx_codec_iface_t;
}

/// An instance of libvpx-based encoder, you have to specify which codec you want to use, current
/// the supported encoders are: `Encoder<VP8>`, `Encoder<VP9>`.
pub struct Encoder<Enc: VpxEncoder> {
    context: Context,
    _phantom: PhantomData<Enc>,
}

impl<Enc: VpxEncoder> Encoder<Enc> {
    /// Creates a new encoder given the configurations given by the user. In case if the
    /// configurations are not explicitly specified, the default configuration will be used.
    pub fn new(config: Option<EncoderConfig<Enc>>, flags: Option<CodecFlags>) -> Result<Self> {
        let iface = Enc::interface();
        let config = config.unwrap_or(EncoderConfig::<Enc>::new()?);
        let flags = flags.unwrap_or(CodecFlags::default());
        let mut ctx = Context::new();
        check_err(unsafe { ffi::vpx_codec_enc_init_ver(&mut *ctx,
                                                       iface,
                                                       &config.into(),
                                                       flags.into(),
                                                       ffi::VPX_ENCODER_ABI_VERSION as i32) })?;
        Ok(Encoder {
            context: ctx,
            _phantom: PhantomData { },
        })
    }

    /// Encodes a single frame, fails in case if the encoding cannot be done. Refer to
    /// `vpx_codec_encode()` to get more info about each parameter.
    pub fn encode(&mut self,
                  image: &Image,
                  pts: ffi::vpx_codec_pts_t,
                  duration: u64,
                  flags: Enc::FrameFlags,
                  deadline: Deadline)
        -> Result<()>
    {
        check_err(unsafe { ffi::vpx_codec_encode(&mut *self.context,
                                                 image.deref(),
                                                 pts,
                                                 duration,
                                                 flags.into(),
                                                 deadline.into()) })?;
        Ok(())
    }

    /// Returns a frame iterator which can be used to iterate over encoded frames so far. You can
    /// call this function directly after `encode()`, but you are not obliged to. You cannot call
    /// any encoding functions while you own a frame iterator.
    pub fn frames_iter(&mut self) -> FramesIter {
        FramesIter::new(&mut self.context)
    }
}

impl<Enc: VpxEncoder> Drop for Encoder<Enc> {
    fn drop(&mut self) {
        // According to libvpx documentation we have to flush the encoder data to ensure that the
        // encoding is stopped and the encoder is done. To do that we have to push a null frame
        // (the parameters has been taken from libvpx examples) and then get the frame iterator and
        // iterate till the end of the stream.
        let res = unsafe {
            ffi::vpx_codec_encode(&mut *self.context, null(), -1, 1, 0, Deadline::GoodQuality.into())
        };
        check_err(res).expect("Could not release the encoder resource");
        let frames_iter = self.frames_iter();
        for _ in frames_iter {
        }
    }
}

/// Soft realtime deadline parameters for libvpx encoder.
pub enum Deadline {
    Realtime,
    GoodQuality,
    BestQuality,
    Custom(Duration),
}

impl Into<libc::c_ulong> for Deadline {
    fn into(self) -> libc::c_ulong {
        match self {
            Deadline::Realtime => ffi::VPX_DL_REALTIME as libc::c_ulong,
            Deadline::GoodQuality => ffi::VPX_DL_GOOD_QUALITY as libc::c_ulong,
            Deadline::BestQuality => ffi::VPX_DL_BEST_QUALITY as libc::c_ulong,
            Deadline::Custom(duration) => duration.num_microseconds()
                                                  .unwrap_or(ffi::VPX_DL_BEST_QUALITY as i64)
                                                  as libc::c_ulong,
        }
    }
}
