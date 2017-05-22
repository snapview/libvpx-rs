use ffi;

use encoder::VpxEncoder;

pub struct VP9;

impl VpxEncoder for VP9 {
    type FrameFlags = FrameFlags;

    fn interface() -> *mut ffi::vpx_codec_iface_t {
        unsafe { &mut ffi::vpx_codec_vp9_cx_algo }
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct FrameFlags {
    force_kf: bool,
}

impl FrameFlags {
    pub fn keyframe(&mut self, keyframe: bool) {
        self.force_kf = keyframe;
    }
}

impl Into<ffi::vpx_enc_frame_flags_t> for FrameFlags {
    fn into(self) -> ffi::vpx_enc_frame_flags_t {
        let mut flags: ffi::vpx_enc_frame_flags_t = 0;
        if self.force_kf {
            flags |= ffi::VPX_EFLAG_FORCE_KF as i64;
        }
        flags
    }
}
