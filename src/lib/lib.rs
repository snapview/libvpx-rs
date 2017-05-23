//! This crate provides Rust bindings to the libvpx library which can be used for VP8/VP9 video
//! encoding/decoding. You may need to refer to the original documentation:
//! http://www.webmproject.org/docs/webm-sdk/modules.html

extern crate libc;
extern crate time;
extern crate vpx_sys as ffi;

mod context;
pub mod encoder;
pub mod image;
mod error;
