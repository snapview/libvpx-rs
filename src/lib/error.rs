use std;
use ffi;

type VPXResult<T> = Result<T, CodecError>;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
/// Corresponds to the `vpx_codec_err_t` enum in C libvpx library.
pub enum CodecError {
    /// Operation completed without error.
    NoError,
    /// Unspecified error.
    Unspecified,
    /// Memory operation failed.
    Mem,
    /// ABI version mismatch.
    AbiMismatch,
    /// Algorithm does not have required capability.
    Incapable,
    /// The bitstream was unable to be parsed at the highest level. The decoder is unable to
    /// proceed. This error SHOULD be treated as fatal to the stream.
    UnsupportedBitstream,
    /// The decoder does not implement a feature required by the encoder. This return code should
    /// only be used for features that prevent future pictures from being properly decoded. This
    /// error MAY be treated as fatal to the stream or MAY be treated as fatal to the current GOP.
    UnsupportedFrame,
    /// There was a problem decoding the current frame. This return code should only be used for
    /// failures that prevent future pictures from being properly decoded. This error MAY be
    /// treated as fatal to the stream or MAY be treated as fatal to the current GOP. If decoding
    /// is continued for the current GOP, artifacts may be present.
    CorruptFrame,
    /// An application-supplied parameter is not valid.
    InvalidParam,
    /// An iterator reached the end of list.
    ListEnd,
}

impl From<ffi::vpx_codec_err_t> for CodecError {
    fn from(v: ffi::vpx_codec_err_t) -> CodecError {
        match v {
            ffi::VPX_CODEC_OK => CodecError::NoError,
            ffi::VPX_CODEC_ERROR => CodecError::Unspecified,
            ffi::VPX_CODEC_MEM_ERROR => CodecError::Mem,
            ffi::VPX_CODEC_ABI_MISMATCH => CodecError::AbiMismatch,
            ffi::VPX_CODEC_INCAPABLE => CodecError::Incapable,
            ffi::VPX_CODEC_UNSUP_BITSTREAM => CodecError::UnsupportedBitstream,
            ffi::VPX_CODEC_UNSUP_FEATURE => CodecError::UnsupportedFrame,
            ffi::VPX_CODEC_CORRUPT_FRAME => CodecError::CorruptFrame,
            ffi::VPX_CODEC_INVALID_PARAM => CodecError::InvalidParam,
            ffi::VPX_CODEC_LIST_END => CodecError::ListEnd,
        }
    }
}

impl std::fmt::Display for CodecError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, fmt)
    }
}

impl std::error::Error for CodecError {
    fn description(&self) -> &str {
        match *self {
            CodecError::NoError => "No error occurred",
            CodecError::Unspecified => "Unspecified error",
            CodecError::Mem => "Memory operation failed",
            CodecError::AbiMismatch => "ABI version mismatch",
            CodecError::Incapable => "Algorithm does not have required capability",
            CodecError::UnsupportedBitstream => "The given bitstream is not supported",
            CodecError::UnsupportedFrame => "Encoded bitstream uses an unsupported feature",
            CodecError::CorruptFrame => "The coded data for this stream is corrupt or incomplete",
            CodecError::InvalidParam => "An application-supplied parameter is not valid",
            CodecError::ListEnd => "An iterator reached the end of list",
        }
    }
}

pub fn check_err(value: ffi::vpx_codec_err_t) -> VPXResult<ffi::vpx_codec_err_t> {
    if value != ffi::VPX_CODEC_OK {
        Err(CodecError::from(value))
    } else {
        Ok(value)
    }
}
