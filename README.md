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

You can get an API key from [openrouter.ai](https://openrouter.ai).

## Installation

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

## Requirements

- Rust toolchain (edition 2024)
- Internet connection
- An OpenRouter API key

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
