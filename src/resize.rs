//! Proportional max-width / max-height caps (Pixel It `resizeImage`).

use crate::error::Result;
use crate::image_buffer::{assert_rgba_image, resize_nearest};
use crate::types::RgbaImage;

/// Proportional resize with max width / max height caps.
///
/// When both caps apply, height ratio overwrites width ratio (same precedence as Pixel It).
/// A missing or zero cap is ignored (JS falsy).
///
/// # Errors
///
/// Returns an error if `image` is invalid.
pub fn resize_image(
    image: &RgbaImage,
    max_width: Option<u32>,
    max_height: Option<u32>,
) -> Result<RgbaImage> {
    assert_rgba_image(image, "image")?;
    let max_width = max_width.filter(|&w| w > 0);
    let max_height = max_height.filter(|&h| h > 0);
    if max_width.is_none() && max_height.is_none() {
        return Ok(image.clone());
    }

    let mut ratio = 1.0_f64;
    if let Some(mw) = max_width {
        if image.width > mw {
            ratio = f64::from(mw) / f64::from(image.width);
        }
    }
    if let Some(mh) = max_height {
        if image.height > mh {
            ratio = f64::from(mh) / f64::from(image.height);
        }
    }
    if (ratio - 1.0).abs() < f64::EPSILON {
        return Ok(image.clone());
    }

    let w = ((f64::from(image.width) * ratio) + 0.5).floor().max(1.0) as u32;
    let h = ((f64::from(image.height) * ratio) + 0.5).floor().max(1.0) as u32;
    resize_nearest(image, w, h)
}
