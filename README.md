[![crates.io](https://img.shields.io/crates/v/ffmpeg-the-third.svg)](https://crates.io/crates/ffmpeg-the-third)
[![docs.rs](https://docs.rs/ffmpeg-the-third/badge.svg)](https://docs.rs/ffmpeg-the-third/)
[![build](https://github.com/shssoichiro/ffmpeg-the-third/workflows/build/badge.svg)](https://github.com/shssoichiro/ffmpeg-the-third/actions)

This is a fork of the abandoned [ffmpeg-next](https://crates.io/crates/ffmpeg-next) crate which is a fork of the abandoned [ffmpeg](https://crates.io/crates/ffmpeg) crate.

Currently supported FFmpeg versions: 4.x, 5.x.

Build instructions can be found on the [wiki](https://github.com/shssoichiro/ffmpeg-the-third/wiki/Notes-on-building).

Documentation:

- [docs.rs](https://docs.rs/ffmpeg-the-third/);
- [FFmpeg user manual](https://ffmpeg.org/ffmpeg-all.html);
- [FFmpeg Doxygen](https://ffmpeg.org/doxygen/trunk/).

_Note on upgrading to v4.3.4 or later: v4.3.4 introduced automatic FFmpeg version detection, obsoleting feature flags `ffmpeg4`, `ffmpeg41`, `ffmpeg42` and `ffmpeg43`. If you manually specify any of these features, now is the time to remove them; if you use `ffmpeg43` through the `default` feature, it's still on for backward-compatibility but it has turned into a no-op, and you don't need to do anything. Deprecation plan: `ffmpeg43` will be dropped from default features come 4.4, and all these features will be removed come 5.0._

_See [CHANGELOG.md](CHANGELOG.md) for other information on version upgrades._

A word on versioning: major and minor versions of this crate track major and minor versions of FFmpeg, e.g. 4.2.x of this crate has been updated to support the 4.2.x series of FFmpeg. Patch level is reserved for changes to this crate and does not track FFmpeg patch versions. Since we can only freely bump the patch level, versioning of this crate differs from semver: minor versions may behave like semver major versions and introduce backward-incompatible changes; patch versions may behave like semver minor versions and introduce new APIs. Please peg the version you use accordingly.

**If you have significant, demonstrable experience in Rust and multimedia-related programming, please let me know, I'll be more than happy to invite you as a collaborator.**
