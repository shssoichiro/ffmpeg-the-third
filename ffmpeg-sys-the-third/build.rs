extern crate bindgen;
extern crate cc;
extern crate pkg_config;

use std::env;
use std::fmt::Write as FmtWrite;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::Command;
use std::str;

use bindgen::callbacks::{
    EnumVariantCustomBehavior, EnumVariantValue, IntKind, MacroParsingBehavior, ParseCallbacks,
};

#[derive(Debug)]
struct Library {
    name: &'static str,
    is_feature: bool,
    features: &'static [AVFeature],
}

impl Library {
    const fn new(name: &'static str, is_feature: bool, features: &'static [AVFeature]) -> Self {
        Self {
            name,
            is_feature,
            features,
        }
    }
}

static LIBRARIES: &[Library] = &[
    Library::new("avutil", false, AVUTIL_FEATURES),
    Library::new("avcodec", true, AVCODEC_FEATURES),
    Library::new("avformat", true, AVFORMAT_FEATURES),
    Library::new("avdevice", true, AVDEVICE_FEATURES),
    Library::new("avfilter", true, AVFILTER_FEATURES),
    Library::new("avresample", true, AVRESAMPLE_FEATURES),
    Library::new("swscale", true, SWSCALE_FEATURES),
    Library::new("swresample", true, SWRESAMPLE_FEATURES),
    Library::new("postproc", true, POSTPROC_FEATURES),
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
];

static AVDEVICE_FEATURES: &[AVFeature] = &[AVFeature::new("DEVICE_CAPABILITIES")];

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
];

static AVRESAMPLE_FEATURES: &[AVFeature] = &[AVFeature::new("RESAMPLE_CLOSE_OPEN")];

static SWSCALE_FEATURES: &[AVFeature] =
    &[AVFeature::new("SWS_CPU_CAPS"), AVFeature::new("ARCH_BFIN")];

static SWRESAMPLE_FEATURES: &[AVFeature] = &[];

static POSTPROC_FEATURES: &[AVFeature] = &[];

#[derive(Debug)]
struct Callbacks;

