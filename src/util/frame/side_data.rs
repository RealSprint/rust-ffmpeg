use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem::transmute;
use std::slice;
use std::str::from_utf8_unchecked;

use super::Frame;
use ffi::AVFrameSideDataType::*;
use ffi::*;
use DictionaryRef;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Type {
    PanScan,
    A53CC,
    Stereo3D,
    MatrixEncoding,
    DownMixInfo,
    ReplayGain,
    DisplayMatrix,
    AFD,
    MotionVectors,
    SkipSamples,
    AudioServiceType,
    MasteringDisplayMetadata,
    GOPTimecode,
    Spherical,

    ContentLightLevel,
    IccProfile,

    #[cfg(feature = "ffmpeg_4_0")]
    QPTableProperties,
    #[cfg(feature = "ffmpeg_4_0")]
    QPTableData,

    #[cfg(feature = "ffmpeg_4_1")]
    S12M_TIMECODE,

    #[cfg(feature = "ffmpeg_4_2")]
    DYNAMIC_HDR_PLUS,
    #[cfg(feature = "ffmpeg_4_2")]
    REGIONS_OF_INTEREST,

    #[cfg(feature = "ffmpeg_4_3")]
    VIDEO_ENC_PARAMS,

    #[cfg(feature = "ni")]
    NETINT_UDU_SEI,
    #[cfg(feature = "ni")]
    NETINT_CUSTOM_SEI,
    #[cfg(feature = "ni")]
    NETINT_BITRATE,
    #[cfg(feature = "ni")]
    NETINT_LONG_TERM_REF,

    OTHER(i32),
}

impl Type {
    #[inline]
    pub fn name(&self) -> &'static str {
        unsafe {
            from_utf8_unchecked(CStr::from_ptr(av_frame_side_data_name((*self).into())).to_bytes())
        }
    }
}

impl From<AVFrameSideDataType> for Type {
    #[inline(always)]
    fn from(value: AVFrameSideDataType) -> Self {
        match value {
            AV_FRAME_DATA_PANSCAN => Type::PanScan,
            AV_FRAME_DATA_A53_CC => Type::A53CC,
            AV_FRAME_DATA_STEREO3D => Type::Stereo3D,
            AV_FRAME_DATA_MATRIXENCODING => Type::MatrixEncoding,
            AV_FRAME_DATA_DOWNMIX_INFO => Type::DownMixInfo,
            AV_FRAME_DATA_REPLAYGAIN => Type::ReplayGain,
            AV_FRAME_DATA_DISPLAYMATRIX => Type::DisplayMatrix,
            AV_FRAME_DATA_AFD => Type::AFD,
            AV_FRAME_DATA_MOTION_VECTORS => Type::MotionVectors,
            AV_FRAME_DATA_SKIP_SAMPLES => Type::SkipSamples,
            AV_FRAME_DATA_AUDIO_SERVICE_TYPE => Type::AudioServiceType,
            AV_FRAME_DATA_MASTERING_DISPLAY_METADATA => Type::MasteringDisplayMetadata,
            AV_FRAME_DATA_GOP_TIMECODE => Type::GOPTimecode,
            AV_FRAME_DATA_SPHERICAL => Type::Spherical,

            AV_FRAME_DATA_CONTENT_LIGHT_LEVEL => Type::ContentLightLevel,
            AV_FRAME_DATA_ICC_PROFILE => Type::IccProfile,

            #[cfg(feature = "ffmpeg_4_0")]
            AV_FRAME_DATA_QP_TABLE_PROPERTIES => Type::QPTableProperties,
            #[cfg(feature = "ffmpeg_4_0")]
            AV_FRAME_DATA_QP_TABLE_DATA => Type::QPTableData,

            #[cfg(feature = "ffmpeg_4_1")]
            AV_FRAME_DATA_S12M_TIMECODE => Type::S12M_TIMECODE,

            #[cfg(feature = "ffmpeg_4_2")]
            AV_FRAME_DATA_DYNAMIC_HDR_PLUS => Type::DYNAMIC_HDR_PLUS,
            #[cfg(feature = "ffmpeg_4_2")]
            AV_FRAME_DATA_REGIONS_OF_INTEREST => Type::REGIONS_OF_INTEREST,

            #[cfg(feature = "ffmpeg_4_3")]
            AV_FRAME_DATA_VIDEO_ENC_PARAMS => Type::VIDEO_ENC_PARAMS,

            #[cfg(feature = "ni")]
            AV_FRAME_DATA_NETINT_UDU_SEI => Type::NETINT_UDU_SEI,
            #[cfg(feature = "ni")]
            AV_FRAME_DATA_NETINT_CUSTOM_SEI => Type::NETINT_CUSTOM_SEI,
            #[cfg(feature = "ni")]
            AV_FRAME_DATA_NETINT_BITRATE => Type::NETINT_BITRATE,
            #[cfg(feature = "ni")]
            AV_FRAME_DATA_NETINT_LONG_TERM_REF => Type::NETINT_LONG_TERM_REF,

            value => Type::OTHER(value as i32),
        }
    }
}

