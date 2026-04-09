//! # black
//!
//! Analyze black pixel percentage in images using parallel processing.
//!
//! ## Features
//! - Fast parallel analysis using rayon
//! - Support for common image formats (PNG, JPEG, BMP, TIFF)
//! - Deterministic results for testing
//!
//! ## Examples
//!
//! ```ignore
//! use black::black;
//!
//! let percentage = black("image.png").unwrap();
//! println!("Black pixels: {:.2}%", percentage);
//! ```

mod black;

/// Analyze an image file and compute the percentage of black pixels.
///
/// Uses parallel processing for better performance on large images.
///
/// # Arguments
/// * `path` - Path to the image file
///
/// # Returns
/// * `Ok(percentage)` - Percentage of black pixels (0.0-100.0)
/// * `Err(msg)` - Error message if file cannot be processed
pub use black::{black, is_image_file};
