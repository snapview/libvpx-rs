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

pub trait VpxEncoder {
    // type Config: Deref<Target=ffi::vpx_codec_enc_cfg_t>;
    // type CodecFlags: Deref<Target=ffi::vpx_codec_flags_t>;
    type FrameFlags: Into<ffi::vpx_enc_frame_flags_t> + Default;
    fn interface() -> *mut ffi::vpx_codec_iface_t;
}

pub struct Encoder<Enc: VpxEncoder> {
    context: Context,
    _phantom: PhantomData<Enc>,
}

impl<Enc: VpxEncoder> Encoder<Enc> {
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

    pub fn frames_iter(&mut self) -> FramesIter {
        FramesIter::new(&mut self.context)
    }
}

impl<Enc: VpxEncoder> Drop for Encoder<Enc> {
    fn drop(&mut self) {
        let res = unsafe {
            ffi::vpx_codec_encode(&mut *self.context, null(), -1, 1, 0, Deadline::GoodQuality.into())
        };
        check_err(res).expect("Could not release the encoder resource");
        let frames_iter = self.frames_iter();
        for _ in frames_iter {
        }
    }
}

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
