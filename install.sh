#!/bin/sh
set -e

REPO="internodelabs/internode-cli"
BINARY="internode"
INSTALL_DIR="${HOME}/.local/bin"

detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Linux)  os="linux" ;;
        Darwin) os="macos" ;;
        MINGW*|MSYS*|CYGWIN*) os="windows" ;;
        *) echo "Unsupported OS: $OS" >&2; exit 1 ;;
    esac

    case "$ARCH" in
        x86_64|amd64)   arch="amd64" ;;
        aarch64|arm64)   arch="arm64" ;;
        *) echo "Unsupported architecture: $ARCH" >&2; exit 1 ;;
    esac

    echo "${os}-${arch}"
}

PLATFORM="$(detect_platform)"
echo "Detected platform: ${PLATFORM}"

if [ "$PLATFORM" = "windows-amd64" ] || [ "$PLATFORM" = "windows-arm64" ]; then
    BINARY="${BINARY}.exe"
fi

DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${BINARY}-${PLATFORM}"
echo "Downloading from ${DOWNLOAD_URL}..."

TMP_FILE="$(mktemp)"
trap 'rm -f "$TMP_FILE"' EXIT

if command -v curl >/dev/null 2>&1; then
    curl -fSL "$DOWNLOAD_URL" -o "$TMP_FILE"
elif command -v wget >/dev/null 2>&1; then
    wget -q "$DOWNLOAD_URL" -O "$TMP_FILE"
else
    echo "Error: curl or wget required." >&2
    exit 1
fi

mkdir -p "$INSTALL_DIR"
mv "$TMP_FILE" "${INSTALL_DIR}/${BINARY}"
chmod +x "${INSTALL_DIR}/${BINARY}"
trap - EXIT

echo ""
echo "Installed ${BINARY} to ${INSTALL_DIR}/${BINARY}"

case ":$PATH:" in
    *":${INSTALL_DIR}:"*) ;;
    *)
        echo ""
        echo "Add ${INSTALL_DIR} to your PATH:"
        echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
        echo ""
        echo "Add that line to your ~/.bashrc or ~/.zshrc to make it permanent."
        ;;
esac

echo ""
echo "Quick start:"
echo "  1. Get an API key from Settings > CLI API Key at https://app.internode.ai"
echo "  2. Run: ${BINARY} configure <your-api-key>"
echo "  3. Verify: ${BINARY} auth status"
echo ""
