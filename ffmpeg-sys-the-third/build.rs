extern crate bindgen;
extern crate cc;
extern crate clang;
extern crate pkg_config;

use std::collections::HashMap;
use std::env;
use std::fmt::Write as FmtWrite;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;

use bindgen::callbacks::{
    EnumVariantCustomBehavior, EnumVariantValue, IntKind, MacroParsingBehavior, ParseCallbacks,
};

#[derive(Debug)]
struct Library {
    name: &'static str,
    optional: bool,
    features: &'static [AVFeature],
    headers: &'static [AVHeader],
}

impl Library {
    const fn required(
        name: &'static str,
        features: &'static [AVFeature],
        headers: &'static [AVHeader],
    ) -> Self {
        Self {
            name,
            optional: false,
            features,
            headers,
        }
    }

    const fn optional(
        name: &'static str,
        features: &'static [AVFeature],
        headers: &'static [AVHeader],
    ) -> Self {
        Self {
            name,
            optional: true,
            features,
            headers,
        }
    }

    fn lib_name(&self) -> String {
        format!("lib{}", self.name)
    }

    fn enabled(&self) -> bool {
        !self.optional || cargo_feature_enabled(self.name)
    }
}

static LIBRARIES: &[Library] = &[
    Library::required("avutil", AVUTIL_FEATURES, AVUTIL_HEADERS),
    Library::optional("avcodec", AVCODEC_FEATURES, AVCODEC_HEADERS),
    Library::optional("avformat", AVFORMAT_FEATURES, AVFORMAT_HEADERS),
    Library::optional("avdevice", AVDEVICE_FEATURES, AVDEVICE_HEADERS),
    Library::optional("avfilter", AVFILTER_FEATURES, AVFILTER_HEADERS),
    Library::optional("avresample", AVRESAMPLE_FEATURES, AVRESAMPLE_HEADERS),
    Library::optional("swscale", SWSCALE_FEATURES, SWSCALE_HEADERS),
    Library::optional("swresample", SWRESAMPLE_FEATURES, SWRESAMPLE_HEADERS),
    Library::optional("postproc", POSTPROC_FEATURES, POSTPROC_HEADERS),
];

#[derive(Debug)]
struct AVFeature {
    name: &'static str,
}

impl AVFeature {
    const fn new(name: &'static str) -> Self {
        Self { name }
    }
}

static AVUTIL_FEATURES: &[AVFeature] = &[
    AVFeature::new("OLD_AVOPTIONS"),
    AVFeature::new("PIX_FMT"),
    AVFeature::new("CONTEXT_SIZE"),
    AVFeature::new("PIX_FMT_DESC"),
    AVFeature::new("AV_REVERSE"),
    AVFeature::new("AUDIOCONVERT"),
    AVFeature::new("CPU_FLAG_MMX2"),
    AVFeature::new("LLS_PRIVATE"),
    AVFeature::new("AVFRAME_LAVC"),
    AVFeature::new("VDPAU"),
    AVFeature::new("GET_CHANNEL_LAYOUT_COMPAT"),
    AVFeature::new("XVMC"),
    AVFeature::new("OPT_TYPE_METADATA"),
    AVFeature::new("DLOG"),
    AVFeature::new("HMAC"),
    AVFeature::new("VAAPI"),
    AVFeature::new("PKT_PTS"),
    AVFeature::new("ERROR_FRAME"),
    AVFeature::new("FRAME_QP"),
    AVFeature::new("D2STR"),
    AVFeature::new("DECLARE_ALIGNED"),
    AVFeature::new("COLORSPACE_NAME"),
    AVFeature::new("AV_MALLOCZ_ARRAY"),
    AVFeature::new("FIFO_PEEK2"),
    AVFeature::new("FIFO_OLD_API"),
    AVFeature::new("OLD_CHANNEL_LAYOUT"),
    AVFeature::new("AV_FOPEN_UTF8"),
    AVFeature::new("PKT_DURATION"),
    AVFeature::new("REORDERED_OPAQUE"),
    AVFeature::new("FRAME_PICTURE_NUMBER"),
    AVFeature::new("HDR_VIVID_THREE_SPLINE"),
    AVFeature::new("FRAME_PKT"),
    AVFeature::new("INTERLACED_FRAME"),
    AVFeature::new("FRAME_KEY"),
    AVFeature::new("PALETTE_HAS_CHANGED"),
    AVFeature::new("VULKAN_CONTIGUOUS_MEMORY"),
    AVFeature::new("H274_FILM_GRAIN_VCS"),
];

