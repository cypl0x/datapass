#!/usr/bin/env bash
# Install shell completions for datapass

set -e

BINARY="${1:-./target/release/datapass}"

if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found at $BINARY"
    echo "Usage: $0 [path-to-datapass-binary]"
    exit 1
fi

echo "Installing shell completions for datapass..."

# Bash
if [ -d "$HOME/.local/share/bash-completion/completions" ]; then
    echo "Installing bash completion..."
    "$BINARY" --generate-completions bash > "$HOME/.local/share/bash-completion/completions/datapass"
    echo "✓ Bash completion installed to ~/.local/share/bash-completion/completions/datapass"
elif [ -d "/etc/bash_completion.d" ] && [ -w "/etc/bash_completion.d" ]; then
    echo "Installing bash completion (system-wide)..."
    sudo "$BINARY" --generate-completions bash > /etc/bash_completion.d/datapass
    echo "✓ Bash completion installed to /etc/bash_completion.d/datapass"
fi

# Zsh
if command -v zsh &> /dev/null; then
    ZSH_COMP_DIR="${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/completions"
    if [ ! -d "$ZSH_COMP_DIR" ]; then
        ZSH_COMP_DIR="$HOME/.zsh/completions"
        mkdir -p "$ZSH_COMP_DIR"
    fi
    echo "Installing zsh completion..."
    "$BINARY" --generate-completions zsh > "$ZSH_COMP_DIR/_datapass"
    echo "✓ Zsh completion installed to $ZSH_COMP_DIR/_datapass"
    echo "  Add 'fpath=($ZSH_COMP_DIR \$fpath)' to your ~/.zshrc if needed"
fi

# Fish
if command -v fish &> /dev/null; then
    FISH_COMP_DIR="$HOME/.config/fish/completions"
    mkdir -p "$FISH_COMP_DIR"
    echo "Installing fish completion..."
    "$BINARY" --generate-completions fish > "$FISH_COMP_DIR/datapass.fish"
    echo "✓ Fish completion installed to $FISH_COMP_DIR/datapass.fish"
fi

echo ""
echo "Installation complete!"
echo "You may need to restart your shell or source your completion files."
