use std::collections::HashMap;
use std::env;
use std::fmt::Write as FmtWrite;
use std::path::PathBuf;
use std::str;

use bindgen::callbacks::{
    EnumVariantCustomBehavior, EnumVariantValue, IntKind, MacroParsingBehavior, ParseCallbacks,
};
use bindgen::EnumVariation;

#[path = "build/compile.rs"]
mod compile;

#[derive(Debug)]
struct Library {
    name: &'static str,
    optional: bool,
    features: &'static [AVFeature],
    headers: &'static [AVHeader],
    min_major_version: u64,
}

impl Library {
    const fn required(
        name: &'static str,
        features: &'static [AVFeature],
        headers: &'static [AVHeader],
        min_version: u64,
    ) -> Self {
        Self {
            name,
            optional: false,
            features,
            headers,
            min_major_version: min_version,
        }
    }

    const fn optional(
        name: &'static str,
        features: &'static [AVFeature],
        headers: &'static [AVHeader],
        min_version: u64,
    ) -> Self {
        Self {
            name,
            optional: true,
            features,
            headers,
            min_major_version: min_version,
        }
    }

    fn lib_name(&self) -> String {
        format!("lib{}", self.name)
    }

    fn enabled(&self) -> bool {
        !self.optional || cargo_feature_enabled(self.name)
    }
}

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
    // before 6.0 (< v58)
    AVFeature::new("D2STR"),
    AVFeature::new("DECLARE_ALIGNED"),
    AVFeature::new("COLORSPACE_NAME"),
    AVFeature::new("AV_MALLOCZ_ARRAY"),
    // before 7.0 (< v59)
    AVFeature::new("XVMC"),
    AVFeature::new("FIFO_PEEK2"),
    AVFeature::new("FIFO_OLD_API"),
    AVFeature::new("OLD_CHANNEL_LAYOUT"),
    AVFeature::new("AV_FOPEN_UTF8"),
    AVFeature::new("PKT_DURATION"),
    AVFeature::new("REORDERED_OPAQUE"),
    AVFeature::new("FRAME_PICTURE_NUMBER"),
    // before 8.0 (< v60)
    AVFeature::new("HDR_VIVID_THREE_SPLINE"),
    AVFeature::new("FRAME_PKT"),
    AVFeature::new("INTERLACED_FRAME"),
    AVFeature::new("FRAME_KEY"),
    AVFeature::new("PALETTE_HAS_CHANGED"),
    AVFeature::new("VULKAN_CONTIGUOUS_MEMORY"),
    AVFeature::new("H274_FILM_GRAIN_VCS"),
    // before 9.0 (< v61)
    AVFeature::new("MOD_UINTP2"),
    AVFeature::new("RISCV_FD_ZBA"),
    AVFeature::new("VULKAN_FIXED_QUEUES"),
    AVFeature::new("OPT_INT_LIST"),
    AVFeature::new("OPT_PTR"),
    AVFeature::new("CPU_FLAG_FORCE"),
    AVFeature::new("DOVI_L11_INVALID_PROPS"),
    AVFeature::new("ASSERT_FPU"),
    // before 10.0 (< v62)
    AVFeature::new("VULKAN_SYNC_QUEUES"),
];

