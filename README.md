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

## Installation

### Using Nix (recommended)

```bash
# Run directly
nix run github:yourusername/datapass

# Install to profile
nix profile install github:yourusername/datapass

# Build from source
git clone https://github.com/yourusername/datapass
cd datapass
nix build
```

### Using Cargo

```bash
cargo install --git https://github.com/yourusername/datapass
```

### From Source

```bash
git clone https://github.com/yourusername/datapass
cd datapass
cargo build --release
# Binary will be at target/release/datapass
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

In watch mode:
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
  "plan_name": "MagentaMobil Prepaid L"
}
```

## Development

### Prerequisites

- Nix with flakes enabled, or
- Rust 1.70+ with Cargo

### Using Nix

```bash
# Enter development shell
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
