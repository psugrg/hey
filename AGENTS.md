# AGENTS.md

## Repo layout

- `hey/` — the actual Rust crate (binary named `hey`). All source lives in `hey/src/`.
  - `main.rs` — entrypoint; wires config → prompt → client → render.
  - `config.rs` — reads `OPENROUTER_API_KEY` (required) and `OPENROUTER_MODEL` (optional) from env; no config file support yet.
  - `client.rs` — OpenRouter API calls + "Thinking..." spinner.
  - `prompt.rs` — terminal question input.
  - `render.rs` — renders answers, including fenced code block styling.
- Root of the repo (`README.md`, `install.sh`, `CHANGELOG.md`, `RELEASING.md`, `BACKLOG.md`) is documentation/release tooling, not crate code.
- No test suite exists in this repo.

## Platform support

- Supports both Linux and macOS.
- Linux binary is built automatically by the release workflow (`.github/workflows/release.yml`) when a `v*` tag is pushed.
- macOS binary is not published by CI; macOS users must build from source (`cargo build --release`) themselves.

## Build / run

```bash
cd hey
cargo build --release
```

Requires `OPENROUTER_API_KEY` to be set to actually run the binary (`cargo run` will fail at startup otherwise).

## Release process (do not skip steps)

Releases are tag-triggered (`.github/workflows/release.yml` on push of `v*` tags). When asked to cut a release, follow `RELEASING.md` exactly, in order:

1. Bump version in `hey/Cargo.toml`.
2. Move `[Unreleased]` in `CHANGELOG.md` into a new `## [X.Y.Z] - YYYY-MM-DD` section (heading must match the tag exactly, e.g. tag `v0.1.1` → heading `## [0.1.1]`) and start a fresh empty `[Unreleased]`. If skipped or mismatched, the release still publishes but with generic auto-generated notes instead of the changelog content.
3. Commit, then tag with `v` prefix and push tags (`git tag vX.Y.Z && git push origin main --tags`).

## Conventions

- Keep `CHANGELOG.md` updated under `[Unreleased]` for any user-facing change (Keep a Changelog format).
- Versioning is semver, but pre-1.0: `y` bumps for breaking changes, `z` for fixes (see README.md#versioning).
- Update the `Usage` section in the `README.md` file whenever there are functional changes that are affecting the user experience. (e.g. new option added, removed or behavior modified).

## Code style

- Keep the codebase modular: prefer multiple small, focused modules (as with `config`/`client`/`prompt`/`render`) over growing `main.rs` or any single file. Give new, distinct concerns their own module rather than bolting them onto an existing one.
- Minimize comments: code should be self-explanatory through naming and structure. Reserve doc comments (`///`, `//!`) for public interfaces and module-level docs (see `config.rs` for the expected style); avoid inline comments that just restate what the code does.
