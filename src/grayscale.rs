//! Average-RGB grayscale (Pixel It `convertGrayscale`).

use crate::clamp::to_u8_clamp;
use crate::error::Result;
use crate::image_buffer::{assert_rgba_image, clone_rgba_image};
use crate::types::RgbaImage;

/// Average RGB → grayscale. Mutates `image` in place and returns a mutable reference.
///
/// # Errors
///
/// Returns an error if `image` fails validation.
pub fn convert_grayscale_in_place(image: &mut RgbaImage) -> Result<&mut RgbaImage> {
    assert_rgba_image(image, "image")?;
    for pixel in image.data.chunks_exact_mut(4) {
        let avg = (f64::from(pixel[0]) + f64::from(pixel[1]) + f64::from(pixel[2])) / 3.0;
        let gray = to_u8_clamp(avg);
        pixel[0] = gray;
        pixel[1] = gray;
        pixel[2] = gray;
    }
    Ok(image)
}

/// Non-mutating grayscale conversion.
///
/// # Errors
///
/// Propagates validation errors from `image`.
pub fn convert_grayscale(image: &RgbaImage) -> Result<RgbaImage> {
    let mut out = clone_rgba_image(image)?;
    convert_grayscale_in_place(&mut out)?;
    Ok(out)
}
