# Changelog

## Version 4.0.0

- [Feature] FFmpeg 8.0 support

## Version 3.0.2

- Add new codec media types
- Misc code modernizations

## Version 3.0.1

- Fix compilation with `serialize` feature and newest `bitflags` crate

## Version 3.0.0

- [Breaking] remove support for < ffmpeg 4.2
- [Breaking] Remove avresample
- [Breaking] Remove old FF*API*-
- [Breaking] Remove libc error reexports
- [Breaking] Make `AVOptionType` a bitfield/int wrapper
- [Feature] ffmpeg 7.1 support
- [Feature] add API for `av_[de]muxer_iterate`
- [Feature] Implement most of the chroma location API
- [Feature] Implement `ChannelLayout::retype`
- Export all lib features
- Refactor `codec::Parameters`
- ci: Remove unnecessary pkgconfig patches
- Don't pass `AsRef<T>` params by reference
- Refactor crate::codec
- Replace util::Range with std::ops::RangeBounds
- Refactor format::{Input, Output}
- Add utils for ptr->&str conversion
- Refactor crate::format
- Add `rustc-check-cfg` for `ff*api*{...}` features
- Add `rustc-check-cfg` for `ffmpeg_x_y` features
- print clang version in build
- Use libc::c_char over i8
- Use clang for inspecting macros instead of compiling the binary.

## Version 2.0.1

- Add Copy, Clone, Eq, Debug to some bitflags types that had lost them

## Version 2.0.0

- [Breaking] Replace avutil macros with const fns
- [Feature] Support ffmpeg 7
- [Feature] Add ChannelLayout API
- Many changes to the build process
- Fix avfiltergraph input/output
- Migrate to Rust edition 2021 (only 3 years late)
- Fix PadIter in FFmpeg pre-5.0

## Version 1.2.2

- Do a better job of fixing ffmpeg 6.0 support :)

## Version 1.2.1

- Fix ffmpeg 4.x support that was broken in 1.2.0

## Version 1.2.0

- Add ffmpeg 6.0 support

## _sys_ Version 1.1.0

- Add ffmpeg 6.0 support

## _sys_ Version 1.0.2

- Fix building against clang 16 by using latest bindgen crate

## _sys_ Version 1.0.1

- Fix static building after a change in ffmpeg's branch naming structure broke it

## Version 1.1.2

No changes from 1.1.1.

## Version 1.1.1

- Fix compilation on some non-x64 platforms

## Version 1.1.0

- Add `serialize` feature, off by default, which derives `serde::{Serialize, Deserialize}` for as many types as possible

## Version 1.0.1

- Remove the "ffmpeg4.x" features that were supposed to have been removed when ffmpeg5 was released.
  This is _technically_ a breaking change, but I'm publishing it so quickly after the initial release of this fork that hopefully nobody is depending on those old, deprecated features that you should have removed ages ago anyway.

## Version 1.0.0

- Fork from https://github.com/zmwangx/rust-ffmpeg
- Fix building against git ffmpeg by making enums non-exhaustive
- Reset versioning. The new versioning scheme makes the crate version independent of the ffmpeg version,
  but tags the ffmpeg version as semver metadata. When including this crate as a dependency, you only need
  to specify the crate version itself e.g. "1.0.0". The tagged ffmpeg version is merely informative and indicates
  the latest stable ffmpeg at the time the version of the crate was released, which is also the version that
  this crate is tested against.