static AVCODEC_FEATURES: &[AVFeature] = &[
    // before 6.0 (< v60)
    AVFeature::new("OPENH264_SLICE_MODE"),
    AVFeature::new("OPENH264_CABAC"),
    AVFeature::new("UNUSED_CODEC_CAPS"),
    AVFeature::new("THREAD_SAFE_CALLBACKS"),
    AVFeature::new("DEBUG_MV"),
    AVFeature::new("GET_FRAME_CLASS"),
    AVFeature::new("AUTO_THREADS"),
    AVFeature::new("AVCTX_TIMEBASE"),
    AVFeature::new("FLAG_TRUNCATED"),
    AVFeature::new("SUB_TEXT_FORMAT"),
    // before 7.0 (< v61)
    AVFeature::new("IDCT_NONE"),
    AVFeature::new("SVTAV1_OPTS"),
    AVFeature::new("AYUV_CODECID"),
    AVFeature::new("VT_OUTPUT_CALLBACK"),
    AVFeature::new("AVCODEC_CHROMA_POS"),
    AVFeature::new("VT_HWACCEL_CONTEXT"),
    AVFeature::new("AVCTX_FRAME_NUMBER"),
    AVFeature::new("SLICE_OFFSET"),
    // before 8.0 (< v62)
    AVFeature::new("SUBFRAMES"),
    AVFeature::new("TICKS_PER_FRAME"),
    AVFeature::new("DROPCHANGED"),
    AVFeature::new("AVFFT"),
    AVFeature::new("FF_PROFILE_LEVEL"),
    AVFeature::new("AVCODEC_CLOSE"),
    AVFeature::new("BUFFER_MIN_SIZE"),
    AVFeature::new("VDPAU_ALLOC_GET_SET"),
    AVFeature::new("QUALITY_FACTOR"),
    // before 9.0 (< v63)
    AVFeature::new("INIT_PACKET"),
    AVFeature::new("V408_CODECID"),
    AVFeature::new("CODEC_PROPS"),
    AVFeature::new("EXR_GAMMA"),
    AVFeature::new("INTRA_DC_PRECISION"),
    AVFeature::new("NVDEC_OLD_PIX_FMTS"),
    AVFeature::new("PARSER_PRIVATE"),
    AVFeature::new("PARSER_CODECID"),
    AVFeature::new("OMX"),
    AVFeature::new("SONIC_ENC"),
    AVFeature::new("SONIC_DEC"),
];

static AVFORMAT_FEATURES: &[AVFeature] = &[
    // before 6.0 (< v60)
    AVFeature::new("LAVF_PRIV_OPT"),
    AVFeature::new("AVIOCONTEXT_WRITTEN"),
    // before 7.0 (< v61)
    AVFeature::new("GET_END_PTS"),
    AVFeature::new("AVIODIRCONTEXT"),
    AVFeature::new("AVFORMAT_IO_CLOSE"),
    AVFeature::new("AVIO_WRITE_NONCONST"),
    // before 8.0 (< v62)
    AVFeature::new("LAVF_SHORTEST"),
    AVFeature::new("ALLOW_FLUSH"),
    AVFeature::new("AVSTREAM_SIDE_DATA"),
    AVFeature::new("GET_DUR_ESTIMATE_METHOD"),
    // before 9.0 (< v63)
    AVFeature::new("COMPUTE_PKT_FIELDS2"),
    AVFeature::new("INTERNAL_TIMING"),
    AVFeature::new("NO_DEFAULT_TLS_VERIFY"),
    // after 5.0 (> v59)
    AVFeature::new("AVSTREAM_CLASS"),
    // for all eternity
    AVFeature::new("R_FRAME_RATE"),
];

static AVDEVICE_FEATURES: &[AVFeature] = &[
    // before 6.0 (< v60)
    AVFeature::new("DEVICE_CAPABILITIES"),
    // before 8.0 (< v62)
    AVFeature::new("BKTR_DEVICE"),
    AVFeature::new("OPENGL_DEVICE"),
    AVFeature::new("SDL2_DEVICE"),
    // before 9.0 (< v63)
    AVFeature::new("ALSA_CHANNELS"),
];

static AVFILTER_FEATURES: &[AVFeature] = &[
    // before 6.0 (< v9)
    AVFeature::new("SWS_PARAM_OPTION"),
    AVFeature::new("BUFFERSINK_ALLOC"),
    AVFeature::new("PAD_COUNT"),
    // before 7.0 (< v10)
    AVFeature::new("LIBPLACEBO_OPTS"),
    // before 8.0 (< v11)
    AVFeature::new("LINK_PUBLIC"),
    // before 9.0 (< v12)
    AVFeature::new("BUFFERSINK_OPTS"),
    AVFeature::new("CONTEXT_PUBLIC"),
    AVFeature::new("LIBNPP_SUPPOR"),
];

