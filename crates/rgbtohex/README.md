# RGBtoHex

A fast and flexible command-line tool and Rust library for converting RGB color values to hex.

## Features

- Fast conversion with minimal allocations
- Accepts RGB and RGBA input as comma-separated values
- Multiple output formats: standard, CSS, JSON, hex, compact
- Optional terminal color preview in CLI mode
- Interactive mode for repeated conversions
- Library-friendly API for embedding in other tools

## Installation

### From workspace source

```bash
cargo install --path rgbtohex --features cli
```

### Library dependency (path)

```toml
[dependencies]
rgbtohex = { path = "../rgbtohex" }
```

## CLI Usage

### Basic conversion

```bash
rgbtohex "255,0,0"
# #FF0000

rgbtohex "255,0,0,0.5"
# #FF00007F
```

### Output formats

```bash
rgbtohex "255,0,0" --format standard
# RGB(255, 0, 0)

rgbtohex "255,0,0,0.5" --format css
# rgba(255, 0, 0, 0.50)

rgbtohex "255,0,0" --format json
# {"r": 255, "g": 0, "b": 0}

rgbtohex "255,0,0" --format compact
# 255,0,0
```

### Interactive mode

```bash
rgbtohex --interactive
```

### Color preview

```bash
rgbtohex "255,0,0" --preview
```

## Library Usage

```rust
use rgbtohex::{parse_rgb, rgbtohex, convert_rgb_to_format};

fn main() {
    let hex = rgbtohex("255,0,0");
    assert_eq!(hex, "#FF0000");

    let (rgb, alpha) = parse_rgb("255,0,0,0.5").unwrap();
    assert_eq!(rgb, [255, 0, 0]);
    assert_eq!(alpha, Some(0.5));

    let css = convert_rgb_to_format("255,0,0,0.5", "css").unwrap();
    assert_eq!(css, "rgba(255, 0, 0, 0.50)");
}
```

## Accepted Input

- RGB: r,g,b
- RGBA: r,g,b,a
- Spaces are allowed around commas and numbers
- r, g, b must be 0-255
- a must be 0.0-1.0

Examples:

- 255,0,0
- 0, 128, 255
- 255, 255, 255, 1.0
- 10,20,30,0.25

## Development

```bash
# Run tests
cargo test -p rgbtohex

# Run CLI binary from workspace
cargo run -p rgbtohex --features cli -- "255,0,0"
```

## License

MIT