static AVCODEC_FEATURES: &[AVFeature] = &[
    AVFeature::new("VIMA_DECODER"),
    AVFeature::new("REQUEST_CHANNELS"),
    AVFeature::new("OLD_DECODE_AUDIO"),
    AVFeature::new("OLD_ENCODE_AUDIO"),
    AVFeature::new("OLD_ENCODE_VIDEO"),
    AVFeature::new("CODEC_ID"),
    AVFeature::new("AUDIO_CONVERT"),
    AVFeature::new("AVCODEC_RESAMPLE"),
    AVFeature::new("DEINTERLACE"),
    AVFeature::new("DESTRUCT_PACKET"),
    AVFeature::new("GET_BUFFER"),
    AVFeature::new("MISSING_SAMPLE"),
    AVFeature::new("LOWRES"),
    AVFeature::new("CAP_VDPAU"),
    AVFeature::new("BUFS_VDPAU"),
    AVFeature::new("VOXWARE"),
    AVFeature::new("SET_DIMENSIONS"),
    AVFeature::new("DEBUG_MV"),
    AVFeature::new("AC_VLC"),
    AVFeature::new("OLD_MSMPEG4"),
    AVFeature::new("ASPECT_EXTENDED"),
    AVFeature::new("THREAD_OPAQUE"),
    AVFeature::new("CODEC_PKT"),
    AVFeature::new("ARCH_ALPHA"),
    AVFeature::new("ERROR_RATE"),
    AVFeature::new("QSCALE_TYPE"),
    AVFeature::new("MB_TYPE"),
    AVFeature::new("MAX_BFRAMES"),
    AVFeature::new("NEG_LINESIZES"),
    AVFeature::new("EMU_EDGE"),
    AVFeature::new("ARCH_SH4"),
    AVFeature::new("ARCH_SPARC"),
    AVFeature::new("UNUSED_MEMBERS"),
    AVFeature::new("IDCT_XVIDMMX"),
    AVFeature::new("INPUT_PRESERVED"),
    AVFeature::new("NORMALIZE_AQP"),
    AVFeature::new("GMC"),
    AVFeature::new("MV0"),
    AVFeature::new("CODEC_NAME"),
    AVFeature::new("AFD"),
    AVFeature::new("VISMV"),
    AVFeature::new("DV_FRAME_PROFILE"),
    AVFeature::new("AUDIOENC_DELAY"),
    AVFeature::new("VAAPI_CONTEXT"),
    AVFeature::new("AVCTX_TIMEBASE"),
    AVFeature::new("MPV_OPT"),
    AVFeature::new("STREAM_CODEC_TAG"),
    AVFeature::new("QUANT_BIAS"),
    AVFeature::new("RC_STRATEGY"),
    AVFeature::new("CODED_FRAME"),
    AVFeature::new("MOTION_EST"),
    AVFeature::new("WITHOUT_PREFIX"),
    AVFeature::new("CONVERGENCE_DURATION"),
    AVFeature::new("PRIVATE_OPT"),
    AVFeature::new("CODER_TYPE"),
    AVFeature::new("RTP_CALLBACK"),
    AVFeature::new("STAT_BITS"),
    AVFeature::new("VBV_DELAY"),
    AVFeature::new("SIDEDATA_ONLY_PKT"),
    AVFeature::new("AVPICTURE"),
    AVFeature::new("OPENH264_SLICE_MODE"),
    AVFeature::new("OPENH264_CABAC"),
    AVFeature::new("UNUSED_CODEC_CAPS"),
    AVFeature::new("THREAD_SAFE_CALLBACKS"),
    AVFeature::new("GET_FRAME_CLASS"),
    AVFeature::new("AUTO_THREADS"),
    AVFeature::new("INIT_PACKET"),
    AVFeature::new("FLAG_TRUNCATED"),
    AVFeature::new("SUB_TEXT_FORMAT"),
    AVFeature::new("IDCT_NONE"),
    AVFeature::new("SVTAV1_OPTS"),
    AVFeature::new("AYUV_CODECID"),
    AVFeature::new("VT_OUTPUT_CALLBACK"),
    AVFeature::new("AVCODEC_CHROMA_POS"),
    AVFeature::new("VT_HWACCEL_CONTEXT"),
    AVFeature::new("AVCTX_FRAME_NUMBER"),
    AVFeature::new("SLICE_OFFSET"),
    AVFeature::new("SUBFRAMES"),
    AVFeature::new("TICKS_PER_FRAME"),
    AVFeature::new("DROPCHANGED"),
    AVFeature::new("AVFFT"),
    AVFeature::new("FF_PROFILE_LEVEL"),
    AVFeature::new("AVCODEC_CLOSE"),
    AVFeature::new("BUFFER_MIN_SIZE"),
    AVFeature::new("VDPAU_ALLOC_GET_SET"),
];

static AVFORMAT_FEATURES: &[AVFeature] = &[
    AVFeature::new("LAVF_BITEXACT"),
    AVFeature::new("LAVF_FRAC"),
    AVFeature::new("URL_FEOF"),
    AVFeature::new("PROBESIZE_32"),
    AVFeature::new("LAVF_AVCTX"),
    AVFeature::new("OLD_OPEN_CALLBACKS"),
    AVFeature::new("LAVF_PRIV_OPT"),
    AVFeature::new("COMPUTE_PKT_FIELDS2"),
    AVFeature::new("AVIOCONTEXT_WRITTEN"),
    AVFeature::new("AVSTREAM_CLASS"),
    AVFeature::new("R_FRAME_RATE"),
    AVFeature::new("GET_END_PTS"),
    AVFeature::new("AVIODIRCONTEXT"),
    AVFeature::new("AVFORMAT_IO_CLOSE"),
    AVFeature::new("AVIO_WRITE_NONCONST"),
    AVFeature::new("LAVF_SHORTEST"),
    AVFeature::new("ALLOW_FLUSH"),
    AVFeature::new("AVSTREAM_SIDE_DATA"),
    AVFeature::new("GET_DUR_ESTIMATE_METHOD"),
];

