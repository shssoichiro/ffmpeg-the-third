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
