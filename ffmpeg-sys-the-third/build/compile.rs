use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::cargo_feature_enabled;
use crate::Library;

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
    ("CODEC2", "libcodec2"),
    ("DAV1D", "libdav1d"),
    ("DAVS2", "libdavs2"),
    ("DCADEC", "libdcadec"),
    ("FAAC", "libfaac"),
    ("FDK_AAC", "libfdk-aac"),
    ("GSM", "libgsm"),
    ("ILBC", "libilbc"),
    ("JXL", "libjxl"),
    ("KVAZAAR", "libkvazaar"),
    ("LC3", "liblc3"),
    ("LCEVC_DEC", "liblcevc-dec"),
    ("MP3LAME", "libmp3lame"),
    ("MPEGHDEC", "libmpeghdec"),
    ("OAPV", "liboapv"),
    ("OPENCORE_AMRNB", "libopencore-amrnb"),
    ("OPENCORE_AMRWB", "libopencore-amrwb"),
    ("OPENH264", "libopenh264"),
    ("OPENH265", "libopenh265"),
    ("OPENJPEG", "libopenjpeg"),
    ("OPENMPT", "libopenmpt"),
    ("OPUS", "libopus"),
    ("RAV1E", "librav1e"),
    ("SCHROEDINGER", "libschroedinger"),
    ("SHINE", "libshine"),
    ("SNAPPY", "libsnappy"),
    ("SPEEX", "libspeex"),
    ("STAGEFRIGHT_H264", "libstagefright-h264"),
    ("SVTAV1", "libsvtav1"),
    ("SVTJPEGXS", "libsvtjpegxs"),
    ("THEORA", "libtheora"),
    ("TWOLAME", "libtwolame"),
    ("UAVS3D", "libuavs3d"),
    ("UTVIDEO", "libutvideo"),
    ("VO_AACENC", "libvo-aacenc"),
    ("VO_AMRWBENC", "libvo-amrwbenc"),
    ("VORBIS", "libvorbis"),
    ("VPX", "libvpx"),
    ("VVENC", "libvvenc"),
    ("WAVPACK", "libwavpack"),
    ("WEBP", "libwebp"),
    ("X264", "libx264"),
    ("X265", "libx265"),
    ("XEVE", "libxeve"),
    ("XEVD", "libxevd"),
    ("XAVS", "libxavs"),
    ("XAVS2", "libxavs2"),
    ("AVS", "libavs"),
    ("XVID", "libxvid"),
    // Protocols
    ("SMBCLIENT", "libsmbclient"),
    ("SSH", "libssh"),
];

fn get_repo_url() -> String {
    env::var("FFMPEG_GIT_URL").unwrap_or("https://github.com/FFmpeg/FFmpeg".into())
}

fn get_newest_patch_version() -> String {
    let crate_ffmpeg_version = env!("CARGO_PKG_VERSION")
        .split_once("+ffmpeg-")
        .expect("crate version follows v1.2.3+ffmpeg-4.5 format")
        .1;

    // The crate version usually doesn't reference patch releases, so
    // see if there's a compatible patch (i.e. 8.0.3 for 8.0) in the remote repository.

    let Output { status, stdout, .. } = Command::new("git")
        .arg("ls-remote")
        .arg("-q")
        .arg("--tags")
        .arg("--refs")
        .arg(&get_repo_url())
        .arg(format!("n{}*", crate_ffmpeg_version))
        .output()
        .expect("can run git ls-remote");

    assert!(
        status.success(),
        "git ls-remote returned non-zero exit code"
    );

    String::from_utf8(stdout)
        .expect("git ls-remote output is utf8")
        .lines()
        // format follows <commit hash><TAB>refs/tags/n8.0
        .filter_map(|line| line.split_once("refs/tags/n"))
        .map(|(_hash, version)| version)
        .filter(|ver| !ver.contains("-dev"))
        .max() // lexicographic maximum is the highest version
        .expect("matching non-dev tag exists")
        .to_string()
}

fn fetch(source_dir: &Path, ffmpeg_version: &str) -> io::Result<()> {
    let _ = std::fs::remove_dir_all(source_dir);
    let status = Command::new("git")
        .arg("clone")
        .arg("--depth=1")
        .arg("-b")
        .arg(format!("n{ffmpeg_version}"))
        .arg(&get_repo_url())
        .arg(source_dir)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other("fetch failed"))
    }
}

pub fn build(libraries: &[Library], out_dir: &Path) -> io::Result<PathBuf> {
    println!("cargo::rerun-if-env-changed=FFMPEG_GIT_URL");

    let ffmpeg_version = get_newest_patch_version();
    let source_dir = out_dir.join(format!("ffmpeg-{ffmpeg_version}"));
    let install_dir = out_dir.join("dist");
    if install_dir.join("lib").join("libavutil.a").exists() {
        rustc_link_extralibs(&source_dir);
        return Ok(install_dir);
    }

    fetch(&source_dir, &ffmpeg_version)?;

    // Explicitly enable building all libraries passed to this function
    let library_flags = libraries.iter().map(|lib| format!("--enable-{}", lib.name));

    // Command's path is not relative to command's current_dir
    let configure_path = source_dir.join("configure");
    assert!(configure_path.exists());
    let mut configure = Command::new(configure_path);
    configure.current_dir(&source_dir);

    configure.arg(format!("--prefix={}", install_dir.to_string_lossy()));
    configure.args(library_flags);

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

    // do not build documentation
    configure.arg("--disable-doc");

    // the binary using ffmpeg-sys must comply with GPL
    configure.switch("BUILD_LICENSE_GPL", "gpl");

    // the binary using ffmpeg-sys must comply with (L)GPLv3
    configure.switch("BUILD_LICENSE_VERSION3", "version3");

    // the binary using ffmpeg-sys cannot be redistributed
    configure.switch("BUILD_LICENSE_NONFREE", "nonfree");

    // configure external libraries based on features
    for (cargo_feat, option_name) in EXTERNAL_BUILD_LIBS {
        configure.enable(&format!("BUILD_LIB_{cargo_feat}"), option_name);
    }

    configure.enable("BUILD_DRM", "libdrm");
    configure.enable("BUILD_NVENC", "nvenc");

    // run ./configure
    let output = configure.output()?;
    if !output.status.success() {
        println!("configure: {}", String::from_utf8_lossy(&output.stdout));

        return Err(io::Error::other(format!(
            "configure failed {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    if !Command::new("make")
        .arg("install")
        .env("MAKEFLAGS", env::var("CARGO_MAKEFLAGS").unwrap())
        .current_dir(&source_dir)
        .status()?
        .success()
    {
        return Err(io::Error::other("make install failed"));
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
        .map(|line| line.expect("config.mak contains valid utf8"))
        .find(|line| line.starts_with("EXTRALIBS="))
        .expect("config.mak contains EXTRALIBS= line");

    let include_libs = extra_libs
        .strip_prefix("EXTRALIBS=")
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|v| v.strip_prefix("-l"));

    for lib in include_libs {
        println!("cargo::rustc-link-lib={lib}");
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
