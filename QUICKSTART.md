# Quick Start Guide

## Important Note

**Network Requirement**: When you run `datapass` without arguments, it fetches from datapass.de which requires an active Telekom mobile data connection.

For testing or development without Telekom network access, use the `--file` option with a saved HTML file.

## Building and Running

### Using Nix (Recommended)

```bash
# Enter development environment
nix develop

# Build the project
cargo build --release

# Run with test file
./target/release/datapass --file "test/Data usage - MagentaMobil Prepaid L.html"

# Build with Nix
nix build

# Run the Nix-built binary
./result/bin/datapass --file "test/Data usage - MagentaMobil Prepaid L.html"
```

### Using Cargo Only

```bash
# Build
cargo build --release

# Run
./target/release/datapass --file "test/Data usage - MagentaMobil Prepaid L.html"
```

## Shell Completions & Man Page

```bash
# Generate and install shell completions
./scripts/install-completions.sh

# Generate and install man page
./scripts/install-man.sh

# Or generate manually for your shell
./target/release/datapass --generate-completions bash
./target/release/datapass --generate-completions zsh
./target/release/datapass --generate-completions fish

# Generate man page
./target/release/datapass --generate-man > datapass.1
man ./datapass.1
```

## Quick Examples

```bash
# Human-readable output (default)
datapass

# With colors
datapass --color

# JSON output
datapass --format json

# Single values for scripting
datapass --used        # 22.12
datapass --total       # 25.00
datapass --remaining   # 2.88
datapass --percentage  # 88.48
datapass --plan        # MagentaMobil Prepaid L

# Watch mode (TUI dashboard, refresh every 60 seconds)
datapass --watch 60

# Use custom URL
datapass --url https://pass.telekom.de

# Enable logging
datapass --verbose --log datapass.log
```

## Development Commands

```bash
# Run tests
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt

# Run all Nix checks
nix flake check

# Format all files
nix fmt
```

## Testing the CLI

```bash
# Test with the provided HTML file
cargo run -- --file "test/Data usage - MagentaMobil Prepaid L.html"

# Test different output formats
cargo run -- --file "test/Data usage - MagentaMobil Prepaid L.html" --format json
cargo run -- --file "test/Data usage - MagentaMobil Prepaid L.html" --used
cargo run -- --file "test/Data usage - MagentaMobil Prepaid L.html" --color
```

## Expected Output

### Default Format

```
Plan: MagentaMobil Prepaid L
Valid until: 12. February 2026
Used:      22.12 GB (88.48%)
Total:     25.00 GB (100%)
Remaining: 2.88 GB (11.52%)
████████████████████████████████████▓░░░ 88.48%
```

### JSON Format

```json
{
  "remaining_gb": 2.88,
  "total_gb": 25.0,
  "used_gb": 22.12,
  "percentage": 88.48,
  "plan_name": "MagentaMobil Prepaid L",
  "valid_until": "12. February 2026"
}
```

## Project Structure

- `src/main.rs` - Entry point
- `src/lib.rs` - Library interface
- `src/cli.rs` - CLI argument parsing
- `src/parser.rs` - HTML parsing
- `src/fetcher.rs` - HTTP fetching
- `src/display.rs` - Output formatting
- `src/tui.rs` - TUI implementation
- `src/types.rs` - Data types
- `src/error.rs` - Error types
- `tests/integration_tests.rs` - Integration tests
- `flake.nix` - Nix flake configuration
- `.github/workflows/ci.yml` - GitHub Actions CI/CD

## Binary Size

The release binary is optimized for size:

- LTO enabled
- Code generation units = 1
- Stripped symbols
- Panic = abort

Expected size: ~5-10 MB (depending on platform)

## Supported Platforms

- x86_64-linux
- aarch64-linux
- i686-linux
- x86_64-darwin (macOS Intel)
- aarch64-darwin (macOS Apple Silicon)

## Notes

- The program uses rustls for TLS, so no OpenSSL runtime dependency is needed
- Watch mode updates automatically every N seconds
- In watch mode, press 'q' or ESC to quit, 'r' to refresh immediately
- All tests pass with the provided test HTML file
