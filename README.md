# datapass

A fast, lightweight CLI tool to fetch and display mobile data usage from [datapass.de](https://datapass.de) (Telekom prepaid data monitoring).

## Features

- 📊 **Multiple output formats**: Human-readable, JSON, or single values
- 🎨 **Colored output**: Optional ANSI color support
- 📈 **Progress bar**: Visual representation of data usage
- 🔄 **Watch mode**: Auto-refresh TUI dashboard
- 🚀 **Fast and lightweight**: Self-contained binary with minimal dependencies
- 🔒 **Secure**: Uses rustls for TLS (no OpenSSL runtime dependency)
- 🌍 **Cross-platform**: Works on Linux, macOS, and more

## Important Note

**Network Requirement**: The datapass.de website is designed to be accessed from within the Telekom mobile network. When running the tool:

**Option 1:** Run on Telekom mobile network

```bash
# Works automatically when connected via Telekom mobile data
./datapass
```

**Option 2:** Use with saved HTML file (for testing)

```bash
# Save the page from your browser while on Telekom network, then:
./datapass --file saved-page.html
```

**Option 3 (Advanced):** Custom cookies

If you need to use specific cookies for authentication, you can provide them:

```bash
./datapass --cookie "your-custom-cookies-here"
```

## Installation

### Using Nix (recommended)

```bash
# Run directly
nix run github:cypl0x/datapass

# Install to profile
nix profile install github:cypl0x/datapass

# Build from source
git clone https://github.com/cypl0x/datapass
cd datapass
nix build
```

### Using Cargo

```bash
cargo install --git https://github.com/cypl0x/datapass
```

### From Source

```bash
git clone https://github.com/cypl0x/datapass
cd datapass
cargo build --release
# Binary will be at target/release/datapass
```

### Install Shell Completions and Man Page

```bash
# Install shell completions (bash, zsh, fish)
./scripts/install-completions.sh

# Install man page
./scripts/install-man.sh

# Or generate them manually
./target/release/datapass --generate-completions bash > ~/.local/share/bash-completion/completions/datapass
./target/release/datapass --generate-completions zsh > ~/.zsh/completions/_datapass
./target/release/datapass --generate-completions fish > ~/.config/fish/completions/datapass.fish
./target/release/datapass --generate-man > ~/.local/share/man/man1/datapass.1
```

## Usage

### Basic Usage

```bash
# Display human-readable output with progress bar
datapass

# With colors
datapass --color
```

### Output Formats

```bash
# JSON output
datapass --format json

# Single values (useful for scripting)
datapass --used        # Output: 12.64
datapass --total       # Output: 51.00
datapass --remaining   # Output: 38.36
datapass --percentage  # Output: 24.78
datapass --plan        # Output: MagentaMobil Prepaid L
```

### Watch Mode (TUI Dashboard)

```bash
# Refresh every 60 seconds
datapass --watch 60

# Refresh every 5 minutes
datapass --watch 300
```

The TUI dashboard displays:

- Plan name and validity date
- Data usage statistics with colored indicators
- Visual progress gauge
- Auto-refresh countdown

Controls:

- Press `q` or `ESC` to quit
- Press `r` to refresh immediately

### Advanced Options

```bash
# Use custom URL
datapass --url https://pass.telekom.de

# Read from local HTML file (for testing)
datapass --file test.html

# Enable verbose logging
datapass --verbose

# Log to file
datapass --log datapass.log
```

## Example Output

### Human-readable format

```
Plan: MagentaMobil Prepaid L
Valid until: 12. February 2026
Used:      12.64 GB (24.78%)
Total:     51.00 GB (100%)
Remaining: 38.36 GB (75.22%)
████████████████████████▓░░░░░░░░░░░░░░ 24.78%
```

### JSON format

```json
{
  "remaining_gb": 38.36,
  "total_gb": 51.0,
  "used_gb": 12.64,
  "percentage": 24.78,
  "plan_name": "MagentaMobil Prepaid L",
  "valid_until": "12. February 2026"
}
```

## Development

### Prerequisites

- Nix with flakes enabled, or
- Rust 1.70+ with Cargo

### Using Nix

```bash
# Enter development shell (automatically installs pre-commit hooks)
nix develop

# Build
nix build

# Run tests
nix develop --command cargo test

# Run all checks
nix flake check

# Format code
nix fmt
```

### Pre-commit Hooks

The project uses [git-hooks.nix](https://github.com/cachix/git-hooks.nix) to automatically run checks before commits:

- **Formatters**: rustfmt, alejandra (Nix), prettier, taplo (TOML)
- **Linters**: clippy, deadnix, statix
- **Tests**: cargo test

The hooks are automatically installed when you enter the Nix development shell with `nix develop`. They will run automatically before each commit to ensure code quality and catch issues early.

### Using Cargo

```bash
# Build
cargo build

# Run tests
cargo test

# Run with example file
cargo run -- --file "test/Data usage - MagentaMobil Prepaid L.html"

# Run linter
cargo clippy

# Format code
cargo fmt
```

## Project Structure

```
datapass/
├── src/
│   ├── main.rs       # Entry point
│   ├── lib.rs        # Library interface
│   ├── cli.rs        # CLI argument parsing
│   ├── parser.rs     # HTML parsing logic
│   ├── fetcher.rs    # HTTP fetching
│   ├── display.rs    # Output formatting
│   ├── tui.rs        # TUI implementation
│   ├── types.rs      # Data types
│   └── error.rs      # Error types
├── tests/
│   └── integration_tests.rs
├── test/
│   └── Data usage - MagentaMobil Prepaid L.html
├── Cargo.toml
├── flake.nix
└── README.md
```

## Testing

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_parse_test_file
```

## Cross-Compilation

Using Nix, you can easily build for multiple platforms:

```bash
# Build for x86_64 Linux
nix build .#packages.x86_64-linux.datapass

# Build for aarch64 Linux
nix build .#packages.aarch64-linux.datapass

# Build for macOS (Intel)
nix build .#packages.x86_64-darwin.datapass

# Build for macOS (Apple Silicon)
nix build .#packages.aarch64-darwin.datapass
```

## CI/CD

The project includes comprehensive GitHub Actions workflows that:

- ✅ Run all tests
- ✅ Check code formatting (rustfmt, alejandra for Nix)
- ✅ Run Clippy lints
- ✅ Check for dead Nix code (deadnix)
- ✅ Run Nix lints (statix)
- ✅ Build for multiple platforms
- ✅ Create releases with binaries for all platforms

## Language Support

The tool supports both **German** and **English** formats:

- Numbers: `38,36 GB` (German) and `38.36 GB` (English)
- Automatically handles both comma and period decimal separators
- Works with German (`Datennutzung`) and English (`Data usage`) page titles

## Use Cases

### Scripting

```bash
#!/bin/bash
# Alert when data usage exceeds 80%
USAGE=$(datapass --percentage)
if (( $(echo "$USAGE > 80" | bc -l) )); then
    notify-send "Data Usage Alert" "You've used $USAGE% of your data!"
fi
```

### Status Bar Integration

```bash
# i3status, polybar, etc.
datapass --remaining --color
```

### Monitoring

```bash
# Log usage every hour
while true; do
    echo "$(date): $(datapass --used) GB used" >> usage.log
    sleep 3600
done
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- CLI parsing with [clap](https://github.com/clap-rs/clap)
- HTML parsing with [scraper](https://github.com/causal-agent/scraper)
- TUI with [ratatui](https://github.com/ratatui-org/ratatui)
- Packaged with [Nix](https://nixos.org/)