static AVDEVICE_FEATURES: &[AVFeature] = &[
    AVFeature::new("DEVICE_CAPABILITIES"),
    AVFeature::new("BKTR_DEVICE"),
    AVFeature::new("OPENGL_DEVICE"),
    AVFeature::new("SDL2_DEVICE"),
];

static AVFILTER_FEATURES: &[AVFeature] = &[
    AVFeature::new("AVFILTERPAD_PUBLIC"),
    AVFeature::new("FOO_COUNT"),
    AVFeature::new("OLD_FILTER_OPTS"),
    AVFeature::new("OLD_FILTER_OPTS_ERROR"),
    AVFeature::new("AVFILTER_OPEN"),
    AVFeature::new("OLD_FILTER_REGISTER"),
    AVFeature::new("OLD_GRAPH_PARSE"),
    AVFeature::new("NOCONST_GET_NAME"),
    AVFeature::new("SWS_PARAM_OPTION"),
    AVFeature::new("BUFFERSINK_ALLOC"),
    AVFeature::new("PAD_COUNT"),
    AVFeature::new("LIBPLACEBO_OPTS"),
    AVFeature::new("LINK_PUBLIC"),
];

static AVRESAMPLE_FEATURES: &[AVFeature] = &[AVFeature::new("RESAMPLE_CLOSE_OPEN")];

static SWSCALE_FEATURES: &[AVFeature] =
    &[AVFeature::new("SWS_CPU_CAPS"), AVFeature::new("ARCH_BFIN")];

static SWRESAMPLE_FEATURES: &[AVFeature] = &[];

static POSTPROC_FEATURES: &[AVFeature] = &[];

#[derive(Debug)]
struct AVHeader {
    name: &'static str,
}

impl AVHeader {
    const fn new(name: &'static str) -> Self {
        Self { name }
    }
}

static AVUTIL_HEADERS: &[AVHeader] = &[
    AVHeader::new("adler32.h"),
    AVHeader::new("aes.h"),
    AVHeader::new("audio_fifo.h"),
    AVHeader::new("base64.h"),
    AVHeader::new("blowfish.h"),
    AVHeader::new("bprint.h"),
    AVHeader::new("buffer.h"),
    AVHeader::new("camellia.h"),
    AVHeader::new("cast5.h"),
    AVHeader::new("channel_layout.h"),
    AVHeader::new("cpu.h"),
    AVHeader::new("crc.h"),
    AVHeader::new("dict.h"),
    AVHeader::new("display.h"),
    AVHeader::new("downmix_info.h"),
    AVHeader::new("error.h"),
    AVHeader::new("eval.h"),
    AVHeader::new("fifo.h"),
    AVHeader::new("file.h"),
    AVHeader::new("frame.h"),
    AVHeader::new("hash.h"),
    AVHeader::new("hmac.h"),
    AVHeader::new("hwcontext.h"),
    AVHeader::new("imgutils.h"),
    AVHeader::new("lfg.h"),
    AVHeader::new("log.h"),
    AVHeader::new("lzo.h"),
    AVHeader::new("macros.h"),
    AVHeader::new("mathematics.h"),
    AVHeader::new("md5.h"),
    AVHeader::new("mem.h"),
    AVHeader::new("motion_vector.h"),
    AVHeader::new("murmur3.h"),
    AVHeader::new("opt.h"),
    AVHeader::new("parseutils.h"),
    AVHeader::new("pixdesc.h"),
    AVHeader::new("pixfmt.h"),
    AVHeader::new("random_seed.h"),
    AVHeader::new("rational.h"),
    AVHeader::new("replaygain.h"),
    AVHeader::new("ripemd.h"),
    AVHeader::new("samplefmt.h"),
    AVHeader::new("sha.h"),
    AVHeader::new("sha512.h"),
    AVHeader::new("stereo3d.h"),
    AVHeader::new("avstring.h"),
    AVHeader::new("threadmessage.h"),
    AVHeader::new("time.h"),
    AVHeader::new("timecode.h"),
    AVHeader::new("twofish.h"),
    AVHeader::new("avutil.h"),
    AVHeader::new("xtea.h"),
];
static AVCODEC_HEADERS: &[AVHeader] = &[
    AVHeader::new("avcodec.h"),
    AVHeader::new("dv_profile.h"),
    AVHeader::new("avfft.h"),
    AVHeader::new("vorbis_parser.h"),
];
static AVFORMAT_HEADERS: &[AVHeader] = &[AVHeader::new("avformat.h"), AVHeader::new("avio.h")];
static AVDEVICE_HEADERS: &[AVHeader] = &[AVHeader::new("avdevice.h")];
static AVFILTER_HEADERS: &[AVHeader] = &[
    AVHeader::new("buffersink.h"),
    AVHeader::new("buffersrc.h"),
    AVHeader::new("avfilter.h"),
];
static AVRESAMPLE_HEADERS: &[AVHeader] = &[AVHeader::new("avresample.h")];
static SWSCALE_HEADERS: &[AVHeader] = &[AVHeader::new("swscale.h")];
static SWRESAMPLE_HEADERS: &[AVHeader] = &[AVHeader::new("swresample.h")];
static POSTPROC_HEADERS: &[AVHeader] = &[AVHeader::new("postprocess.h")];

