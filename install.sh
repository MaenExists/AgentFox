#!/usr/bin/env bash

# AgentFox Installer
# One-line install: curl -sSL https://raw.githubusercontent.com/user/AgentFox/main/install.sh | bash

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}==>${NC} Installing ${GREEN}AgentFox${NC}..."

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error:${NC} Rust/Cargo is not installed. Please install it from https://rustup.rs/"
    exit 1
fi

# Check for WebKitGTK (Linux)
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if ! pkg-config --exists webkit2gtk-4.1 && ! pkg-config --exists webkit2gtk-4.0; then
        echo -e "${RED}Warning:${NC} WebKitGTK development headers not found."
        echo -e "On Ubuntu/Debian: ${BLUE}sudo apt install libwebkit2gtk-4.1-dev${NC}"
    fi
fi

# Build
echo -e "${BLUE}==>${NC} Building binaries (release)..."
cargo build --release --quiet

# Install
INSTALL_DIR="${HOME}/.local/bin"
mkdir -p "$INSTALL_DIR"

cp target/release/afox "$INSTALL_DIR/afox"
cp target/release/afoxd "$INSTALL_DIR/afoxd"

echo -e "${BLUE}==>${NC} Installation successful!"
echo -e "${GREEN}AgentFox binaries installed to:${NC} $INSTALL_DIR"

# Check PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${RED}Note:${NC} $INSTALL_DIR is not in your PATH."
    echo -e "Add this to your .bashrc or .zshrc:"
    echo -e "  export PATH=\"\$PATH:$INSTALL_DIR\""
fi

echo -e "\n${BLUE}Try it out:${NC}"
echo -e "  ${GREEN}afoxd &${NC}          # Start the daemon"
echo -e "  ${GREEN}afox search google.com${NC} # Run a command"
