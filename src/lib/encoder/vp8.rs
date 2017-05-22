use ffi;

use encoder::VpxEncoder;

pub struct VP8;

impl VpxEncoder for VP8 {
    type FrameFlags = FrameFlags;

    fn interface() -> *mut ffi::vpx_codec_iface_t {
        unsafe { &mut ffi::vpx_codec_vp8_cx_algo }
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct FrameFlags {
    force_kf: bool,

    no_ref_last: bool,
    no_ref_gf: bool,
    no_ref_arf: bool,
    no_upd_last: bool,
    no_upd_gf: bool,
    no_upd_arf: bool,
    force_gf: bool,
    force_arf: bool,
    no_upd_entropy: bool,
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
        if self.no_ref_last {
            flags |= ffi::VP8_EFLAG_NO_REF_LAST as i64;
        }
        if self.no_ref_gf {
            flags |= ffi::VP8_EFLAG_NO_REF_GF as i64;
        }
        if self.no_ref_arf {
            flags |= ffi::VP8_EFLAG_NO_REF_ARF as i64;
        }
        if self.no_upd_last {
            flags |= ffi::VP8_EFLAG_NO_UPD_LAST as i64;
        }
        if self.no_upd_gf {
            flags |= ffi::VP8_EFLAG_NO_UPD_GF as i64;
        }
        if self.no_upd_arf {
            flags |= ffi::VP8_EFLAG_NO_UPD_ARF as i64;
        }
        if self.no_upd_entropy {
            flags |= ffi::VP8_EFLAG_NO_UPD_ENTROPY as i64;
        }
        flags
    }
}