impl From<Type> for AVFrameSideDataType {
    #[inline(always)]
    fn from(val: Type) -> Self {
        match val {
            Type::PanScan => AV_FRAME_DATA_PANSCAN,
            Type::A53CC => AV_FRAME_DATA_A53_CC,
            Type::Stereo3D => AV_FRAME_DATA_STEREO3D,
            Type::MatrixEncoding => AV_FRAME_DATA_MATRIXENCODING,
            Type::DownMixInfo => AV_FRAME_DATA_DOWNMIX_INFO,
            Type::ReplayGain => AV_FRAME_DATA_REPLAYGAIN,
            Type::DisplayMatrix => AV_FRAME_DATA_DISPLAYMATRIX,
            Type::AFD => AV_FRAME_DATA_AFD,
            Type::MotionVectors => AV_FRAME_DATA_MOTION_VECTORS,
            Type::SkipSamples => AV_FRAME_DATA_SKIP_SAMPLES,
            Type::AudioServiceType => AV_FRAME_DATA_AUDIO_SERVICE_TYPE,
            Type::MasteringDisplayMetadata => AV_FRAME_DATA_MASTERING_DISPLAY_METADATA,
            Type::GOPTimecode => AV_FRAME_DATA_GOP_TIMECODE,
            Type::Spherical => AV_FRAME_DATA_SPHERICAL,

            Type::ContentLightLevel => AV_FRAME_DATA_CONTENT_LIGHT_LEVEL,
            Type::IccProfile => AV_FRAME_DATA_ICC_PROFILE,

            #[cfg(feature = "ffmpeg_4_0")]
            Type::QPTableProperties => AV_FRAME_DATA_QP_TABLE_PROPERTIES,
            #[cfg(feature = "ffmpeg_4_0")]
            Type::QPTableData => AV_FRAME_DATA_QP_TABLE_DATA,

            #[cfg(feature = "ffmpeg_4_1")]
            Type::S12M_TIMECODE => AV_FRAME_DATA_S12M_TIMECODE,

            #[cfg(feature = "ffmpeg_4_2")]
            Type::DYNAMIC_HDR_PLUS => AV_FRAME_DATA_DYNAMIC_HDR_PLUS,
            #[cfg(feature = "ffmpeg_4_2")]
            Type::REGIONS_OF_INTEREST => AV_FRAME_DATA_REGIONS_OF_INTEREST,

            #[cfg(feature = "ffmpeg_4_3")]
            Type::VIDEO_ENC_PARAMS => AV_FRAME_DATA_VIDEO_ENC_PARAMS,

            #[cfg(feature = "ni")]
            Type::NETINT_UDU_SEI => AV_FRAME_DATA_NETINT_UDU_SEI,
            #[cfg(feature = "ni")]
            Type::NETINT_CUSTOM_SEI => AV_FRAME_DATA_NETINT_CUSTOM_SEI,
            #[cfg(feature = "ni")]
            Type::NETINT_BITRATE => AV_FRAME_DATA_NETINT_BITRATE,
            #[cfg(feature = "ni")]
            Type::NETINT_LONG_TERM_REF => AV_FRAME_DATA_NETINT_LONG_TERM_REF,

            Type::OTHER(value) => unsafe { transmute(value) },
        }
    }
}

pub struct SideData<'a> {
    ptr: *mut AVFrameSideData,

    _marker: PhantomData<&'a Frame>,
}

impl<'a> SideData<'a> {
    #[inline(always)]
    pub unsafe fn wrap(ptr: *mut AVFrameSideData) -> Self {
        SideData {
            ptr,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub unsafe fn as_ptr(&self) -> *const AVFrameSideData {
        self.ptr as *const _
    }

    #[inline(always)]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVFrameSideData {
        self.ptr
    }
}

impl<'a> SideData<'a> {
    #[inline]
    pub fn kind(&self) -> Type {
        unsafe { Type::from((*self.as_ptr()).type_) }
    }

    #[inline]
    pub fn data(&self) -> &[u8] {
        unsafe { slice::from_raw_parts((*self.as_ptr()).data, (*self.as_ptr()).size as usize) }
    }

    #[inline]
    pub fn metadata(&self) -> DictionaryRef {
        unsafe { DictionaryRef::wrap((*self.as_ptr()).metadata) }
    }
}
