# hey

`hey` is a simple command-line AI assistant for your terminal - right there where you need it.

---

![Example usage of the hey application](./.github/example_usage.gif)

---

Install the latest release with a single command

```bash
curl -fsSL https://raw.githubusercontent.com/psugrg/hey/main/install.sh | sh
```

> [!NOTE]
> `hey` requires the [OpenRouter.ai](https://openrouter.ai) API key to function!

## Installation methods

### Install with curl

Install the latest release with a single command:

```bash
curl -fsSL https://raw.githubusercontent.com/psugrg/hey/main/install.sh | sh
```

This downloads the latest `linux_amd64` release and installs the `hey` binary to `~/.local/bin` (override with the `INSTALL_DIR` environment variable, and pin a specific version with `VERSION`, e.g. `VERSION=0.1.0 sh install.sh`).

> [!TIP]
> If `~/.local/bin` isn't already on your `$PATH`, the script will tell you how to add it.

### Download a prebuilt binary

Prebuilt binaries for Linux (amd64) are published on the [GitHub Releases](../../releases) page as `hey_x.x.x_linux_amd64.tar.gz`. Download and extract the archive, then copy the `hey` binary somewhere on your `PATH`, for example:

```bash
tar -xzf hey_x.x.x_linux_amd64.tar.gz
cp hey ~/.local/bin/hey
```

### Build from source

Build from source using Cargo:

```bash
git clone <repository-url>
cd hey/hey
cargo build --release
```

The compiled binary will be available at `target/release/hey`. Copy it somewhere on your `PATH` to use it as the `hey` command, for example:

```bash
cp target/release/hey ~/.local/bin/hey
```

### Make it accessible

Add the `~/.local/bin` directory to your `$PATH` variable.

```bash
export PATH="$HOME/.local/bin:$PATH"

```

> [!TIP]
> Add this export routine to your `.bashrc` or `.zshrc` file

## Configuration

`hey` requires an OpenRouter API key, provided via the `OPENROUTER_API_KEY` environment variable:

```bash
export OPENROUTER_API_KEY="your-api-key-here"
```

> [!TIP]
> Add this export routine to your `.bashrc` or `.zshrc` file

You can get an API key from [openrouter.ai](https://openrouter.ai).

### Configuration file

You can override the default API URL and UI symbols by creating `~/.config/hey.toml`:

```toml
api_url = "https://openrouter.ai/api/v1/chat/completions"
prompt_marker = "› "
prompt_open = "●"
prompt_line = "╎"
prompt_done = "○"
prompt_close = "◉"
spinner_frames = ["◜", "◝", "◞", "◟"]
spinner_interval_ms = 120
```

All fields are optional; any field omitted from the file falls back to its default value shown above. If the file doesn't exist, `hey` uses the defaults as-is.

- `prompt_marker`: printed right before you type your question.
- `prompt_open`: printed on its own line above the question.
- `prompt_line`: printed at the start of every question/answer line, including code block borders.
- `prompt_done`: replaces the spinner once the answer is ready.
- `prompt_close`: printed on its own line to close the answer box.
- `spinner_frames`: the animation frames shown in place while waiting for a response.
- `spinner_interval_ms`: delay in milliseconds between spinner frames.

> [!NOTE]
> The API key is never read from `hey.toml`. It must always be set via the `OPENROUTER_API_KEY` environment variable.

### Buddies

_Buddies_ are named assistants, each with their own model and system prompt. Configure them in `~/.config/hey.toml` with one `[[buddies]]` entry per buddy:

```toml
[[buddies]]
name = "Tom"
default = true
model = "openai/gpt-4o-mini"
system_prompt = "your name is Tom and you are a helpful assistant that answers questions about command-line tools and commands (e.g. bash, ls, grep, cat, find, etc). keep answers concise and focused on cli usage."

[[buddies]]
name = "John"
model = "google/gemini-2.5-flash"
system_prompt = "your name is John and you are a helpful english teacher that can translate words and phrases and explain their meaning to the user who asked the question"
```

Select a buddy by passing its name (case-insensitive) as the first argument:

```bash
hey John
```

Running `hey` without a name uses the default buddy:

- The buddy marked `default = true`. If more than one is marked `default`, the first one wins.
- If none is marked `default`, the first buddy listed in `hey.toml`.
- If `hey.toml` doesn't exist, or has no `[[buddies]]` configured, a hardcoded default buddy is used (the same model and system prompt as before buddies existed).

Each `[[buddies]]` entry's `model` and `system_prompt` are optional and fall back to those same hardcoded defaults when omitted.

If you ask for a buddy name that isn't configured, `hey` exits with an error listing the available buddy names.

## Usage

Run `hey` and type your question at the prompt:

```bash
hey
```

To ask a specific buddy, pass its name (case-insensitive) as the first argument:

```bash
hey John
```

To ask a follow-up question that continues the previous answer, use the `-f` (or `--follow-up`) flag:

```bash
hey -f
```



> [!NOTE]
> Conversation history is kept separately per terminal, so using `-f` only continues the last conversation held in _that_ terminal — it won't pick up context from other terminals. Only the most recent conversation is remembered (no long-term history).

Use `-h` (or `--help`) to print usage information:

```bash
hey --help
```

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

Run `hey --version` (or `hey -V`) to check the version of your installed binary. See [CHANGELOG.md](CHANGELOG.md) for release history and [RELEASING.md](RELEASING.md) for the release process.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Contributing

See [AGENTS.md](AGENTS.md) for repo layout, build commands, and the release checklist. See [BACKLOG.md](BACKLOG.md) for planned work.
