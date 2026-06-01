use std::path::Path;

use image::ImageReader;
use rayon::prelude::*;

/// Analyze an image file and compute the percentage of black pixels.
///
/// Black pixels are defined as RGB(0, 0, 0).
///
/// # Arguments
/// * `path` - Path to the image file
/// * `parallel` - Whether to use parallel processing (recommended for large images)
///
/// # Returns
/// * `Ok(percentage)` - Percentage of black pixels (0.0-100.0)
/// * `Err(e)` - Error if image cannot be read or decoded
///
/// # Examples
/// ```ignore
/// use black::black;
/// use std::path::Path;
///
/// let percentage = black(Path::new("image.png"), true).unwrap();
/// println!("Black pixels: {:.2}%", percentage);
/// ```
pub fn black(path: &Path, parallel: bool) -> Result<f64, image::ImageError> {
    let image = ImageReader::open(path)?.decode()?.into_rgb8();

    let (width, height) = image.dimensions();
    let total_pixels = (width * height) as f64;

    let black_pixels = if parallel {
        image
            .as_raw()
            .par_chunks_exact(3)
            .filter(|rgb| rgb[0] == 0 && rgb[1] == 0 && rgb[2] == 0)
            .count() as f64
    } else {
        image
            .as_raw()
            .chunks_exact(3)
            .filter(|rgb| rgb[0] == 0 && rgb[1] == 0 && rgb[2] == 0)
            .count() as f64
    };

    if total_pixels == 0.0 {
        return Ok(0.0);
    }

    Ok((black_pixels / total_pixels) * 100.0)
}

/// Check if a file path has a recognized image file extension.
///
/// Supported formats: PNG, JPG, JPEG, GIF, BMP, TIFF, WEBP.
///
/// # Arguments
/// * `path` - Path to check
///
/// # Returns
/// * `true` if the file has a recognized image extension
/// * `false` otherwise
pub fn is_image_file(path: &Path) -> bool {
    let image_extensions = ["png", "jpg", "jpeg", "gif", "bmp", "tiff", "webp"];

    match path.extension().and_then(|ext| ext.to_str()) {
        Some(extension) => image_extensions.contains(&extension.to_lowercase().as_str()),
        None => false,
    }
}
