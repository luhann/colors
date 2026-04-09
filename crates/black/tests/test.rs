use black::{black, is_image_file};
use image::{ImageBuffer, Rgb};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn write_test_image(filename: &str, pixels: &[[u8; 3]], width: u32, height: u32) -> String {
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);

    for (index, pixel) in pixels.iter().enumerate() {
        let x = (index as u32) % width;
        let y = (index as u32) / width;
        image.put_pixel(x, y, Rgb(*pixel));
    }

    let mut path = std::env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    path.push(format!("black-test-{}-{}.png", filename, nanos));
    image.save(&path).expect("test image should be written to temp dir");
    path.to_string_lossy().to_string()
}

#[test]
fn black_percentage_matches_expected_in_sequential_mode() {
    let path =
        write_test_image("sequential", &[[0, 0, 0], [255, 255, 255], [0, 0, 0], [255, 0, 0]], 2, 2);

    let percentage = black(Path::new(&path), false).expect("image should be processed");
    assert!((percentage - 50.0).abs() < f64::EPSILON);

    std::fs::remove_file(path).expect("temporary test image should be deleted");
}

#[test]
fn black_percentage_matches_expected_in_parallel_mode() {
    let path =
        write_test_image("parallel", &[[0, 0, 0], [255, 255, 255], [0, 0, 0], [255, 0, 0]], 2, 2);

    let percentage = black(Path::new(&path), true).expect("image should be processed");
    assert!((percentage - 50.0).abs() < f64::EPSILON);

    std::fs::remove_file(path).expect("temporary test image should be deleted");
}

#[test]
fn sequential_and_parallel_results_match() {
    let path = write_test_image(
        "parity",
        &[[0, 0, 0], [0, 0, 0], [0, 0, 0], [255, 255, 255], [255, 255, 255], [255, 255, 255]],
        3,
        2,
    );

    let sequential = black(Path::new(&path), false).expect("sequential path should succeed");
    let parallel = black(Path::new(&path), true).expect("parallel path should succeed");
    assert!((sequential - parallel).abs() < f64::EPSILON);

    std::fs::remove_file(path).expect("temporary test image should be deleted");
}

#[test]
fn is_image_file_identifies_supported_extensions_case_insensitively() {
    assert!(is_image_file(Path::new("sample.png")));
    assert!(is_image_file(Path::new("sample.JPEG")));
    assert!(is_image_file(Path::new("sample.WebP")));
    assert!(!is_image_file(Path::new("sample.txt")));
    assert!(!is_image_file(Path::new("sample")));
}
