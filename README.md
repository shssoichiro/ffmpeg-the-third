[![crates.io](https://img.shields.io/crates/v/ffmpeg-the-third.svg)](https://crates.io/crates/ffmpeg-the-third)
[![docs.rs](https://docs.rs/ffmpeg-the-third/badge.svg)](https://docs.rs/ffmpeg-the-third/)
[![build](https://github.com/shssoichiro/ffmpeg-the-third/workflows/build/badge.svg)](https://github.com/shssoichiro/ffmpeg-the-third/actions)

This is a fork of the abandoned [ffmpeg-next](https://crates.io/crates/ffmpeg-next) crate which is a fork of the abandoned [ffmpeg](https://crates.io/crates/ffmpeg) crate.

Currently supported FFmpeg versions: 4.x, 5.x, 6.x.

Build instructions can be found on the [wiki](https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building).

Documentation:

- [docs.rs](https://docs.rs/ffmpeg-the-third/);
- [FFmpeg user manual](https://ffmpeg.org/ffmpeg-all.html);
- [FFmpeg Doxygen](https://ffmpeg.org/doxygen/trunk/).

_See [CHANGELOG.md](CHANGELOG.md) for other information on version upgrades._

**If you have significant, demonstrable experience in Rust and multimedia-related programming, please let me know, I'll be more than happy to invite you as a collaborator.**

## Minimum supported Rust version (MSRV)

Both `ffmpeg-the-third` and `ffmpeg-sys-the-third` currently require a Rust version of 1.61.0 or higher. Increases in MSRV will result in a semver MINOR version increase.

If you are having issues compiling this crate on your version of Rust, there are two tools you can use to help find MSRV-compatible dependency versions:

- Install a nightly Rust toolchain and run `cargo +nightly update -Zmsrv-policy`. This will automatically resolve dependencies to versions that are compatible with the `rust-version` in `Cargo.toml`.
- Check the `Cargo.lock.MSRV` in this repository. It contains dependency versions that are known to compile on the MSRV. In the simplest case, you can just `cp Cargo.lock.MSRV Cargo.lock`. For more complex dependency graphs, you might need to manually select compatible versions from the `Cargo.lock.MSRV`.
