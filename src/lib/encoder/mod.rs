use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::null;

use ffi;
use libc;
use time::Duration;

use context::CodecContext as Context;
use error::{VPXResult as Result, check_err};
use image::Image;
use self::frame::FramesIter;

mod frame;

pub trait VpxEncoder {
    type Config: Deref<Target=ffi::vpx_codec_enc_cfg_t>;
    type CodecFlags: Deref<Target=ffi::vpx_codec_flags_t>;
    type FrameFlags: Deref<Target=ffi::vpx_enc_frame_flags_t>;

    fn create_interface() -> *mut ffi::vpx_codec_iface_t;
    fn create_default_config() -> Result<Self::Config>;
    fn create_default_codec_flags() -> Result<Self::CodecFlags>;
    fn create_default_frame_flags() -> Result<Self::FrameFlags>;
}

pub struct Encoder<Enc: VpxEncoder> {
    context: Context,
    _phantom: PhantomData<Enc>,
}

impl<Enc: VpxEncoder> Encoder<Enc> {
    pub fn new(config: Option<Enc::Config>, flags: Option<Enc::CodecFlags>) -> Result<Self> {
        let mut config = config.unwrap_or(Enc::create_default_config()?);
        let mut flags = flags.unwrap_or(Enc::create_default_codec_flags()?);
        let mut ctx = Context::new();
        check_err(unsafe { ffi::vpx_codec_enc_init_ver(&mut *ctx,
                                                       Enc::create_interface(),
                                                       &*config,
                                                       *flags,
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
                                                 *flags,
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