#[derive(Debug)]
struct Callbacks;

impl ParseCallbacks for Callbacks {
    fn int_macro(&self, name: &str, value: i64) -> Option<IntKind> {
        let ch_layout_prefix = "AV_CH_";
        let codec_cap_prefix = "AV_CODEC_CAP_";
        let codec_flag_prefix = "AV_CODEC_FLAG_";
        let error_max_size = "AV_ERROR_MAX_STRING_SIZE";

        if name.starts_with(ch_layout_prefix) {
            Some(IntKind::ULongLong)
        } else if (i32::MIN as i64..=i32::MAX as i64).contains(&value)
            && (name.starts_with(codec_cap_prefix) || name.starts_with(codec_flag_prefix))
        {
            Some(IntKind::UInt)
        } else if name == error_max_size {
            Some(IntKind::Custom {
                name: "usize",
                is_signed: false,
            })
        } else if (i32::MIN as i64..=i32::MAX as i64).contains(&value) {
            Some(IntKind::Int)
        } else {
            None
        }
    }

    fn enum_variant_behavior(
        &self,
        _enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: EnumVariantValue,
    ) -> Option<EnumVariantCustomBehavior> {
        let dummy_codec_id_prefix = "AV_CODEC_ID_FIRST_";
        if original_variant_name.starts_with(dummy_codec_id_prefix) {
            Some(EnumVariantCustomBehavior::Constify)
        } else {
            None
        }
    }

    // https://github.com/rust-lang/rust-bindgen/issues/687#issuecomment-388277405
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        use crate::MacroParsingBehavior::*;

        match name {
            "FP_INFINITE" => Ignore,
            "FP_NAN" => Ignore,
            "FP_NORMAL" => Ignore,
            "FP_SUBNORMAL" => Ignore,
            "FP_ZERO" => Ignore,
            _ => Default,
        }
    }
}

trait FFmpegConfigure {
    fn switch(&mut self, feature: &str, option_name: &str);
    fn enable(&mut self, feature: &str, option_name: &str);
}

impl FFmpegConfigure for Command {
    fn switch(&mut self, feature: &str, option_name: &str) {
        let arg = if cargo_feature_enabled(feature) {
            format!("--enable-{option_name}")
        } else {
            format!("--disable-{option_name}")
        };

        self.arg(arg);
    }

    fn enable(&mut self, feature: &str, option_name: &str) {
        if cargo_feature_enabled(feature) {
            self.arg(format!("--enable-{option_name}"));
        }
    }
}

fn cargo_feature_enabled(feature: &str) -> bool {
    env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_ok()
}

fn ffmpeg_version() -> String {
    env!("CARGO_PKG_VERSION")
        .split('+')
        .nth(1)
        .unwrap()
        .replace("ffmpeg-", "")
}

fn get_major_version(version_string: &str) -> u32 {
    version_string.split('.').next().unwrap().parse().unwrap()
}

fn output() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn fetch(source_dir: &Path, ffmpeg_version: &str) -> io::Result<()> {
    let _ = std::fs::remove_dir_all(source_dir);
    let status = Command::new("git")
        .arg("clone")
        .arg("--depth=1")
        .arg("-b")
        .arg(format!("n{ffmpeg_version}"))
        .arg("https://github.com/FFmpeg/FFmpeg")
        .arg(source_dir)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "fetch failed"))
    }
}