static SWSCALE_FEATURES: &[AVFeature] = &[];

static SWRESAMPLE_FEATURES: &[AVFeature] = &[];

#[derive(Debug, Clone, Copy)]
struct AVHeader {
    name: &'static str,
    from_ver: Option<u64>,
    to_ver: Option<u64>,
}

impl AVHeader {
    const fn new(name: &'static str) -> Self {
        Self {
            name,
            from_ver: None,
            to_ver: None,
        }
    }

    const fn from_ver(mut self, ver: u64) -> Self {
        self.from_ver = Some(ver);
        self
    }

    const fn to_ver(mut self, ver: u64) -> Self {
        self.to_ver = Some(ver);
        self
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
    AVHeader::new("hwcontext_drm.h"),
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
    AVHeader::new("tx.h").from_ver(60), // post-8.0
    AVHeader::new("avutil.h"),
    AVHeader::new("xtea.h"),
];
static AVCODEC_HEADERS: &[AVHeader] = &[
    AVHeader::new("avcodec.h"),
    AVHeader::new("dv_profile.h"),
    AVHeader::new("avfft.h").to_ver(61), // pre-8.0
    AVHeader::new("vorbis_parser.h"),
];
static AVFORMAT_HEADERS: &[AVHeader] = &[AVHeader::new("avformat.h"), AVHeader::new("avio.h")];
static AVDEVICE_HEADERS: &[AVHeader] = &[AVHeader::new("avdevice.h")];
static AVFILTER_HEADERS: &[AVHeader] = &[
    AVHeader::new("buffersink.h"),
    AVHeader::new("buffersrc.h"),
    AVHeader::new("avfilter.h"),
];
static SWSCALE_HEADERS: &[AVHeader] = &[AVHeader::new("swscale.h")];
static SWRESAMPLE_HEADERS: &[AVHeader] = &[AVHeader::new("swresample.h")];

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

