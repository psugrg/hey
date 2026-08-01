# hey

`hey` is a simple command-line AI assistant for questions about command-line tools. Ask it things like:

- "how to list all files in a directory?"
- "explain the `cat` command"
- "how to search a text in files"

and get a quick, focused answer right in your terminal.

## Features

- Natural language Q&A focused on command-line usage and tools
- Powered by [OpenRouter.ai](https://openrouter.ai)
- Minimal, single-question-per-session workflow — no clutter, no chat history

## Usage

Run `hey` with no arguments:

```
$ hey
```

The tool will wait for you to type your question and press Enter. It sends the question to OpenRouter.ai, prints the answer, then exits.

Example:

```
$ hey
> how to list all files in a directory?
You can use `ls` to list files in a directory. For example:
  ls -la
lists all files (including hidden ones) with detailed information.
```

## Configuration

`hey` requires an OpenRouter API key, provided via the `OPENROUTER_API_KEY` environment variable:

```
export OPENROUTER_API_KEY="your-api-key-here"
```

> [!TIP]
> Add this export routine to your `.bashrc` or `.zshrc` file

You can get an API key from [openrouter.ai](https://openrouter.ai).

## Installation

### Install with curl

Install the latest release with a single command:

```
curl -fsSL https://raw.githubusercontent.com/psugrg/hey/main/install.sh | sh
```

This downloads the latest `linux_amd64` release and installs the `hey` binary to `~/.local/bin` (override with the `INSTALL_DIR` environment variable, and pin a specific version with `VERSION`, e.g. `VERSION=0.1.0 sh install.sh`).

> [!TIP]
> If `~/.local/bin` isn't already on your `$PATH`, the script will tell you how to add it.

### Download a prebuilt binary

Prebuilt binaries for Linux (amd64) are published on the [GitHub Releases](../../releases) page as `hey_x.x.x_linux_amd64.tar.gz`. Download and extract the archive, then copy the `hey` binary somewhere on your `PATH`, for example:

```
tar -xzf hey_x.x.x_linux_amd64.tar.gz
cp hey ~/.local/bin/hey
```

### Build from source

Build from source using Cargo:

```
git clone <repository-url>
cd hey/hey
cargo build --release
```

The compiled binary will be available at `target/release/hey`. Copy it somewhere on your `PATH` to use it as the `hey` command, for example:

```
cp target/release/hey ~/.local/bin/hey
```

### Make it accessible

Add the `~/.local/bin` directory to your `$PATH` variable.

```
export PATH="$HOME/.local/bin:$PATH"

```

> [!TIP]
> Add this export routine to your `.bashrc` or `.zshrc` file

## Requirements

- Rust toolchain (edition 2024)
- Internet connection
- An OpenRouter API key

## Versioning

This project follows [Semantic Versioning](https://semver.org/) (`MAJOR.MINOR.PATCH`):

- **MAJOR** — breaking changes to CLI behavior (e.g. invocation, output format, env vars)
- **MINOR** — new backward-compatible features (e.g. new flags, new providers)
- **PATCH** — bug fixes and other changes with no user-facing behavior change

While the version is `0.y.z` (pre-1.0), `y` is bumped for potentially breaking changes and `z` for fixes, per common SemVer convention for early-stage projects.

The `version` field in `hey/Cargo.toml` is the single source of truth. Releases are tagged in git as `vX.Y.Z`. Run `hey --version` (or `hey -V`) to check the version of your installed binary. See [CHANGELOG.md](CHANGELOG.md) for release history.

The version number is bumped only when preparing the next release (not immediately after tagging), so `Cargo.toml` always reflects the last released version until a new release is being cut.

See [RELEASING.md](RELEASING.md) for the step-by-step release process.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Backlog

1. Refactor the code by making smaller modules that are easier to maintain

   - [x] Extract the `config` module (API key, AI model, color scheme)
   - [x] Extract the `render` module (answer rendering)
   - [x] Extract the `client` module (OpenRouter API client and spinner logic)
   - [x] Extract the `prompt` module (question input)

2. Create Github actions to generate the release assets

   - [x] Automatically build and publish the `linux_amd64` release asset (`hey_x.x.x_linux_amd64.tar.gz`) on tag push

3. Installation script

   - [x] Add the `install.sh` script that will install the latest release version from the repository.

     The link to the repository `https://github.com/psugrg/hey`.
     The example path to the asset `https://github.com/psugrg/hey/releases/download/v0.1.0/hey_0.1.0_linux_amd64.tar.gz`.
     The example invocation `curl -fsSL https://github.com/psugrg/hey/install.sh | sh`.

   - [x] Extend the installation instructions to contain the step of installing the lates version with `curl`.
