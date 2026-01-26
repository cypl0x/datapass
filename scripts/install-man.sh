#!/usr/bin/env bash
# Install man page for datapass

set -e

BINARY="${1:-./target/release/datapass}"

if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found at $BINARY"
    echo "Usage: $0 [path-to-datapass-binary]"
    exit 1
fi

echo "Installing man page for datapass..."

# Try user-local man directory first
MAN_DIR="$HOME/.local/share/man/man1"
if [ ! -d "$MAN_DIR" ]; then
    mkdir -p "$MAN_DIR"
fi

"$BINARY" --generate-man > "$MAN_DIR/datapass.1"
echo "✓ Man page installed to $MAN_DIR/datapass.1"

# Update man database if possible
if command -v mandb &> /dev/null; then
    mandb -q "$HOME/.local/share/man" 2>/dev/null || true
fi

echo ""
echo "Installation complete!"
echo "Try: man datapass"
echo ""
echo "If 'man datapass' doesn't work, add this to your ~/.bashrc or ~/.zshrc:"
echo "  export MANPATH=\"\$HOME/.local/share/man:\$MANPATH\""
