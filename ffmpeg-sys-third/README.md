[![ffmpeg-sys-the-third on crates.io](https://img.shields.io/crates/v/ffmpeg-sys-the-third?cacheSeconds=3600)](https://crates.io/crates/ffmpeg-sys-the-third)
[![build](https://github.com/shssoichiro/ffmpeg-the-third-sys/workflows/build/badge.svg)](https://github.com/shssoichiro/ffmpeg-the-third-sys/actions)

This is a fork of the abandoned [ffmpeg-sys](https://github.com/meh/rust-ffmpeg-sys) crate. You can find this crate as [ffmpeg-sys-the-third](https://crates.io/crates/ffmpeg-sys-the-third) on crates.io.

This crate contains low level bindings to FFmpeg. You're probably interested in the high level bindings instead: [ffmpeg-next](https://github.com/shssoichiro/ffmpeg-the-third).

A word on versioning: The crate version includes the **maximum supported** FFmpeg version for each release. E.g. `ffmpeg-sys-the-third@2.0.0+ffmpeg-7.0` supports *up to* FFmpeg 7.0. The minimum supported FFmpeg version at the moment is 4.2.

## Feature flags

In addition to feature flags declared in `Cargo.toml`, this crate performs various compile-time version and feature detections and exposes the results in additional flags. These flags are briefly documented below; run `cargo build -vv` to view more details.

- `ffmpeg_<x>_<y>` flags, e.g. `ffmpeg_4_4`, indicating the FFmpeg installation being compiled against is at least version `<x>.<y>`. Currently available:

  - "ffmpeg_4_3"
  - "ffmpeg_4_4"
  - "ffmpeg_5_0"
  - "ffmpeg_5_1"
  - "ffmpeg_6_0"
  - "ffmpeg_6_1"
  - "ffmpeg_7_0"
  - "ffmpeg_7_1"

- `avcodec_version_greater_than_<x>_<y>`, e.g., `avcodec_version_greater_than_58_90`. The name should be self-explanatory.

- `ff_api_<feature>`, e.g. `ff_api_vaapi`, corresponding to whether their respective uppercase deprecation guards evaluate to true.

- `ff_api_<feature>_is_defined`, e.g. `ff_api_vappi_is_defined`, similar to above except these are enabled as long as the corresponding deprecation guards are defined.