// left side: cargo feature name ("CARGO_FEATURE_BUILD_LIB_{}")
// right side: FFmpeg configure name ("--enable-{}")
static EXTERNAL_BUILD_LIBS: &[(&str, &str)] = &[
    // SSL
    ("GNUTLS", "gnutls"),
    ("OPENSSL", "openssl"),
    // Filters
    ("FONTCONFIG", "fontconfig"),
    ("FREI0R", "frei0r"),
    ("LADSPA", "ladspa"),
    ("ASS", "libass"),
    ("FREETYPE", "libfreetype"),
    ("FRIBIDI", "libfribidi"),
    ("OPENCV", "libopencv"),
    ("VMAF", "libvmaf"),
    // Encoders/decoders
    ("AACPLUS", "libaacplus"),
    ("CELT", "libcelt"),
    ("DCADEC", "libdcadec"),
    ("DAV1D", "libdav1d"),
    ("FAAC", "libfaac"),
    ("FDK_AAC", "libfdk-aac"),
    ("GSM", "libgsm"),
    ("ILBC", "libilbc"),
    ("VAZAAR", "libvazaar"),
    ("MP3LAME", "libmp3lame"),
    ("OPENCORE_AMRNB", "libopencore-amrnb"),
    ("OPENCORE_AMRWB", "libopencore-amrwb"),
    ("OPENH264", "libopenh264"),
    ("OPENH265", "libopenh265"),
    ("OPENJPEG", "libopenjpeg"),
    ("OPUS", "libopus"),
    ("SCHROEDINGER", "libschroedinger"),
    ("SHINE", "libshine"),
    ("SNAPPY", "libsnappy"),
    ("SPEEX", "libspeex"),
    ("STAGEFRIGHT_H264", "libstagefright-h264"),
    ("THEORA", "libtheora"),
    ("TWOLAME", "libtwolame"),
    ("UTVIDEO", "libutvideo"),
    ("VO_AACENC", "libvo-aacenc"),
    ("VO_AMRWBENC", "libvo-amrwbenc"),
    ("VORBIS", "libvorbis"),
    ("VPX", "libvpx"),
    ("WAVPACK", "libwavpack"),
    ("WEBP", "libwebp"),
    ("X264", "libx264"),
    ("X265", "libx265"),
    ("AVS", "libavs"),
    ("XVID", "libxvid"),
    // Protocols
    ("SMBCLIENT", "libsmbclient"),
    ("SSH", "libssh"),
];

