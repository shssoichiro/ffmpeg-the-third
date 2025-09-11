# Release checklist

This file contains the work needed when upstream FFmpeg releases a new major or minor version.

## Preparation

Do this on the `master` branch:
1. Ensure CI passes before the update
2. `cargo clean && cargo build`, go into `target/debug/build/ffmpeg-sys-.../out/` and copy the `bindings.rs` -> `./old-bindings.rs`.

### Compiling FFmpeg

Download FFmpeg itself and build the new version:
1. `git checkout n8.0`
2. `./configure --prefix=<somepath>` (remember this path for later)
3. `make -j install`

## Update ffmpeg-sys-the-third

First, prepare your development environment:
1. `export FFMPEG_DIR=<somepath>` (point to the FFmpeg you just built)
2. `cargo clean`

The build script inside `ffmpeg-sys` needs to be updated. This means:
1. Go through `libavutil/version.h` and `libav*/version_major.h` and note the API feature changes (`FF_API_*`). In `build.rs`, add a new divider per library ("before 9.0") and add all existing API features below it. Move old features into the new version.
2. Check the FFmpeg `configure` script for new features and libraries. Add them to `build.rs` and *both* `Cargo.toml` files (-sys and main crate).
3. Check `libavcodec/version_major.h` for the new major and minor versions. Add them to `ffmpeg_lavc_versions` in `build.rs`.

There is a bug in bindgen which means we have to add the channel layout values manually. So go into the FFmpeg source code directory and run

```sh
git diff n7.1..n8.0 libavutil/channel_layout.h
```

Adjust the git tags accordingly. Look for new or changed `AV_CHAN_*`, `AV_CH_LAYOUT_*` and `AV_CHANNEL_LAYOUT_*` values and add them to `ffmpeg-sys-the-third/src/avutil/channel_layout.rs`. Don't forget to add cfg-gates to new constants like `#[cfg(feature = "ffmpeg_8_0")]`.

The `-sys` crate should now build. If it doesn't, figure out why and fix the remaining errors.

After building, go into `target/debug/build/ffmpeg-sys-.../out/` and copy the `bindings.rs` -> `new-bindings.rs`.

## Update the main crate

First, compatibility should be restored. The crate needs to compile successfully before adding new features. **Never enable the crate feature `non-exhaustive-enums` during this step!** This means compiling with `--no-default-features`. Make sure that all other features are enabled though (this will result in a messy build command, sorry).

It is helpful to have FFmpeg's `doc/APIchanges` open in a separate window. `git log -S` and `git diff` in the FFmpeg repository are also useful.

Run `diff old-bindings.rs new-bindings.rs` and go through all changes. This can mean:

- Adding new enum variants (including `#[cfg(feature = "ffmpeg_8_0")]`)
    - also update the related `From<AVEnum> for Enum` and `From<Enum> for AVEnum` implementations
- Marking removed enum variants with a cfg gate (`#[cfg(not(feature = "ffmpeg_8_0"))]`)
- Updating getter/setter functions for added/removed struct fields
- Updating/adding/removing calls to FFmpeg API functions (read the docs)

There can also be other changes needed to get the crate to compile again, use your own judgment.

If you see something completely new, it is not necessary to add a new wrapper / Rust API immediately. Getting the crate to compile takes priority. Take note of big changes and additions to the FFmpeg API for later.

## Metadata, documentation etc.

When the crate compiles on your local machine, the crate manifest can be updated to reflect support for a new FFmpeg version. This usually requires a major version bump:

Cargo.toml:
```diff
[package]
name = "ffmpeg-the-third"
-version = "3.0.2+ffmpeg-7.1"
+version = "4.0.0+ffmpeg-8.0"
```

Do this for both crates.

Update the MSRV lockfile:

```sh
CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSION=fallback cargo update
cp Cargo.lock Cargo.lock.MSRV
```

### CI

Update the CI workflows for all 3 tested OSs in `.github/workflows/build.yml` to include the new FFmpeg version:
1. Add matrix entries for the new FFmpeg version (Linux, macOS and Windows).
    - If new versions aren't available yet for some OS, prioritize Linux, Windows, macOS in that order. **Make sure to update the CI workflow ASAP if you need to exclude any OS for the initial update.**
2. Update the `save-if` keys for the rust-cache to the new FFmpeg version
3. Bump the `prefix-key` for rust-cache.

### Remaining tasks

Read `doc/APIchanges` and `diff old-bindings.rs new-bindings.rs` and make sure that everything that was missed is documented in an issue so it can be added later. If the new API is small enough (e.g. one new function in an existing system), you can add it now.
