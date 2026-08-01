# Releasing

This document describes how to cut a new release of `hey`.

## Prerequisites

- Push access to the repository (tags trigger the release workflow).
- The `GITHUB_TOKEN` used by GitHub Actions has `contents: write` permission, already configured in the workflow — no extra secrets are needed for a same-repo release.

## Steps to trigger a release

1. **Bump the version** in `hey/Cargo.toml` (e.g. `0.1.0` → `0.1.1`), following the versioning convention described in [README.md](README.md#versioning).

2. **Update `CHANGELOG.md`**: move the `[Unreleased]` content into a new dated section (e.g. `## [0.1.1] - 2026-08-01`), and start a fresh empty `[Unreleased]` section for future work.

3. **Commit these changes**:

   ```
   git commit -am "Release v0.1.1"
   ```

4. **Create and push a git tag** matching the pattern `v*` that the release workflow listens for:

   ```
   git tag v0.1.1
   git push origin main --tags
   ```

   (or `git push origin v0.1.1` to push just the tag)

## What happens automatically once the tag is pushed

The [`.github/workflows/release.yml`](.github/workflows/release.yml) workflow triggers on `push: tags: v*` and runs:

1. **Checkout** the repo at that tag.
2. **Install Rust** (stable toolchain) and restore the cargo/target cache.
3. **Build**: `cargo build --release --manifest-path hey/Cargo.toml`.
4. **Derive the version** from the tag name — strips the leading `v`, so `v0.1.1` → `0.1.1`.
5. **Package**: copies the built `hey` binary, `LICENSE`, and `README.md` into a staging folder, then tars them into `hey_0.1.1_linux_amd64.tar.gz` (files sit at the archive root, no subdirectory).
6. **Publish**: uses `softprops/action-gh-release@v2` to create a GitHub Release for that tag (if one doesn't already exist) and attach the `.tar.gz` as a release asset.

## End result

A new GitHub Release appears at `vX.Y.Z` on the repo's Releases page, with `hey_X.Y.Z_linux_amd64.tar.gz` downloadable — matching the "Download a prebuilt binary" instructions in [README.md](README.md#installation).
