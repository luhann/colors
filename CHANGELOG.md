# Changelog

All notable changes to the Colors project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-01-XX

### Added
- **colorctl**: Unified CLI for all color manipulation tools
  - `hex` command: Convert hexadecimal colors to RGB format
  - `rgb` command: Convert RGB values to hexadecimal format
  - `black` command: Analyze black pixel percentage in images
- **hextorgb crate** (v0.6.0): Fast hex-to-RGB conversion library
  - `parse_hex()`: Parse hex color strings
  - `convert_hex_to_format()`: Convert to various output formats
  - Optional CLI feature with colored output preview
- **rgbtohex crate** (v0.1.0): RGB/RGBA-to-hex conversion library
  - `parse_rgb()`: Parse RGB color strings
  - `convert_rgb_to_format()`: Convert to various output formats
  - Mirrors hextorgb architecture for consistency
- **black crate** (v0.2.0): Image analysis library
  - `black()`: Compute black pixel percentage (parallel/sequential)
  - `is_image_file()`: Check if file is a supported image format
- **Documentation**: Library crate READMEs, root architecture overview
- **CI/CD**: GitHub Actions workflow with fmt, clippy, tests, audit
- **Linting**: rustfmt and clippy configuration files

### Changed
- **Workspace structure**: Reorganized to monorepo with `crates/` subdirectory
  - Single entry point: `colorsctl` binary
  - Library crates have optional CLI feature gates
  - Removed redundant standalones from hextorgb, rgbtohex, black

### Fixed
- **Workspace organization**: Removed binary naming collisions

### Testing
- 29 total tests across workspace
- hextorgb: 19 conversion tests
- rgbtohex: 6 conversion tests
- black: 4 integration tests with deterministic image generation

---

## Future Planning

### v0.2.0 (Planned)
- [ ] Legacy history import tooling (`scripts/git/*.sh`) and migration runbook (`docs/history-migration.md`)
- [ ] Additional color format conversions (HSL, HSV, CMYK)
- [ ] Batch processing support
- [ ] Configuration file support
- [ ] Extended image analysis features

### v1.0.0 (Milestone)
- [ ] Stable API guarantees
- [ ] Comprehensive documentation site
- [ ] Performance benchmarks
- [ ] Extended platform support
