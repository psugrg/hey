# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `-f`/`--follow-up` flag to continue the previous conversation with a follow-up question. Conversation history is scoped per terminal (keyed by the controlling TTY), so multiple open terminals don't mix contexts.
- `install.sh` now checks whether `OPENROUTER_API_KEY` is set and, if not, guides the user to get a key from [openrouter.ai](https://openrouter.ai) and add it to their environment (and to `.bashrc`/`.zshrc`).

### Changed

- The release workflow now sources GitHub Release notes from the corresponding `CHANGELOG.md` section, falling back to auto-generated notes if no matching section is found.

## [0.2.0] - 2026-08-01

### Added

- Fenced code blocks in answers are now rendered with grey top/bottom borders showing the language, and the code itself in blue.
- GitHub Actions workflow to automatically build and publish the `linux_amd64` release asset (`hey_x.x.x_linux_amd64.tar.gz`, including the binary, `LICENSE`, and `README.md`) when a version tag is pushed.
- `install.sh` script to download and install the latest (or a specific, via `VERSION`) `linux_amd64` release binary from GitHub, configurable via the `INSTALL_DIR` environment variable.

### Changed

- Extracted configuration handling (API key, AI model, color scheme) into a dedicated `config` module.
- Extracted answer rendering logic into a dedicated `render` module.
- Extracted the OpenRouter API client and "Thinking..." spinner logic into a dedicated `client` module.
- Extracted question prompt/input handling into a dedicated `prompt` module.
- Prompt marker (`> `) is now configurable in the `config` module.
- README installation instructions now include a one-line `curl | sh` install step using `install.sh`.

## [0.1.0] - 2026-07-30

### Added

- Natural language Q&A for command-line tool questions, powered by [OpenRouter.ai](https://openrouter.ai).
- "Thinking..." spinner animation while waiting for the API response.
- Question prompt enclosed with top and bottom divider lines.
- `--version` / `-V` flag to print the current version.