impl ParseCallbacks for Callbacks {
    fn int_macro(&self, _name: &str, value: i64) -> Option<IntKind> {
        let ch_layout_prefix = "AV_CH_";
        let codec_cap_prefix = "AV_CODEC_CAP_";
        let codec_flag_prefix = "AV_CODEC_FLAG_";
        let error_max_size = "AV_ERROR_MAX_STRING_SIZE";

        if _name.starts_with(ch_layout_prefix) {
            Some(IntKind::ULongLong)
        } else if value >= i32::min_value() as i64
            && value <= i32::max_value() as i64
            && (_name.starts_with(codec_cap_prefix) || _name.starts_with(codec_flag_prefix))
        {
            Some(IntKind::UInt)
        } else if _name == error_max_size {
            Some(IntKind::Custom {
                name: "usize",
                is_signed: false,
            })
        } else if value >= i32::min_value() as i64 && value <= i32::max_value() as i64 {
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

fn ffmpeg_major_version() -> u32 {
    ffmpeg_version().split('.').next().unwrap().parse().unwrap()
}

fn output() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn source() -> PathBuf {
    output().join(format!("ffmpeg-{}", ffmpeg_version()))
}

fn search() -> PathBuf {
    let mut absolute = env::current_dir().unwrap();
    absolute.push(&output());
    absolute.push("dist");

    absolute
}

fn fetch() -> io::Result<()> {
    let output_base_path = output();
    let clone_dest_dir = format!("ffmpeg-{}", ffmpeg_version());
    let _ = std::fs::remove_dir_all(output_base_path.join(&clone_dest_dir));
    let status = Command::new("git")
        .current_dir(&output_base_path)
        .arg("clone")
        .arg("--depth=1")
        .arg("-b")
        .arg(format!("n{}", ffmpeg_version()))
        .arg("https://github.com/FFmpeg/FFmpeg")
        .arg(&clone_dest_dir)
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

fn build() -> io::Result<()> {
    let source_dir = source();

    // Command's path is not relative to command's current_dir
    let configure_path = source_dir.join("configure");
    assert!(configure_path.exists());
    let mut configure = Command::new(&configure_path);
    configure.current_dir(&source_dir);

    configure.arg(format!("--prefix={}", search().to_string_lossy()));

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

    let ffmpeg_major_version: u32 = ffmpeg_major_version();

    // configure building libraries based on features
    for lib in LIBRARIES
        .iter()
        .filter(|lib| lib.is_feature)
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
        .current_dir(&source())
        .status()?
        .success()
    {
        return Err(io::Error::new(io::ErrorKind::Other, "make failed"));
    }

    // run make install
    if !Command::new("make")
        .current_dir(&source())
        .arg("install")
        .status()?
        .success()
    {
        return Err(io::Error::new(io::ErrorKind::Other, "make install failed"));
    }

    Ok(())
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

fn check_features(include_paths: &[PathBuf]) {
    let mut includes_code = String::new();
    let mut main_code = String::new();

    for lib in LIBRARIES {
        if lib.is_feature && !cargo_feature_enabled(lib.name) {
            continue;
        }

        let header = format!("lib{}/{}.h", lib.name, lib.name);
        for feature in lib.features {
            let var = format!("FF_API_{}", feature.name);

            let include = format!("#include <{}>", header);
            if !includes_code.contains(&include) {
                includes_code.push_str(&include);
                includes_code.push('\n');
            }
            let _ = write!(
                includes_code,
                r#"
                #ifndef {var}_is_defined
                #ifndef {var}
                #define {var} 0
                #define {var}_is_defined 0
                #else
                #define {var}_is_defined 1
                #endif
                #endif
            "#,
                var = var
            );

            let _ = write!(
                main_code,
                r#"printf("[{var}]%d%d\n", {var}, {var}_is_defined);
                "#,
                var = var
            );
        }
    }
    let version_check_info = [("avcodec", 56, 61, 0, 108)];
    for &(lib, begin_version_major, end_version_major, begin_version_minor, end_version_minor) in
        version_check_info.iter()
    {
        for version_major in begin_version_major..end_version_major {
            for version_minor in begin_version_minor..end_version_minor {
                let _ = write!(
                    main_code,
                    r#"printf("[{lib}_version_greater_than_{version_major}_{version_minor}]%d\n", LIB{lib_uppercase}_VERSION_MAJOR > {version_major} || (LIB{lib_uppercase}_VERSION_MAJOR == {version_major} && LIB{lib_uppercase}_VERSION_MINOR > {version_minor}));
                    "#,
                    lib = lib,
                    lib_uppercase = lib.to_uppercase(),
                    version_major = version_major,
                    version_minor = version_minor
                );
            }
        }
    }

    let out_dir = output();

    write!(
        File::create(out_dir.join("check.c")).expect("Failed to create file"),
        r#"
            #include <stdio.h>
            {includes_code}

            int main()
            {{
                {main_code}
                return 0;
            }}
           "#,
        includes_code = includes_code,
        main_code = main_code
    )
    .expect("Write failed");

    let executable = out_dir.join(if cfg!(windows) { "check.exe" } else { "check" });
    let mut compiler = cc::Build::new()
        .target(&env::var("HOST").unwrap()) // don't cross-compile this
        .get_compiler()
        .to_command();

    for dir in include_paths {
        compiler.arg("-I");
        compiler.arg(dir.to_string_lossy().into_owned());
    }
    if !compiler
        .current_dir(&out_dir)
        .arg("-o")
        .arg(&executable)
        .arg("check.c")
        .status()
        .expect("Command failed")
        .success()
    {
        panic!("Compile failed");
    }

    let check_output = Command::new(out_dir.join(&executable))
        .current_dir(&out_dir)
        .output()
        .expect("Check failed");
    if !check_output.status.success() {
        panic!(
            "{} failed: {}\n{}",
            executable.display(),
            String::from_utf8_lossy(&check_output.stdout),
            String::from_utf8_lossy(&check_output.stderr)
        );
    }

    let stdout = str::from_utf8(&check_output.stdout).unwrap();

    println!("stdout of {}={}", executable.display(), stdout);

    for lib in LIBRARIES {
        if lib.is_feature && !cargo_feature_enabled(lib.name) {
            continue;
        }

        for feature in lib.features {
            let var = format!("FF_API_{}", feature.name);
            let var_str = format!("[{var}]");
            let pos = var_str.len()
                + stdout
                    .find(&var_str)
                    .unwrap_or_else(|| panic!("Variable '{}' not found in stdout output", var_str));
            if &stdout[pos..pos + 1] == "1" {
                println!(r#"cargo:rustc-cfg=feature="{}""#, var.to_lowercase());
                println!(r#"cargo:{}=true"#, var.to_lowercase());
            }

            // Also find out if defined or not (useful for cases where only the definition of a macro
            // can be used as distinction)
            if &stdout[pos + 1..pos + 2] == "1" {
                println!(
                    r#"cargo:rustc-cfg=feature="{}_is_defined""#,
                    var.to_lowercase()
                );
                println!(r#"cargo:{}_is_defined=true"#, var.to_lowercase());
            }
        }
    }

    for &(lib, begin_version_major, end_version_major, begin_version_minor, end_version_minor) in
        version_check_info.iter()
    {
        for version_major in begin_version_major..end_version_major {
            for version_minor in begin_version_minor..end_version_minor {
                let search_str = format!(
                    "[{lib}_version_greater_than_{version_major}_{version_minor}]",
                    version_major = version_major,
                    version_minor = version_minor,
                    lib = lib
                );
                let pos = stdout
                    .find(&search_str)
                    .expect("Variable not found in output")
                    + search_str.len();

                if &stdout[pos..pos + 1] == "1" {
                    println!(
                        r#"cargo:rustc-cfg=feature="{}""#,
                        &search_str[1..(search_str.len() - 1)]
                    );
                    println!(r#"cargo:{}=true"#, &search_str[1..(search_str.len() - 1)]);
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
    ];
    for &(ffmpeg_version_flag, lavc_version_major, lavc_version_minor) in
        ffmpeg_lavc_versions.iter()
    {
        let search_str = format!(
            "[avcodec_version_greater_than_{lavc_version_major}_{lavc_version_minor}]",
            lavc_version_major = lavc_version_major,
            lavc_version_minor = lavc_version_minor - 1
        );
        let pos = stdout
            .find(&search_str)
            .expect("Variable not found in output")
            + search_str.len();
        if &stdout[pos..pos + 1] == "1" {
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
    for lib in LIBRARIES {
        if !lib.is_feature || cargo_feature_enabled(lib.name) {
            println!("cargo:rustc-link-lib={}={}", ffmpeg_ty, lib.name);
        }
    }
    if cargo_feature_enabled("build_zlib") && cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=z");
    }
}

fn main() {
    let statik = cargo_feature_enabled("static");
    let ffmpeg_major_version: u32 = ffmpeg_major_version();

    let include_paths: Vec<PathBuf> = if cargo_feature_enabled("build") {
        println!(
            "cargo:rustc-link-search=native={}",
            search().join("lib").to_string_lossy()
        );
        link_to_libraries(statik);
        if fs::metadata(search().join("lib").join("libavutil.a")).is_err() {
            fs::create_dir_all(output()).expect("failed to create build directory");
            fetch().unwrap();
            build().unwrap();
        }

        // Check additional required libraries.
        {
            let config_mak = source().join("ffbuild/config.mak");
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
                println!("cargo:rustc-link-lib={}", lib);
            }
        }

        vec![search().join("include")]
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
    }
    // Fallback to pkg-config
    else {
        add_pkg_config_path();
        pkg_config::Config::new()
            .statik(statik)
            .probe("libavutil")
            .unwrap();

        let mut libs = vec![
            ("libavformat", "AVFORMAT"),
            ("libavfilter", "AVFILTER"),
            ("libavdevice", "AVDEVICE"),
            ("libswscale", "SWSCALE"),
            ("libswresample", "SWRESAMPLE"),
        ];
        if ffmpeg_major_version < 5 {
            libs.push(("libavresample", "AVRESAMPLE"));
        }

        for (lib_name, env_variable_name) in libs.iter() {
            if cargo_feature_enabled(env_variable_name) {
                pkg_config::Config::new()
                    .statik(statik)
                    .probe(lib_name)
                    .unwrap();
            }
        }

        pkg_config::Config::new()
            .statik(statik)
            .probe("libavcodec")
            .unwrap()
            .include_paths
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
        // https://github.com/rust-lang/rust-bindgen/issues/550
        .blocklist_type("max_align_t")
        .blocklist_function("_.*")
        // Blocklist functions with u128 in signature.
        // https://github.com/zmwangx/rust-ffmpeg-sys/issues/1
        // https://github.com/rust-lang/rust-bindgen/issues/1549
        .blocklist_function("acoshl")
        .blocklist_function("acosl")
        .blocklist_function("asinhl")
        .blocklist_function("asinl")
        .blocklist_function("atan2l")
        .blocklist_function("atanhl")
        .blocklist_function("atanl")
        .blocklist_function("cbrtl")
        .blocklist_function("ceill")
        .blocklist_function("copysignl")
        .blocklist_function("coshl")
        .blocklist_function("cosl")
        .blocklist_function("dreml")
        .blocklist_function("ecvt_r")
        .blocklist_function("erfcl")
        .blocklist_function("erfl")
        .blocklist_function("exp2l")
        .blocklist_function("expl")
        .blocklist_function("expm1l")
        .blocklist_function("fabsl")
        .blocklist_function("fcvt_r")
        .blocklist_function("fdiml")
        .blocklist_function("finitel")
        .blocklist_function("floorl")
        .blocklist_function("fmal")
        .blocklist_function("fmaxl")
        .blocklist_function("fminl")
        .blocklist_function("fmodl")
        .blocklist_function("frexpl")
        .blocklist_function("gammal")
        .blocklist_function("hypotl")
        .blocklist_function("ilogbl")
        .blocklist_function("isinfl")
        .blocklist_function("isnanl")
        .blocklist_function("j0l")
        .blocklist_function("j1l")
        .blocklist_function("jnl")
        .blocklist_function("ldexpl")
        .blocklist_function("lgammal")
        .blocklist_function("lgammal_r")
        .blocklist_function("llrintl")
        .blocklist_function("llroundl")
        .blocklist_function("log10l")
        .blocklist_function("log1pl")
        .blocklist_function("log2l")
        .blocklist_function("logbl")
        .blocklist_function("logl")
        .blocklist_function("lrintl")
        .blocklist_function("lroundl")
        .blocklist_function("modfl")
        .blocklist_function("nanl")
        .blocklist_function("nearbyintl")
        .blocklist_function("nextafterl")
        .blocklist_function("nexttoward")
        .blocklist_function("nexttowardf")
        .blocklist_function("nexttowardl")
        .blocklist_function("powl")
        .blocklist_function("qecvt")
        .blocklist_function("qecvt_r")
        .blocklist_function("qfcvt")
        .blocklist_function("qfcvt_r")
        .blocklist_function("qgcvt")
        .blocklist_function("remainderl")
        .blocklist_function("remquol")
        .blocklist_function("rintl")
        .blocklist_function("roundl")
        .blocklist_function("scalbl")
        .blocklist_function("scalblnl")
        .blocklist_function("scalbnl")
        .blocklist_function("significandl")
        .blocklist_function("sinhl")
        .blocklist_function("sinl")
        .blocklist_function("sqrtl")
        .blocklist_function("strtold")
        .blocklist_function("tanhl")
        .blocklist_function("tanl")
        .blocklist_function("tgammal")
        .blocklist_function("truncl")
        .blocklist_function("y0l")
        .blocklist_function("y1l")
        .blocklist_function("ynl")
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
    if cargo_feature_enabled("avcodec") {
        builder = builder
            .header(search_include(&include_paths, "libavcodec/avcodec.h"))
            .header(search_include(&include_paths, "libavcodec/dv_profile.h"))
            .header(search_include(&include_paths, "libavcodec/avfft.h"))
            .header(search_include(&include_paths, "libavcodec/vorbis_parser.h"));

        if ffmpeg_major_version < 5 {
            builder = builder.header(search_include(&include_paths, "libavcodec/vaapi.h"))
        }
    }

    if cargo_feature_enabled("avdevice") {
        builder = builder.header(search_include(&include_paths, "libavdevice/avdevice.h"));
    }

    if cargo_feature_enabled("avfilter") {
        builder = builder
            .header(search_include(&include_paths, "libavfilter/buffersink.h"))
            .header(search_include(&include_paths, "libavfilter/buffersrc.h"))
            .header(search_include(&include_paths, "libavfilter/avfilter.h"));
    }

    if cargo_feature_enabled("avformat") {
        builder = builder
            .header(search_include(&include_paths, "libavformat/avformat.h"))
            .header(search_include(&include_paths, "libavformat/avio.h"));
    }

    if cargo_feature_enabled("avresample") {
        builder = builder.header(search_include(&include_paths, "libavresample/avresample.h"));
    }

    builder = builder
        .header(search_include(&include_paths, "libavutil/adler32.h"))
        .header(search_include(&include_paths, "libavutil/aes.h"))
        .header(search_include(&include_paths, "libavutil/audio_fifo.h"))
        .header(search_include(&include_paths, "libavutil/base64.h"))
        .header(search_include(&include_paths, "libavutil/blowfish.h"))
        .header(search_include(&include_paths, "libavutil/bprint.h"))
        .header(search_include(&include_paths, "libavutil/buffer.h"))
        .header(search_include(&include_paths, "libavutil/camellia.h"))
        .header(search_include(&include_paths, "libavutil/cast5.h"))
        .header(search_include(&include_paths, "libavutil/channel_layout.h"))
        .header(search_include(&include_paths, "libavutil/cpu.h"))
        .header(search_include(&include_paths, "libavutil/crc.h"))
        .header(search_include(&include_paths, "libavutil/dict.h"))
        .header(search_include(&include_paths, "libavutil/display.h"))
        .header(search_include(&include_paths, "libavutil/downmix_info.h"))
        .header(search_include(&include_paths, "libavutil/error.h"))
        .header(search_include(&include_paths, "libavutil/eval.h"))
        .header(search_include(&include_paths, "libavutil/fifo.h"))
        .header(search_include(&include_paths, "libavutil/file.h"))
        .header(search_include(&include_paths, "libavutil/frame.h"))
        .header(search_include(&include_paths, "libavutil/hash.h"))
        .header(search_include(&include_paths, "libavutil/hmac.h"))
        .header(search_include(&include_paths, "libavutil/hwcontext.h"))
        .header(search_include(&include_paths, "libavutil/imgutils.h"))
        .header(search_include(&include_paths, "libavutil/lfg.h"))
        .header(search_include(&include_paths, "libavutil/log.h"))
        .header(search_include(&include_paths, "libavutil/lzo.h"))
        .header(search_include(&include_paths, "libavutil/macros.h"))
        .header(search_include(&include_paths, "libavutil/mathematics.h"))
        .header(search_include(&include_paths, "libavutil/md5.h"))
        .header(search_include(&include_paths, "libavutil/mem.h"))
        .header(search_include(&include_paths, "libavutil/motion_vector.h"))
        .header(search_include(&include_paths, "libavutil/murmur3.h"))
        .header(search_include(&include_paths, "libavutil/opt.h"))
        .header(search_include(&include_paths, "libavutil/parseutils.h"))
        .header(search_include(&include_paths, "libavutil/pixdesc.h"))
        .header(search_include(&include_paths, "libavutil/pixfmt.h"))
        .header(search_include(&include_paths, "libavutil/random_seed.h"))
        .header(search_include(&include_paths, "libavutil/rational.h"))
        .header(search_include(&include_paths, "libavutil/replaygain.h"))
        .header(search_include(&include_paths, "libavutil/ripemd.h"))
        .header(search_include(&include_paths, "libavutil/samplefmt.h"))
        .header(search_include(&include_paths, "libavutil/sha.h"))
        .header(search_include(&include_paths, "libavutil/sha512.h"))
        .header(search_include(&include_paths, "libavutil/stereo3d.h"))
        .header(search_include(&include_paths, "libavutil/avstring.h"))
        .header(search_include(&include_paths, "libavutil/threadmessage.h"))
        .header(search_include(&include_paths, "libavutil/time.h"))
        .header(search_include(&include_paths, "libavutil/timecode.h"))
        .header(search_include(&include_paths, "libavutil/twofish.h"))
        .header(search_include(&include_paths, "libavutil/avutil.h"))
        .header(search_include(&include_paths, "libavutil/xtea.h"));

    if cargo_feature_enabled("postproc") {
        builder = builder.header(search_include(&include_paths, "libpostproc/postprocess.h"));
    }

    if cargo_feature_enabled("swresample") {
        builder = builder.header(search_include(&include_paths, "libswresample/swresample.h"));
    }

    if cargo_feature_enabled("swscale") {
        builder = builder.header(search_include(&include_paths, "libswscale/swscale.h"));
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
        .write_to_file(output().join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
