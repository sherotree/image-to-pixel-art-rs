//! Pixel It block-size geometry and bilinear→nearest pixelation.

use crate::clamp::js_round_nonneg;
use crate::image_buffer::{assert_rgba_image, resize_bilinear, resize_nearest, ImageBufferError};
use crate::types::RgbaImage;

/// Working scale and destination size for a pixelation pass.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PixelateGeometry {
    /// Effective work scale after optional >900 halving.
    pub work_scale: f64,
    /// Intermediate downscale width (may be fractional before rounding).
    pub scaled_width: f64,
    /// Intermediate downscale height (may be fractional before rounding).
    pub scaled_height: f64,
    /// Final upscale width (may be fractional before rounding).
    pub final_width: f64,
    /// Final upscale height (may be fractional before rounding).
    pub final_height: f64,
}

/// Normalize Pixel It block-size control (1–50) to the internal scale factor.
///
/// Invalid / missing values fall back to `0.08` (block size 8), matching Pixel It.
pub fn normalize_scale(block_size: Option<u32>) -> f64 {
    match block_size {
        Some(n) if (1..=50).contains(&n) => f64::from(n) * 0.01,
        _ => 0.08,
    }
}

/// Compute working scale and destination size using the same rules as Pixel It.
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
/// Geometry matches Pixel It; filtering approximates canvas `drawImage` (smooth down, crisp up).
///
/// # Errors
///
/// Propagates validation / resize errors.
pub fn pixelate_image(
    source: &RgbaImage,
    block_size: Option<u32>,
) -> Result<RgbaImage, ImageBufferError> {
    assert_rgba_image(source, "source")?;
    let g = compute_pixelate_geometry(source.width, source.height, block_size);

    let sw = js_round_nonneg(g.scaled_width).max(1);
    let sh = js_round_nonneg(g.scaled_height).max(1);
    let dw = js_round_nonneg(g.final_width).max(1);
    let dh = js_round_nonneg(g.final_height).max(1);

    let small = resize_bilinear(source, f64::from(sw), f64::from(sh))?;
    resize_nearest(&small, f64::from(dw), f64::from(dh))
}
