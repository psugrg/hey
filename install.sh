#!/bin/sh
# Installer for `hey`.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/psugrg/hey/main/install.sh | sh
#
# Environment variables:
#   VERSION      Version to install, e.g. "0.1.0" (defaults to the latest release)
#   INSTALL_DIR  Directory to install the binary into (defaults to "$HOME/.local/bin")

set -eu

REPO="psugrg/hey"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

log() {
  printf '%s\n' "$*"
}

err() {
  printf 'Error: %s\n' "$*" >&2
  exit 1
}

need_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    err "\`$1\` is required but was not found on PATH"
  fi
}

need_cmd curl
need_cmd tar
need_cmd uname
need_cmd mktemp

os="$(uname -s)"
arch="$(uname -m)"

case "$os" in
  Linux) ;;
  *) err "unsupported platform: only Linux/amd64 is currently supported (found: $os/$arch)" ;;
esac

case "$arch" in
  x86_64 | amd64) ;;
  *) err "unsupported platform: only Linux/amd64 is currently supported (found: $os/$arch)" ;;
esac

if [ -z "${VERSION:-}" ]; then
  log "Looking up the latest release..."
  latest_tag="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
    | grep '"tag_name":' \
    | head -n 1 \
    | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/')"

  if [ -z "$latest_tag" ]; then
    err "could not determine the latest release version"
  fi

  VERSION="${latest_tag#v}"
fi

asset_name="hey_${VERSION}_linux_amd64.tar.gz"
download_url="https://github.com/${REPO}/releases/download/v${VERSION}/${asset_name}"

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT INT TERM

log "Downloading hey ${VERSION}..."
if ! curl -fsSL "$download_url" -o "${tmp_dir}/${asset_name}"; then
  err "failed to download ${download_url}"
fi

log "Extracting..."
tar -xzf "${tmp_dir}/${asset_name}" -C "$tmp_dir"

if [ ! -f "${tmp_dir}/hey" ]; then
  err "the release archive did not contain a \`hey\` binary"
fi

mkdir -p "$INSTALL_DIR"
cp "${tmp_dir}/hey" "${INSTALL_DIR}/hey"
chmod +x "${INSTALL_DIR}/hey"

log "Installed hey ${VERSION} to ${INSTALL_DIR}/hey"

case ":${PATH}:" in
  *":${INSTALL_DIR}:"*) ;;
  *)
    log ""
    log "Note: ${INSTALL_DIR} is not on your PATH. Add it with:"
    log "  export PATH=\"${INSTALL_DIR}:\$PATH\""
    log "and add that line to your .bashrc or .zshrc to make it permanent."
    ;;
esac

if [ -z "${OPENROUTER_API_KEY:-}" ]; then
  log ""
  log "Note: the OPENROUTER_API_KEY environment variable is not set."
  log "hey needs an OpenRouter API key to work. Get one at https://openrouter.ai"
  log "then set it with:"
  log "  export OPENROUTER_API_KEY=\"your-api-key-here\""
  log "and add that line to your .bashrc or .zshrc to make it permanent."
fi