fn build(out_dir: &Path, ffmpeg_version: &str) -> io::Result<PathBuf> {
    let source_dir = out_dir.join(format!("ffmpeg-{ffmpeg_version}"));
    let install_dir = out_dir.join("dist");
    if install_dir.join("lib").join("libavutil.a").exists() {
        rustc_link_extralibs(&source_dir);
        return Ok(install_dir);
    }

    fetch(&source_dir, ffmpeg_version)?;

    // Command's path is not relative to command's current_dir
    let configure_path = source_dir.join("configure");
    assert!(configure_path.exists());
    let mut configure = Command::new(&configure_path);
    configure.current_dir(&source_dir);

    configure.arg(format!("--prefix={}", install_dir.to_string_lossy()));

    if env::var("TARGET").unwrap() != env::var("HOST").unwrap() {
        // Rust targets are subtly different than naming scheme for compiler prefixes.
        // The cc crate has the messy logic of guessing a working prefix,
        // and this is a messy way of reusing that logic.
        let cc = cc::Build::new();
        let compiler = cc.get_compiler();
        let compiler = compiler.path().file_stem().unwrap().to_str().unwrap();
        let suffix_pos = compiler.rfind('-').unwrap(); // cut off "-gcc"
        let prefix = compiler[0..suffix_pos].trim_end_matches("-wr"); // "wr-c++" compiler

        configure.arg(format!("--cross-prefix={}-", prefix));
        configure.arg(format!(
            "--arch={}",
            env::var("CARGO_CFG_TARGET_ARCH").unwrap()
        ));
        configure.arg(format!(
            "--target_os={}",
            env::var("CARGO_CFG_TARGET_OS").unwrap()
        ));
    }

    // control debug build
    if env::var("DEBUG").is_ok() {
        configure.arg("--enable-debug");
        configure.arg("--disable-stripping");
    } else {
        configure.arg("--disable-debug");
        configure.arg("--enable-stripping");
    }

    // make it static
    configure.arg("--enable-static");
    configure.arg("--disable-shared");

    configure.arg("--enable-pic");

    // stop autodetected libraries enabling themselves, causing linking errors
    configure.arg("--disable-autodetect");

    // do not build programs since we don't need them
    configure.arg("--disable-programs");

    // the binary using ffmpeg-sys must comply with GPL
    configure.switch("BUILD_LICENSE_GPL", "gpl");

    // the binary using ffmpeg-sys must comply with (L)GPLv3
    configure.switch("BUILD_LICENSE_VERSION3", "version3");

    // the binary using ffmpeg-sys cannot be redistributed
    configure.switch("BUILD_LICENSE_NONFREE", "nonfree");

    let ffmpeg_major_version: u32 = get_major_version(ffmpeg_version);

    // configure building libraries based on features
    for lib in LIBRARIES
        .iter()
        .filter(|lib| lib.optional)
        .filter(|lib| !(lib.name == "avresample" && ffmpeg_major_version >= 5))
    {
        configure.switch(&lib.name.to_uppercase(), lib.name);
    }

    // configure external libraries based on features
    for (cargo_feat, option_name) in EXTERNAL_BUILD_LIBS {
        configure.enable(&format!("BUILD_LIB_{cargo_feat}"), option_name);
    }

    configure.enable("BUILD_DRM", "libdrm");
    configure.enable("BUILD_NVENC", "nvenc");
    // configure misc build options
    configure.enable("BUILD_PIC", "pic");

    // run ./configure
    let output = configure
        .output()
        .unwrap_or_else(|_| panic!("{:?} failed", configure));
    if !output.status.success() {
        println!("configure: {}", String::from_utf8_lossy(&output.stdout));

        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "configure failed {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }

    let num_jobs = if let Ok(cpus) = std::thread::available_parallelism() {
        cpus.to_string()
    } else {
        "1".to_string()
    };

    // run make
    if !Command::new("make")
        .arg(format!("-j{num_jobs}"))
        .current_dir(&source_dir)
        .status()?
        .success()
    {
        return Err(io::Error::new(io::ErrorKind::Other, "make failed"));
    }

    // run make install
    if !Command::new("make")
        .current_dir(&source_dir)
        .arg("install")
        .status()?
        .success()
    {
        return Err(io::Error::new(io::ErrorKind::Other, "make install failed"));
    }

    rustc_link_extralibs(&source_dir);
    Ok(install_dir)
}

fn rustc_link_extralibs(source_dir: &Path) {
    let config_mak = source_dir.join("ffbuild").join("config.mak");
    let file = File::open(config_mak).unwrap();
    let reader = BufReader::new(file);
    let extra_libs = reader
        .lines()
        .find(|line| line.as_ref().unwrap().starts_with("EXTRALIBS"))
        .map(|line| line.unwrap())
        .unwrap();

    let linker_args = extra_libs.split('=').last().unwrap().split(' ');
    let include_libs = linker_args
        .filter(|v| v.starts_with("-l"))
        .map(|flag| &flag[2..]);

    for lib in include_libs {
        println!("cargo:rustc-link-lib={lib}");
    }
}

#[cfg(not(target_env = "msvc"))]
fn try_vcpkg(_statik: bool) -> Option<Vec<PathBuf>> {
    None
}

#[cfg(target_env = "msvc")]
fn try_vcpkg(statik: bool) -> Option<Vec<PathBuf>> {
    if !statik {
        env::set_var("VCPKGRS_DYNAMIC", "1");
    }

    vcpkg::find_package("ffmpeg")
        .map_err(|e| {
            println!("Could not find ffmpeg with vcpkg: {}", e);
        })
        .map(|library| library.include_paths)
        .ok()
}

// add well known package manager lib paths us as homebrew (or macports)
#[cfg(target_os = "macos")]
fn add_pkg_config_path() {
    let pc_path = pkg_config::get_variable("pkg-config", "pc_path").unwrap();
    // append M1 homebrew pkgconfig path
    let brew_pkgconfig = cfg!(target_arch = "aarch64")
        .then_some("/opt/homebrew/lib/pkgconfig/")
        .unwrap_or("/usr/local/homebrew/lib/pkgconfig/"); // x86 as fallback
    if !pc_path.to_lowercase().contains(brew_pkgconfig) && Path::new(brew_pkgconfig).is_dir() {
        let new_pc_path = env::var("PKG_CONFIG_PATH")
            // PKG_CONFIG_PATH="/our/path:$PKG_CONFIG_PATH"
            .map(|p| format!("{brew_pkgconfig}:{p}"))
            .unwrap_or_else(|_| brew_pkgconfig.to_string());
        env::set_var("PKG_CONFIG_PATH", new_pc_path);
    }
}
#[cfg(not(target_os = "macos"))]
fn add_pkg_config_path() {}

fn check_features(include_paths: &[PathBuf]) {
    let clang = clang::Clang::new().expect("Cannot find clang");
    let index = clang::Index::new(&clang, false, false);

    println!("loaded clang version: {}", clang::get_version());

    let enabled_libraries = || LIBRARIES.iter().filter(|lib| lib.enabled());

    let mut code = String::new();
    for lib in enabled_libraries() {
        let _ = writeln!(code, "#include <lib{}/{}.h>", lib.name, lib.name);
    }

    let mut features_defined_enabled = enabled_libraries()
        .flat_map(|lib| lib.features)
        .map(|feature| {
            let feature_name = format!("FF_API_{}", feature.name);
            let _ = writeln!(code, "#ifdef {feature_name}");
            let _ = writeln!(
                code,
                "    int {} = {feature_name};",
                feature_name.to_lowercase()
            );
            let _ = writeln!(code, "#endif");
            (feature_name.to_lowercase(), (false, false))
        })
        .collect::<HashMap<_, _>>();

    let mut versions = enabled_libraries()
        .map(|lib| (lib.name, (0, 0)))
        .collect::<HashMap<_, _>>();

    let include_args = include_paths
        .iter()
        .map(|path| format!("-I{}", path.to_string_lossy()))
        .collect::<Vec<_>>();

    let tu = index
        .parser("check.c")
        .arguments(&include_args)
        .detailed_preprocessing_record(true)
        .unsaved(&[clang::Unsaved::new("check.c", &code)])
        .parse()
        .expect("Unable to parse generated file");

    tu.get_entity().visit_children(|entity, _parent| {
        if let Some(name) = entity.get_name() {
            if entity.get_kind() == clang::EntityKind::VarDecl && name.starts_with("ff_api") {
                if let Some(clang::EvaluationResult::SignedInteger(value)) = entity.evaluate() {
                    if let Some(val) = features_defined_enabled.get_mut(&name) {
                        *val = (true, value != 0);
                    }
                }
            }
        }
        clang::EntityVisitResult::Continue
    });

    for def in clang::sonar::find_definitions(tu.get_entity().get_children()) {
        if let clang::sonar::DefinitionValue::Integer(_, value) = def.value {
            if let Some(name) = def.name.strip_prefix("LIB") {
                if let Some(name) = name.strip_suffix("_VERSION_MAJOR") {
                    if let Some(ver) = versions.get_mut(name.to_lowercase().as_str()) {
                        ver.0 = value;
                    }
                } else if let Some(name) = name.strip_suffix("_VERSION_MINOR") {
                    if let Some(ver) = versions.get_mut(name.to_lowercase().as_str()) {
                        ver.1 = value;
                    }
                }
            }
        }
    }

    for (var, (var_defined, var_enabled)) in features_defined_enabled {
        // Every possible feature needs an unconditional check-cfg to prevent warnings
        println!(r#"cargo:rustc-check-cfg=cfg(feature, values("{}"))"#, var);
        println!(r#"cargo:check_{}=true"#, var);

        if var_enabled {
            println!(r#"cargo:rustc-cfg=feature="{}""#, var);
            println!(r#"cargo:{}=true"#, var);
        }

        // Also find out if defined or not (useful for cases where only the definition of a macro
        // can be used as distinction)
        if var_defined {
            println!(r#"cargo:rustc-cfg=feature="{}_is_defined""#, var);
            println!(r#"cargo:{}_is_defined=true"#, var);
        }
    }

    let version_check_info = [("avcodec", 56, 62, 0, 108)];
    for &(lib, begin_version_major, end_version_major, begin_version_minor, end_version_minor) in
        &version_check_info
    {
        let libversion = *versions
            .get(lib)
            .expect("Unable to find the version for lib{lib}");

        for version_major in begin_version_major..end_version_major {
            for version_minor in begin_version_minor..end_version_minor {
                if libversion >= (version_major, version_minor) {
                    println!(
                        r#"cargo:rustc-cfg=feature="{lib}_version_greater_than_{version_major}_{version_minor}""#
                    );
                    println!(
                        r#"cargo:{lib}_version_greater_than_{version_major}_{version_minor}=true"#
                    );
                }
            }
        }
    }

    let ffmpeg_lavc_versions = [
        ("ffmpeg_3_0", 57, 24),
        ("ffmpeg_3_1", 57, 48),
        ("ffmpeg_3_2", 57, 64),
        ("ffmpeg_3_3", 57, 89),
        ("ffmpeg_3_1", 57, 107),
        ("ffmpeg_4_0", 58, 18),
        ("ffmpeg_4_1", 58, 35),
        ("ffmpeg_4_2", 58, 54),
        ("ffmpeg_4_3", 58, 91),
        ("ffmpeg_4_4", 58, 100),
        ("ffmpeg_5_0", 59, 18),
        ("ffmpeg_5_1", 59, 37),
        ("ffmpeg_6_0", 60, 3),
        ("ffmpeg_6_1", 60, 31),
        ("ffmpeg_7_0", 61, 3),
    ];

    let lavc_version = *versions
        .get("avcodec")
        .expect("Unable to find the version for lib{lib}");

    for &(ffmpeg_version_flag, lavc_version_major, lavc_version_minor) in &ffmpeg_lavc_versions {
        // Every possible feature needs an unconditional check-cfg to prevent warnings
        println!(
            r#"cargo:rustc-check-cfg=cfg(feature, values("{}"))"#,
            ffmpeg_version_flag
        );
        println!(r#"cargo:check_{}=true"#, ffmpeg_version_flag);

        if lavc_version >= (lavc_version_major, lavc_version_minor) {
            println!(r#"cargo:rustc-cfg=feature="{}""#, ffmpeg_version_flag);
            println!(r#"cargo:{}=true"#, ffmpeg_version_flag);
        }
    }
}

fn search_include(include_paths: &[PathBuf], header: &str) -> String {
    for dir in include_paths {
        let include = dir.join(header);
        if fs::metadata(&include).is_ok() {
            return include.as_path().to_str().unwrap().to_string();
        }
    }
    format!("/usr/include/{}", header)
}

fn maybe_search_include(include_paths: &[PathBuf], header: &str) -> Option<String> {
    let path = search_include(include_paths, header);
    if fs::metadata(&path).is_ok() {
        Some(path)
    } else {
        None
    }
}

fn link_to_libraries(statik: bool) {
    let ffmpeg_ty = if statik { "static" } else { "dylib" };
    for lib in LIBRARIES.iter().filter(|lib| lib.enabled()) {
        println!("cargo:rustc-link-lib={}={}", ffmpeg_ty, lib.name);
    }
    if cargo_feature_enabled("build_zlib") && cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=z");
    }
}

fn main() {
    let out_dir = output();
    let statik = cargo_feature_enabled("static");
    let ffmpeg_version = ffmpeg_version();
    let ffmpeg_major_version: u32 = get_major_version(&ffmpeg_version);

    let include_paths: Vec<PathBuf> = if cargo_feature_enabled("build") {
        let install_dir = build(&out_dir, &ffmpeg_version).unwrap();
        println!(
            "cargo:rustc-link-search=native={}",
            install_dir.join("lib").to_string_lossy()
        );
        link_to_libraries(statik);

        vec![install_dir.join("include")]
    }
    // Use prebuilt library
    else if let Ok(ffmpeg_dir) = env::var("FFMPEG_DIR") {
        let ffmpeg_dir = PathBuf::from(ffmpeg_dir);
        println!(
            "cargo:rustc-link-search=native={}",
            ffmpeg_dir.join("lib").to_string_lossy()
        );
        link_to_libraries(statik);
        vec![ffmpeg_dir.join("include")]
    } else if let Some(paths) = try_vcpkg(statik) {
        // vcpkg doesn't detect the "system" dependencies
        if statik {
            if cfg!(feature = "avcodec") || cfg!(feature = "avdevice") {
                println!("cargo:rustc-link-lib=ole32");
            }

            if cfg!(feature = "avformat") {
                println!("cargo:rustc-link-lib=secur32");
                println!("cargo:rustc-link-lib=ws2_32");
            }

            // avutil depdendencies
            println!("cargo:rustc-link-lib=bcrypt");
            println!("cargo:rustc-link-lib=user32");
        }

        paths
    } else {
        // Fallback to pkg-config
        add_pkg_config_path();
        let mut pkgconfig = pkg_config::Config::new();
        pkgconfig.statik(statik);

        for lib in LIBRARIES.iter().filter(|lib| lib.enabled()) {
            let _ = pkgconfig.probe(&lib.lib_name()).unwrap();
        }

        pkgconfig.probe("libavcodec").unwrap().include_paths
    };

    if statik && cfg!(target_os = "macos") {
        let frameworks = vec![
            "AppKit",
            "AudioToolbox",
            "AVFoundation",
            "CoreFoundation",
            "CoreGraphics",
            "CoreMedia",
            "CoreServices",
            "CoreVideo",
            "Foundation",
            "OpenCL",
            "OpenGL",
            "QTKit",
            "QuartzCore",
            "Security",
            "VideoDecodeAcceleration",
            "VideoToolbox",
        ];
        for f in frameworks {
            println!("cargo:rustc-link-lib=framework={}", f);
        }
    }

    check_features(&include_paths);

    let clang_includes = include_paths
        .iter()
        .map(|include| format!("-I{}", include.to_string_lossy()));

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut builder = bindgen::Builder::default()
        .clang_args(clang_includes)
        .ctypes_prefix("libc")
        // Not trivially copyable
        .no_copy("AVChannelLayout")
        // We need/want to implement Debug by hand for some types
        .no_debug("AVChannelLayout")
        .no_debug("AVChannelCustom")
        // In FFmpeg 7.0+, this has bitfield-like behaviour,
        // so cannot be a "rustified" enum
        .newtype_enum("AVOptionType")
        .allowlist_file(r#".*[/\\]libavutil[/\\].*"#)
        .allowlist_file(r#".*[/\\]libavcodec[/\\].*"#)
        .allowlist_file(r#".*[/\\]libavformat[/\\].*"#)
        .allowlist_file(r#".*[/\\]libavdevice[/\\].*"#)
        .allowlist_file(r#".*[/\\]libavfilter[/\\].*"#)
        .allowlist_file(r#".*[/\\]libavresample[/\\].*"#)
        .allowlist_file(r#".*[/\\]libswscale[/\\].*"#)
        .allowlist_file(r#".*[/\\]libswresample[/\\].*"#)
        .allowlist_file(r#".*[/\\]libpostproc[/\\].*"#)
        .opaque_type("__mingw_ldbl_type_t")
        .prepend_enum_name(false)
        .derive_eq(true)
        .size_t_is_usize(true)
        .parse_callbacks(Box::new(Callbacks));

    if cargo_feature_enabled("non_exhaustive_enums") {
        builder = builder.rustified_non_exhaustive_enum(".*");
    } else {
        builder = builder.rustified_enum(".*");
    }

    // The input headers we would like to generate
    // bindings for.
    for lib in LIBRARIES.iter().filter(|lib| lib.enabled()) {
        for header in lib.headers {
            builder = builder.header(search_include(
                &include_paths,
                &format!("lib{}/{}", lib.name, header.name),
            ));
        }
    }

    if cargo_feature_enabled("avcodec") && ffmpeg_major_version < 5 {
        builder = builder.header(search_include(&include_paths, "libavcodec/vaapi.h"))
    }

    if let Some(hwcontext_drm_header) =
        maybe_search_include(&include_paths, "libavutil/hwcontext_drm.h")
    {
        builder = builder.header(hwcontext_drm_header);
    }

    // Finish the builder and generate the bindings.
    let bindings = builder
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
