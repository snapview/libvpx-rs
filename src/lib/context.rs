//! Contains the representation of the encoder/decoder context (instance). It is used to identify
//! the particular instance of encoder/decoder used by the developer.

use std::ops::{Deref, DerefMut};
use ffi;

pub struct CodecContext {
    inner: ffi::vpx_codec_ctx_t
}

impl CodecContext {
    pub fn new() -> Self {
        CodecContext {
            inner: Default::default()
        }
    }
}

impl Drop for CodecContext {
    fn drop(&mut self) {
        unsafe { ffi::vpx_codec_destroy(&mut self.inner) };
    }
}

impl Deref for CodecContext {
    type Target = ffi::vpx_codec_ctx_t;

    fn deref(&self) -> &ffi::vpx_codec_ctx_t {
        &self.inner
    }
}

impl DerefMut for CodecContext {
    fn deref_mut(&mut self) -> &mut ffi::vpx_codec_ctx_t {
        &mut self.inner
    }
}
