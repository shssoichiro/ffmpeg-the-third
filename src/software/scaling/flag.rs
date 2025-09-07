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
        const FAST_BILINEAR        = SF::SWS_FAST_BILINEAR as c_int;
        const BILINEAR             = SF::SWS_BILINEAR as c_int;
        const BICUBIC              = SF::SWS_BICUBIC as c_int;
        const X                    = SF::SWS_X as c_int;
        const POINT                = SF::SWS_POINT as c_int;
        const AREA                 = SF::SWS_AREA as c_int;
        const BICUBLIN             = SF::SWS_BICUBLIN as c_int;
        const GAUSS                = SF::SWS_GAUSS as c_int;
        const SINC                 = SF::SWS_SINC as c_int;
        const LANCZOS              = SF::SWS_LANCZOS as c_int;
        const SPLINE               = SF::SWS_SPLINE as c_int;
        const SRC_V_CHR_DROP_MASK  = SWS_SRC_V_CHR_DROP_MASK;
        const SRC_V_CHR_DROP_SHIFT = SWS_SRC_V_CHR_DROP_SHIFT;
        const PARAM_DEFAULT        = SWS_PARAM_DEFAULT;
        const PRINT_INFO           = SF::SWS_PRINT_INFO as c_int;
        const FULL_CHR_H_INT       = SF::SWS_FULL_CHR_H_INT as c_int;
        const FULL_CHR_H_INP       = SF::SWS_FULL_CHR_H_INP as c_int;
        const DIRECT_BGR           = SF::SWS_DIRECT_BGR as c_int;
        const ACCURATE_RND         = SF::SWS_ACCURATE_RND as c_int;
        const BITEXACT             = SF::SWS_BITEXACT as c_int;
        const ERROR_DIFFUSION      = SF::SWS_ERROR_DIFFUSION as c_int;
    }
}
