//! Pixel It–compatible pixelation (bilinear down → nearest up).

use crate::error::Result;
use crate::image_buffer::{assert_rgba_image, resize_bilinear_f, resize_nearest_f};
use crate::types::{PixelateGeometry, RgbaImage};

/// Normalize Pixel It block-size control (1–50) to the internal scale factor.
///
/// Invalid values fall back to `0.08` (block size 8), matching Pixel It.
#[must_use]
pub fn normalize_scale(block_size: Option<u32>) -> f64 {
    match block_size {
        Some(n) if n > 0 && n <= 50 => f64::from(n) * 0.01,
        _ => 0.08,
    }
}

/// Compute working scale and destination size using the same rules as Pixel It
/// (`pixelate()` in giventofly/pixelit and the hosted tool port).
#[must_use]
pub fn compute_pixelate_geometry(
    width: u32,
    height: u32,
    block_size: Option<u32>,
) -> PixelateGeometry {
    let mut work_scale = normalize_scale(block_size);
    let mut scaled_width = f64::from(width) * work_scale;
    let mut scaled_height = f64::from(height) * work_scale;

    if width > 900 || height > 900 {
        work_scale *= 0.5;
        scaled_width = f64::from(width) * work_scale;
        scaled_height = f64::from(height) * work_scale;
    }

    let mut final_width = f64::from(width);
    if width > 300 {
        // Math.floor(width / (width * workScale)) == Math.floor(1 / workScale)
        let inv = (f64::from(width) / (f64::from(width) * work_scale)).floor();
        final_width += if width > height { inv / 1.5 } else { inv };
    }

    let mut final_height = f64::from(height);
    if height > 300 {
        let inv = (f64::from(height) / (f64::from(height) * work_scale)).floor();
        final_height += if height > width { inv / 1.5 } else { inv };
    }

    PixelateGeometry {
        work_scale,
        scaled_width,
        scaled_height,
        final_width,
        final_height,
    }
}

/// Portable pixelation: bilinear downsample → nearest-neighbor upsample.
///
/// Geometry matches Pixel It; filtering approximates canvas `drawImage`
/// (smooth down, crisp up).
///
/// # Errors
///
/// Returns an error if `source` fails validation or allocation overflows.
pub fn pixelate_image(source: &RgbaImage, block_size: Option<u32>) -> Result<RgbaImage> {
    assert_rgba_image(source, "source")?;
    let g = compute_pixelate_geometry(source.width, source.height, block_size);
    let small = resize_bilinear_f(source, g.scaled_width, g.scaled_height)?;
    resize_nearest_f(&small, g.final_width, g.final_height)
}
