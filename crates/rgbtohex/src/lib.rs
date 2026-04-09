//! # rgbtohex
//!
//! Fast, zero-dependency RGB-to-hex color conversion library.
//!
//! ## Features
//! - Parse RGB/RGBA color values
//! - Convert to multiple output formats (hex, CSS, JSON, compact)
//! - Optional CLI feature for command-line usage
//!
//! ## Examples
//!
//! ```ignore
//! use rgbtohex::convert_rgb_to_format;
//!
//! let result = convert_rgb_to_format("255,0,0", "hex").unwrap();
//! assert_eq!(result, "#FF0000");
//! ```

use std::fmt;

/// Represents an RGB or RGBA color value.
///
/// - `Rgb`: Three-component color (red, green, blue)
/// - `Rgba`: Four-component color with alpha transparency (0.0-1.0)
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Rgb { r: u8, g: u8, b: u8 },
    Rgba { r: u8, g: u8, b: u8, a: f64 },
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Rgb { r, g, b } => write!(f, "RGB({}, {}, {})", r, g, b),
            Color::Rgba { r, g, b, a } => write!(f, "RGBA({}, {}, {}, {:.2})", r, g, b, a),
        }
    }
}

/// Parse an RGB or RGBA color string into components and optional alpha channel.
///
/// Accepts format: `R,G,B` or `R,G,B,A` where components are 0-255 and alpha is 0.0-1.0.
///
/// # Arguments
/// * `rgb` - An RGB color string (e.g., `255,0,0` or `255,0,0,0.5`)
///
/// # Returns
/// * `Ok(([r, g, b], alpha))` - RGB triple and optional alpha value
/// * `Err(msg)` - Error message if parsing failed
///
/// # Examples
/// ```ignore
/// assert_eq!(parse_rgb("255,0,0"), Ok(([255, 0, 0], None)));
/// assert_eq!(parse_rgb("255,0,0,0.5"), Ok(([255, 0, 0], Some(0.5))));
/// ```
pub fn parse_rgb(rgb: &str) -> Result<([u8; 3], Option<f64>), &'static str> {
    let parts: Vec<&str> = rgb.split(',').map(|part| part.trim()).collect();

    if parts.len() != 3 && parts.len() != 4 {
        return Err("Invalid RGB format");
    }

    let r = parts[0].parse::<u8>().map_err(|_| "Invalid RGB component value")?;
    let g = parts[1].parse::<u8>().map_err(|_| "Invalid RGB component value")?;
    let b = parts[2].parse::<u8>().map_err(|_| "Invalid RGB component value")?;

    let alpha = if parts.len() == 4 {
        let a = parts[3].parse::<f64>().map_err(|_| "Invalid alpha value")?;
        if !(0.0..=1.0).contains(&a) {
            return Err("Alpha must be between 0.0 and 1.0");
        }
        Some(a)
    } else {
        None
    };

    Ok(([r, g, b], alpha))
}

/// Convert an RGB color string to various output formats.
///
/// Supports formats: `standard`, `css`, `json`, `hex`, `compact`.
///
/// # Arguments
/// * `rgb` - RGB color string (e.g., `255,0,0`)
/// * `format` - Output format to convert to
///
/// # Returns
/// * `Ok(formatted_string)` - Color in requested format
/// * `Err(msg)` - Error message if conversion failed
///
/// # Examples
/// ```ignore
/// assert_eq!(convert_rgb_to_format("255,0,0", "hex"), Ok("#FF0000".to_string()));
/// assert_eq!(convert_rgb_to_format("255,0,0", "standard"), Ok("RGB(255, 0, 0)".to_string()));
/// ```
pub fn convert_rgb_to_format(rgb: &str, format: &str) -> Result<String, String> {
    let (rgb, alpha) = parse_rgb(rgb).map_err(|e| e.to_string())?;

    let converted = match alpha {
        Some(a) => Color::Rgba { r: rgb[0], g: rgb[1], b: rgb[2], a },
        None => Color::Rgb { r: rgb[0], g: rgb[1], b: rgb[2] },
    };

    let output = match format {
        "standard" => converted.to_string(),
        "css" => match &converted {
            Color::Rgba { r, g, b, a } => format!("rgba({}, {}, {}, {:.2})", r, g, b, a),
            Color::Rgb { r, g, b } => format!("rgb({}, {}, {})", r, g, b),
        },
        "json" => match &converted {
            Color::Rgba { r, g, b, a } => {
                format!(r#"{{"r": {}, "g": {}, "b": {}, "a": {:.2}}}"#, r, g, b, a)
            }
            Color::Rgb { r, g, b } => format!(r#"{{"r": {}, "g": {}, "b": {}}}"#, r, g, b),
        },
        "hex" => match &converted {
            Color::Rgba { r, g, b, a } => {
                format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, (*a * 255.0) as u8)
            }
            Color::Rgb { r, g, b } => format!("#{:02X}{:02X}{:02X}", r, g, b),
        },
        "compact" => match &converted {
            Color::Rgba { r, g, b, a } => format!("{},{},{},{:.2}", r, g, b, a),
            Color::Rgb { r, g, b } => format!("{},{},{}", r, g, b),
        },
        _ => return Err(format!("Unknown format: {}", format)),
    };

    Ok(output)
}

#[cfg(feature = "cli")]
pub use clap::ValueEnum;

#[cfg(feature = "cli")]
#[derive(Clone, clap::ValueEnum)]
pub enum OutputFormat {
    Standard,
    Css,
    Json,
    Hex,
    Compact,
}

#[cfg(feature = "cli")]
impl OutputFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            OutputFormat::Standard => "standard",
            OutputFormat::Css => "css",
            OutputFormat::Json => "json",
            OutputFormat::Hex => "hex",
            OutputFormat::Compact => "compact",
        }
    }
}

#[cfg(feature = "cli")]
pub fn convert_with_format(
    rgb: &str,
    format: &OutputFormat,
    show_preview: bool,
) -> Result<String, String> {
    use colored::*;

    let mut output = convert_rgb_to_format(rgb, format.as_str())?;

    if show_preview {
        let (rgb, _) = parse_rgb(rgb).map_err(|e| e.to_string())?;
        let preview = "   ".on_truecolor(rgb[0], rgb[1], rgb[2]);
        output = format!("{} {}", preview, output);
    }

    Ok(output)
}

/// Converts an RGB color string to hex format
///
/// # Examples
/// ```
/// use rgbtohex::rgbtohex;
/// assert_eq!(rgbtohex("255,0,0"), "#FF0000");
/// ```
pub fn rgbtohex(rgb: &str) -> String {
    match convert_rgb_to_format(rgb, "hex") {
        Ok(hex) => hex,
        Err(e) => e,
    }
}
