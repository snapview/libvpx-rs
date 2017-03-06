#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

// VP9
#[repr(i32)]
pub enum AQ_MODE {
  NO_AQ = 0,
  VARIANCE_AQ = 1,
  COMPLEXITY_AQ = 2,
  CYCLIC_REFRESH_AQ = 3,
  EQUATOR360_AQ = 4,
  // AQ based on lookahead temporal
  // variance (only valid for altref frames)
  LOOKAHEAD_AQ = 5,
}

// Back compat
pub use vpx_codec_err_t::*;

impl Default for vpx_codec_enc_cfg {
    fn default() -> Self {
        unsafe {::std::mem::zeroed()}
    }
}
impl Default for vpx_codec_ctx {
    fn default() -> Self {
        unsafe {::std::mem::zeroed()}
    }
}
impl Default for vpx_image_t {
    fn default() -> Self {
        unsafe {::std::mem::zeroed()}
    }
}

include!(concat!(env!("OUT_DIR"), "/ffi.rs"));
