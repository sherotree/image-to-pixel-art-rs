//! Average-RGB grayscale (Pixel It `convertGrayscale`).

use crate::clamp::clamp_u8;
use crate::image_buffer::{assert_rgba_image, clone_rgba_image, ImageBufferError};
use crate::types::RgbaImage;

/// Average RGB → grayscale. Mutates `image` in place and returns a mutable reference.
///
/// # Errors
///
/// Propagates validation errors from [`assert_rgba_image`].
pub fn convert_grayscale_in_place(image: &mut RgbaImage) -> Result<(), ImageBufferError> {
    assert_rgba_image(image, "image")?;
    let d = &mut image.data;
    let mut i = 0;
    while i + 3 < d.len() {
        let avg = (f64::from(d[i]) + f64::from(d[i + 1]) + f64::from(d[i + 2])) / 3.0;
        let g = clamp_u8(avg);
        d[i] = g;
        d[i + 1] = g;
        d[i + 2] = g;
        i += 4;
    }
    Ok(())
}

/// Non-mutating grayscale conversion.
///
/// # Errors
///
/// Propagates validation / clone errors.
pub fn convert_grayscale(image: &RgbaImage) -> Result<RgbaImage, ImageBufferError> {
    let mut out = clone_rgba_image(image)?;
    convert_grayscale_in_place(&mut out)?;
    Ok(out)
}
