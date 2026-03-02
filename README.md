[![crates.io](https://img.shields.io/crates/v/ffmpeg-the-third.svg)](https://crates.io/crates/ffmpeg-the-third)
[![docs.rs](https://docs.rs/ffmpeg-the-third/badge.svg)](https://docs.rs/ffmpeg-the-third/)
[![build](https://github.com/shssoichiro/ffmpeg-the-third/workflows/build/badge.svg)](https://github.com/shssoichiro/ffmpeg-the-third/actions)

This is a fork of the abandoned [ffmpeg-next](https://crates.io/crates/ffmpeg-next) crate which is a fork of the abandoned [ffmpeg](https://crates.io/crates/ffmpeg) crate.

Currently supported FFmpeg versions: 5.0 - 8.0.

Versions that are considered [old and unmaintained](https://ffmpeg.org/olddownload.html) by FFmpeg usually work, but are not actively tested during development.

## Usage

Build instructions can be found on the [wiki](https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building). API documentation for this crate can be found on [docs.rs](https://docs.rs/ffmpeg-the-third/).

_See [CHANGELOG.md](CHANGELOG.md) for information on version upgrades._

### FFmpeg documentation

- [FFmpeg user manual](https://ffmpeg.org/ffmpeg-all.html)
- [FFmpeg Doxygen](https://ffmpeg.org/doxygen/trunk/)

## Contributing

Issues and PRs are welcome.

If you have significant, demonstrable experience in Rust and multimedia-related programming, please let me know, I'll be more than happy to invite you as a collaborator.

## Minimum supported Rust version (MSRV)

Both `ffmpeg-the-third` and `ffmpeg-sys-the-third` currently require a Rust version of 1.71.0 or higher. Increases in MSRV will result in a semver MINOR version increase.

Note that Rust versions 1.84 and newer will automatically resolve dependencies to versions that are compatible with the `rust-version` in `Cargo.toml`.