        // mathematics.h contains some math constants that also exists in Rust
        // like M_SQRT2f == std::f32::consts::SQRT_2
        if name.starts_with("M_") {
            return Ignore;
        }

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

fn cargo_feature_enabled(feature: &str) -> bool {
    env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_ok()
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
        .inspect_err(|e| println!("Could not find ffmpeg with vcpkg: {e}"))
        .map(|library| library.include_paths)
        .ok()
}

// add well known package manager lib paths us as homebrew (or macports)
#[cfg(target_os = "macos")]
fn add_pkg_config_path() {
    use std::path::Path;

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

fn check_features(libraries: &[Library], include_paths: &[PathBuf]) -> u64 {
    let clang = clang::Clang::new().expect("Cannot find clang");
    let index = clang::Index::new(&clang, false, false);

    println!("loaded clang version: {}", clang::get_version());

    let mut code = String::new();
    for lib in libraries {
        let _ = writeln!(code, "#include <lib{}/{}.h>", lib.name, lib.name);
    }

    let mut features_defined_enabled = libraries
        .iter()
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

    let mut versions = libraries
        .iter()
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
        println!(r#"cargo::rustc-check-cfg=cfg(feature, values("{var}"))"#);
        println!(r#"cargo::metadata=check_{var}=true"#);

        if var_enabled {
            println!(r#"cargo::rustc-cfg=feature="{var}""#);
            println!(r#"cargo::metadata={var}=true"#);
        }

        // Also find out if defined or not (useful for cases where only the definition of a macro
        // can be used as distinction)
        if var_defined {
            println!(r#"cargo::rustc-cfg=feature="{var}_is_defined""#);
            println!(r#"cargo::metadata={var}_is_defined=true"#);
        }
    }

    for lib in libraries {
        let ver = if let Some(v) = versions.get(&lib.name) {
            v
        } else {
            continue;
        };
        for major in lib.min_major_version..=ver.0 {
            for minor in 0..=135 {
                if *ver >= (major, minor) {
                    println!(
                        r#"cargo::rustc-cfg=feature="{}_version_greater_than_{major}_{minor}""#,
                        lib.name,
                    );
                    println!(
                        r#"cargo::metadata={}_version_greater_than_{major}_{minor}=true"#,
                        lib.name,
                    );
                }
            }
        }
    }

    let ffmpeg_lavc_versions = [
        ("ffmpeg_6_0", 60, 3),
        ("ffmpeg_6_1", 60, 31),
        ("ffmpeg_7_0", 61, 3),
        ("ffmpeg_7_1", 61, 19),
        ("ffmpeg_8_0", 62, 11),
        ("ffmpeg_8_1", 62, 28),
    ];

    let lavc_version = *versions
        .get("avcodec")
        .expect("Unable to find the version for libavcodec");

    // This allows removing a lot of #[cfg] attributes.
    assert!(
        lavc_version >= (59, 37),
        "FFmpeg 5.1 or higher is required, but found avcodec version {lavc_version:?}"
    );

    for &(ff_version, lavc_version_major, lavc_version_minor) in &ffmpeg_lavc_versions {
        // Every possible feature needs an unconditional check-cfg to prevent warnings
        println!(r#"cargo::rustc-check-cfg=cfg(feature, values("{ff_version}"))"#,);
        println!(r#"cargo::metadata=check_{ff_version}=true"#);

        if lavc_version >= (lavc_version_major, lavc_version_minor) {
            println!(r#"cargo::rustc-cfg=feature="{ff_version}""#);
            println!(r#"cargo::metadata={ff_version}=true"#);
        }
    }

    // FIXME: Remove this hack and make Library version-aware
    lavc_version.0
}

fn link_to_libraries(libraries: &[Library], statik: bool) {
    let ffmpeg_ty = if statik { "static" } else { "dylib" };
    for lib in libraries {
        println!("cargo::rustc-link-lib={}={}", ffmpeg_ty, lib.name);
    }
    if cargo_feature_enabled("build_zlib") && cfg!(target_os = "linux") {
        println!("cargo::rustc-link-lib=z");
    }
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let statik = cargo_feature_enabled("static");

    let all_libraries = [
        Library::required("avutil", AVUTIL_FEATURES, AVUTIL_HEADERS, 57),
        Library::optional("avcodec", AVCODEC_FEATURES, AVCODEC_HEADERS, 59),
        Library::optional("avformat", AVFORMAT_FEATURES, AVFORMAT_HEADERS, 59),
        Library::optional("avdevice", AVDEVICE_FEATURES, AVDEVICE_HEADERS, 59),
        Library::optional("avfilter", AVFILTER_FEATURES, AVFILTER_HEADERS, 8),
        Library::optional("swscale", SWSCALE_FEATURES, SWSCALE_HEADERS, 6),
        Library::optional("swresample", SWRESAMPLE_FEATURES, SWRESAMPLE_HEADERS, 4),
    ];

    let enabled_libraries: Vec<_> = all_libraries
        .into_iter()
        .filter(|lib| lib.enabled())
        .collect();

    let include_paths: Vec<PathBuf> = if cargo_feature_enabled("build") {
        let install_dir = compile::build(&enabled_libraries, &out_dir).unwrap();
        println!(
            "cargo::rustc-link-search=native={}",
            install_dir.join("lib").to_string_lossy()
        );
        link_to_libraries(&enabled_libraries, statik);

        vec![install_dir.join("include")]
    }
    // Use prebuilt library
    else if let Ok(ffmpeg_dir) = env::var("FFMPEG_DIR") {
        let ffmpeg_dir = PathBuf::from(ffmpeg_dir);
        println!(
            "cargo::rustc-link-search=native={}",
            ffmpeg_dir.join("lib").to_string_lossy()
        );
        link_to_libraries(&enabled_libraries, statik);
        vec![ffmpeg_dir.join("include")]
    } else if let Some(paths) = try_vcpkg(statik) {
        // vcpkg doesn't detect the "system" dependencies
        if statik {
            if cfg!(feature = "avcodec") || cfg!(feature = "avdevice") {
                println!("cargo::rustc-link-lib=ole32");
            }

            if cfg!(feature = "avformat") {
                println!("cargo::rustc-link-lib=secur32");
                println!("cargo::rustc-link-lib=ws2_32");
            }

            // avutil dependencies
            println!("cargo::rustc-link-lib=bcrypt");
            println!("cargo::rustc-link-lib=user32");
        }

        paths
    } else {
        // Fallback to pkg-config
        add_pkg_config_path();
        let mut pkgconfig = pkg_config::Config::new();
        pkgconfig.statik(statik);

        for lib in &enabled_libraries {
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
            println!("cargo::rustc-link-lib=framework={f}");
        }
    }

    check_features(&enabled_libraries, &include_paths);

    let mut wrapper_h = String::with_capacity(2048);

    for lib in &enabled_libraries {
        for header in lib.headers {
            add_include(&mut wrapper_h, lib, header).expect("failed to write to String");
        }
    }

    // eprintln!("wrapper header: {wrapper_h}");

    let clang_includes = include_paths
        .iter()
        .map(|include| format!("-I{}", include.to_string_lossy()));

    let default_enum_style = EnumVariation::Rust {
        non_exhaustive: cargo_feature_enabled("non_exhaustive_enums"),
    };

    bindgen::Builder::default()
        .clang_args(clang_includes)
        .ctypes_prefix("libc")
        // Not trivially copyable
        .no_copy("AVChannelLayout")
        // We need/want to implement Debug by hand for some types
        .no_debug("AVChannelLayout")
        .no_debug("AVChannelCustom")
        .default_enum_style(default_enum_style)
        // Some enums can never be rustified, use the newtype
        // pattern for them instead.
        .newtype_enum("AVOptionType")
        .newtype_enum("AVAlphaMode")
        // Only generate bindings from FFmpeg headers
        .allowlist_file(r#".*[/\\]libavutil[/\\].*"#)
        .allowlist_file(r#".*[/\\]libavcodec[/\\].*"#)
        .allowlist_file(r#".*[/\\]libavformat[/\\].*"#)
        .allowlist_file(r#".*[/\\]libavdevice[/\\].*"#)
        .allowlist_file(r#".*[/\\]libavfilter[/\\].*"#)
        .allowlist_file(r#".*[/\\]libswscale[/\\].*"#)
        .allowlist_file(r#".*[/\\]libswresample[/\\].*"#)
        .opaque_type("__mingw_ldbl_type_t")
        .prepend_enum_name(false)
        .derive_eq(true)
        .size_t_is_usize(true)
        .parse_callbacks(Box::new(Callbacks))
        .header_contents("wrapper.h", &wrapper_h)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn add_include(s: &mut String, lib: &Library, header: &AVHeader) -> std::fmt::Result {
    let ver_name = format!("LIB{}_VERSION_MAJOR", lib.name.to_uppercase());

    let end_if = match (header.from_ver, header.to_ver) {
        (Some(from), Some(to)) => {
            writeln!(s, "#if {ver_name} >= {from} && {ver_name} <= {to}")?;
            true
        }
        (Some(from), None) => {
            writeln!(s, "#if {ver_name} >= {from}")?;
            true
        }
        (None, Some(to)) => {
            writeln!(s, "#if {ver_name} <= {to}")?;
            true
        }
        (None, None) => false,
    };

    writeln!(s, r#"#include "lib{}/{}""#, lib.name, header.name)?;

    if end_if {
        writeln!(s, "#endif")?;
    }

    Ok(())
}
