//! Proportional resize with max width / max height caps.

use crate::image_buffer::{assert_rgba_image, resize_nearest, ImageBufferError};
use crate::types::RgbaImage;

/// Proportional resize with max width / max height caps (Pixel It `resizeImage`).
///
/// When both caps apply, height ratio overwrites width ratio (same precedence as Pixel It).
///
/// # Errors
///
/// Propagates validation / resize errors.
pub fn resize_image(
    image: &RgbaImage,
    max_width: Option<u32>,
    max_height: Option<u32>,
) -> Result<RgbaImage, ImageBufferError> {
    assert_rgba_image(image, "image")?;
    if max_width.is_none() && max_height.is_none() {
        return Ok(image.clone());
    }

    let mut ratio = 1.0_f64;
    if let Some(mw) = max_width {
        if mw > 0 && image.width > mw {
            ratio = f64::from(mw) / f64::from(image.width);
        }
    }
    if let Some(mh) = max_height {
        if mh > 0 && image.height > mh {
            // Height overwrites width ratio when both would apply (Pixel It).
            ratio = f64::from(mh) / f64::from(image.height);
        }
    }
    if (ratio - 1.0).abs() < f64::EPSILON {
        return Ok(image.clone());
    }

    let w = f64::from(image.width) * ratio;
    let h = f64::from(image.height) * ratio;
    resize_nearest(image, w, h)
}
