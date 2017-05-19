use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use ffi;

pub struct Image<'data> {
    inner: ffi::vpx_image_t,
    format: Format,
    data: Cow<'data, [u8]>,
}

impl<'data> Image<'data> {
    /// Creates a wrapper around an image data of the given format. **Does not check that the
    /// container is big enough.**
    pub fn new(data: Cow<'data, [u8]>,
               fmt: Format,
               color_space: ColorSpace,
               width: u32,
               height: u32,
               stride: u32) -> Self
    {
        let mut img: ffi::vpx_image_t = Default::default();
        unsafe { ffi::vpx_img_wrap(&mut img, fmt.into(), width, height,
                                   stride, data.as_ptr() as *mut _) };
        img.cs = color_space.into();

        Image {
            inner: img,
            format: fmt,
            data: data,
        }
    }

    pub fn get_format(&self) -> &Format {
        &self.format
    }
}

impl<'data> Drop for Image<'data> {
    fn drop(&mut self) {
        unsafe { ffi::vpx_img_free(&mut self.inner) }
    }
}

impl<'data> Deref for Image<'data> {
    type Target = ffi::vpx_image_t;

    fn deref(&self) -> &ffi::vpx_image_t {
        &self.inner
    }
}

impl<'data> DerefMut for Image<'data> {
    fn deref_mut(&mut self) -> &mut ffi::vpx_image_t {
        &mut self.inner
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
#[allow(non_camel_case_types)]
pub enum Format {
    RGB24,
    RGB32 { le: bool, },
    RGB565 { le: bool, },
    RGB555 { le: bool, },

    UYVY,
    YUY2,
    YVYU,
    BGR24,
    ARGB,
    BGRA,

    YV12_VPX,
    I420_VPX,

    YV12,

    I420 { hi_bit_depth: bool },
    I422 { hi_bit_depth: bool },
    I440 { hi_bit_depth: bool },
    I444 { hi_bit_depth: bool },

    /// Should be named `444A`.
    I444A,
}

impl Into<ffi::vpx_img_fmt_t> for Format {
    fn into(self) -> ffi::vpx_img_fmt_t {
        use self::Format::*;
        use ffi::vpx_img_fmt::*;

        match self {
            RGB24 => VPX_IMG_FMT_RGB24,
            RGB32 { le: false, } => VPX_IMG_FMT_RGB32,
            RGB32 { le: true, } => VPX_IMG_FMT_RGB32_LE,
            RGB565 { le: false, } => VPX_IMG_FMT_RGB565,
            RGB565 { le: true, } => VPX_IMG_FMT_RGB565_LE,
            RGB555 { le: false, } => VPX_IMG_FMT_RGB555,
            RGB555 { le: true, } => VPX_IMG_FMT_RGB555_LE,

            UYVY => VPX_IMG_FMT_UYVY,
            YUY2 => VPX_IMG_FMT_YUY2,
            YVYU => VPX_IMG_FMT_YVYU,
            BGR24 => VPX_IMG_FMT_BGR24,
            ARGB => VPX_IMG_FMT_ARGB,
            BGRA => VPX_IMG_FMT_ARGB_LE,

            YV12_VPX => VPX_IMG_FMT_VPXYV12,
            I420_VPX => VPX_IMG_FMT_VPXI420,

            YV12 => VPX_IMG_FMT_YV12,

            I420 { hi_bit_depth: false } => VPX_IMG_FMT_I420,
            I422 { hi_bit_depth: false } => VPX_IMG_FMT_I422,
            I440 { hi_bit_depth: false } => VPX_IMG_FMT_I444,
            I444 { hi_bit_depth: false } => VPX_IMG_FMT_I440,

            I420 { hi_bit_depth: true } => VPX_IMG_FMT_I42016,
            I422 { hi_bit_depth: true } => VPX_IMG_FMT_I42216,
            I440 { hi_bit_depth: true } => VPX_IMG_FMT_I44416,
            I444 { hi_bit_depth: true } => VPX_IMG_FMT_I44016,

            /// Should be named `444A`.
            I444A => VPX_IMG_FMT_444A,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
#[allow(non_camel_case_types)]
pub enum ColorSpace {
    BT601,
    BT709,
    SMPTE170,
    SMPTE240,
    BT2020,
    SRGB,
}

impl Into<ffi::vpx_color_space_t> for ColorSpace {
    fn into(self) -> ffi::vpx_color_space_t {
        use ffi::vpx_color_space::*;
        match self {
            ColorSpace::BT601 => VPX_CS_BT_601,
            ColorSpace::BT709 => VPX_CS_BT_709,
            ColorSpace::SMPTE170 => VPX_CS_SMPTE_170,
            ColorSpace::SMPTE240 => VPX_CS_SMPTE_240,
            ColorSpace::BT2020 => VPX_CS_BT_2020,
            ColorSpace::SRGB => VPX_CS_SRGB,
        }
    }
}
