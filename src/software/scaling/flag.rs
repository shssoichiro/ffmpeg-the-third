#[cfg(not(feature = "ffmpeg_8_0"))]
use crate::ffi::*;
use libc::c_int;

#[cfg(not(feature = "ffmpeg_8_0"))]
bitflags::bitflags! {
    pub struct Flags: c_int {
        const FAST_BILINEAR        = SWS_FAST_BILINEAR;
        const BILINEAR             = SWS_BILINEAR;
        const BICUBIC              = SWS_BICUBIC;
        const X                    = SWS_X;
        const POINT                = SWS_POINT;
        const AREA                 = SWS_AREA;
        const BICUBLIN             = SWS_BICUBLIN;
        const GAUSS                = SWS_GAUSS;
        const SINC                 = SWS_SINC;
        const LANCZOS              = SWS_LANCZOS;
        const SPLINE               = SWS_SPLINE;
        const SRC_V_CHR_DROP_MASK  = SWS_SRC_V_CHR_DROP_MASK;
        const SRC_V_CHR_DROP_SHIFT = SWS_SRC_V_CHR_DROP_SHIFT;
        const PARAM_DEFAULT        = SWS_PARAM_DEFAULT;
        const PRINT_INFO           = SWS_PRINT_INFO;
        const FULL_CHR_H_INT       = SWS_FULL_CHR_H_INT;
        const FULL_CHR_H_INP       = SWS_FULL_CHR_H_INP;
        const DIRECT_BGR           = SWS_DIRECT_BGR;
        const ACCURATE_RND         = SWS_ACCURATE_RND;
        const BITEXACT             = SWS_BITEXACT;
        const ERROR_DIFFUSION      = SWS_ERROR_DIFFUSION;
    }
}

#[cfg(feature = "ffmpeg_8_0")]
use crate::ffi::SwsFlags as SF;

#[cfg(feature = "ffmpeg_8_0")]
bitflags::bitflags! {
    pub struct Flags: c_int {
        const FAST_BILINEAR        = SF::FAST_BILINEAR.0 as _;
        const BILINEAR             = SF::BILINEAR.0 as _;
        const BICUBIC              = SF::BICUBIC.0 as _;
        const X                    = SF::X.0 as _;
        const POINT                = SF::POINT.0 as _;
        const AREA                 = SF::AREA.0 as _;
        const BICUBLIN             = SF::BICUBLIN.0 as _;
        const GAUSS                = SF::GAUSS.0 as _;
        const SINC                 = SF::SINC.0 as _;
        const LANCZOS              = SF::LANCZOS.0 as _;
        const SPLINE               = SF::SPLINE.0 as _;
        const PRINT_INFO           = SF::PRINT_INFO.0 as _;
        const FULL_CHR_H_INT       = SF::FULL_CHR_H_INT.0 as _;
        const FULL_CHR_H_INP       = SF::FULL_CHR_H_INP.0 as _;
        const DIRECT_BGR           = SF::DIRECT_BGR.0 as _;
        const ACCURATE_RND         = SF::ACCURATE_RND.0 as _;
        const BITEXACT             = SF::BITEXACT.0 as _;
        const ERROR_DIFFUSION      = SF::ERROR_DIFFUSION.0 as _;
    }
}
