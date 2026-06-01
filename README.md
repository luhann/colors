# Colors

A unified Rust workspace for color utilities and image analysis. Organized as a monorepo with specialized libraries for hex-to-RGB conversion, RGB-to-hex conversion, and image black-pixel analysis.

## Quick Start

### CLI Usage

```bash
# Convert hex to RGB
colorsctl hex "#FFFFFF"
# Output: RGB(255, 255, 255)

# Convert RGB to hex
colorsctl rgb "255,0,0"
# Output: #FF0000

# Analyze black pixels in image
colorsctl black "image.png"
# Output: Black pixels: 23.45%
```

### Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
hextorgb = "0.6.0"
rgbtohex = "0.1.0"
black = "0.2.0"
```

Example in Rust:

```rust
use hextorgb::convert_hex_to_format;
use rgbtohex::rgbtohex;
use black::black;
use std::path::Path;

// Hex to RGB (multiple formats)
let rgb = convert_hex_to_format("#FF0000", "standard")?;
assert_eq!(rgb, "RGB(255, 0, 0)");

let css = convert_hex_to_format("#FF0000", "css")?;
assert_eq!(css, "rgb(255, 0, 0)");

// RGB to Hex
let hex = rgbtohex("255,0,0");
assert_eq!(hex, "#FF0000");

// Analyze images
let black_percentage = black(Path::new("image.png"), true)?;
println!("Black pixels: {:.2}%", black_percentage);
```

## Installation

### From Source

Requires Rust 1.70 or later.

```bash
# Clone repository
git clone https://github.com/luhann/colors.git
cd colors

# Build release binary
cargo build --release -p colorctl

# Binary will be in target/release/colorctl
./target/release/colorctl --help
```

### As Library Dependency

For individual crates:

```bash
# Embed hextorgb in your project
cargo add hextorgb

# For other crates
cargo add rgbtohex black
```

## Crates

| Crate | Version | Purpose |
|-------|---------|---------|
| `colorctl` | 0.1.0 | Unified CLI dispatcher for all color tools |
| `hextorgb` | 0.6.0 | Fast hex-to-RGB conversion library |
| `rgbtohex` | 0.1.0 | RGB/RGBA-to-hex conversion library |
| `black` | 0.2.0 | Image analysis - compute black pixel percentages |

## Project Structure

```
colors/
├── Cargo.toml              # Root workspace metadata
├── CHANGELOG.md            # Release notes and version history
├── README.md               # This file
├── .github/
│   └── workflows/
│       └── rust.yml        # CI/CD pipeline (fmt, clippy, tests, audit)
├── crates/
│   ├── colorctl/           # Main CLI application
│   ├── black/              # Image analysis library
│   ├── hextorgb/           # Hex-to-RGB library
│   └── rgbtohex/           # RGB-to-hex library
└── target/                 # Cargo build output (shared)
```

## Stability Matrix

| Component | Stability | MSRV | Notes |
|-----------|-----------|------|-------|
| hextorgb  | Stable (0.6.x) | 1.70 | Mature hex parsing and formatting |
| rgbtohex  | Beta (0.1.x) | 1.70 | New; mirrors hextorgb architecture |
| black     | Beta (0.2.x) | 1.70 | Image format support depends on image crate |
| colorctl  | Beta (0.1.x) | 1.70 | CLI integration layer |

**Minimum Supported Rust Version (MSRV):** 1.70

To verify MSRV compliance:
```bash
rustup install 1.70
cargo +1.70 build --all --all-targets
```

## Development

### Common Commands

```bash
# Build default workspace member (colorctl)
cargo build

# Build all members
cargo build --all

# Build for release
cargo build --release

# Run CLI with arguments
cargo run -p colorctl -- hex "#FFFFFF"

# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p hextorgb
cargo test -p rgbtohex
cargo test -p black

# Check code formatting
cargo fmt --all --check

# Auto-format code
cargo fmt --all

# Lint with Clippy
cargo clippy --workspace --all-targets -- -D warnings

# Additional checks
cargo doc --open         # Generate and view docs
cargo audit              # Check for known vulnerabilities
```

### Adding New Color Converters

When adding a new conversion tool:

1. **Create crate**: `cargo new crates/yourconverter`
2. **Organize code**:
   - Keep parsing and conversion logic in `src/lib.rs`
   - Mark as `#[warn(missing_docs)]` for comprehensive documentation
   - Add features for optional CLI dependencies
3. **Add tests**: Create `tests/conversions.rs` with comprehensive test coverage
4. **Update colorctl**: Add new command variant to dispatcher in `crates/colorctl/src/main.rs`
5. **Document**: Include README in crate with examples
6. **Register**: Add to root `Cargo.toml` members list

### Code Quality

This project enforces:
- All tests passing: `cargo test --workspace`
- No compiler warnings: `cargo clippy -- -D warnings`
- Formatted code: `cargo fmt --check`
- Security audits: `cargo audit`

All checks run automatically in CI for pull requests.

## Testing

Test coverage by crate:

- **hextorgb**: 19 tests covering all hex parsing variations and output formats
- **rgbtohex**: 6 tests covering RGB/RGBA parsing and format conversions
- **black**: 4 integration tests with deterministic image generation

Run all tests:
```bash
cargo test --workspace -- --nocapture
```

Run with coverage (requires `cargo-tarpaulin`):
```bash
cargo tarpaulin --workspace --out Html
```

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please ensure:
- All tests pass: `cargo test --workspace`
- Code is formatted: `cargo fmt --all`
- Linting passes: `cargo clippy -- -D warnings`
- Documentation is complete for public APIs
- Changes are documented in CHANGELOG.md

## Roadmap

### v0.2.0 (Planned)
- Additional color format conversions (HSL, HSV, CMYK)
- Batch processing support for colorctl
- Configuration file support

### Future Enhancements
- Performance benchmarks and optimization
- Extended platform support
- WebAssembly builds for browser integration
