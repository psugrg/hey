# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Fenced code blocks in answers are now rendered with grey top/bottom borders showing the language, and the code itself in blue.

### Changed

- Extracted configuration handling (API key, AI model, color scheme) into a dedicated `config` module.
- Extracted answer rendering logic into a dedicated `render` module.
- Extracted the OpenRouter API client and "Thinking..." spinner logic into a dedicated `client` module.

## [0.1.0] - 2026-07-30

### Added

- Natural language Q&A for command-line tool questions, powered by [OpenRouter.ai](https://openrouter.ai).
- "Thinking..." spinner animation while waiting for the API response.
- Question prompt enclosed with top and bottom divider lines.
- `--version` / `-V` flag to print the current version.
