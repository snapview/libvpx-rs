use std::ops::DerefMut;
use std::ptr::null_mut;
use std::slice;

use ffi;

use context::CodecContext as Context;

pub struct Frame<'encoder> {
    data: &'encoder [u8],
    pts: ffi::vpx_codec_pts_t,
    duration: u64,
    flags: ffi::vpx_codec_frame_flags_t,
    partition_id: i32,
}

impl<'encoder> Frame<'encoder> {
    pub fn data(&self) -> &'encoder [u8] {
        self.data
    }

    pub fn is_keyframe(&self) -> bool {
        self.flags & ffi::VPX_FRAME_IS_KEY != 0
    }

    pub fn is_droppable(&self) -> bool {
        self.flags & ffi::VPX_FRAME_IS_DROPPABLE != 0
    }

    pub fn is_invisible(&self) -> bool {
        self.flags & ffi::VPX_FRAME_IS_INVISIBLE != 0
    }

    pub fn is_fragment(&self) -> bool {
        self.flags & ffi::VPX_FRAME_IS_FRAGMENT != 0
    }

    pub fn pts(&self) -> ffi::vpx_codec_pts_t {
        self.pts
    }

    pub fn duration(&self) -> u64 {
        self.duration
    }

    pub fn partition(&self) -> i32 {
        self.partition_id
    }
}

pub enum PacketKind<'encoder> {
    EncodedFrame(Frame<'encoder>),
    Stats,
    FPMB,
    PSNR,
    Unknown,
}

pub struct FramesIter<'encoder> {
    inner: ffi::vpx_codec_iter_t,
    codec_ctx: &'encoder mut Context,
}

impl<'encoder> FramesIter<'encoder> {
    pub fn new(ctx: &'encoder mut Context) -> Self {
        FramesIter {
            inner: null_mut(),
            codec_ctx: ctx,
        }
    }
}

impl<'encoder> Iterator for FramesIter<'encoder> {
    type Item = PacketKind<'encoder>;

    fn next(&mut self) -> Option<Self::Item> {
        let pkt = unsafe { ffi::vpx_codec_get_cx_data(self.codec_ctx.deref_mut(), &mut self.inner) };
        unsafe { pkt.as_ref().map(|pkt_ref| {
            match pkt_ref.kind {
                ffi::vpx_codec_cx_pkt_kind::VPX_CODEC_CX_FRAME_PKT => {
                    let frame = pkt_ref.data.frame.as_ref();
                    PacketKind::EncodedFrame(Frame {
                        data: slice::from_raw_parts(frame.buf as *mut u8, frame.sz as usize),
                        pts: frame.pts,
                        duration: frame.duration,
                        flags: frame.flags,
                        partition_id: frame.partition_id,
                    })
                },
                ffi::vpx_codec_cx_pkt_kind::VPX_CODEC_STATS_PKT => {
                    // TODO: implement an appopriate structure to hold the data when it's needed
                    PacketKind::Stats
                },
                ffi::vpx_codec_cx_pkt_kind::VPX_CODEC_FPMB_STATS_PKT => {
                    // TODO: implement an appopriate structure to hold the data when it's needed
                    PacketKind::FPMB
                },
                ffi::vpx_codec_cx_pkt_kind::VPX_CODEC_PSNR_PKT => {
                    // TODO: implement an appopriate structure to hold the data
                    PacketKind::PSNR
                },
                _ => {
                    PacketKind::Unknown
                },
            }
        })
    } }
}
