//! Snap RGBA samples to the nearest palette color (Pixel It `convertPalette`).

use crate::color::similar_color;
use crate::error::Result;
use crate::image_buffer::{assert_rgba_image, clone_rgba_image};
use crate::types::{Rgb, RgbaImage};

/// Snap each opaque RGB sample to the nearest palette color.
///
/// Alpha is preserved. Mutates `image` in place.
///
/// # Errors
///
/// Returns an error if `image` is invalid or `palette` is empty.
pub fn convert_palette_in_place<'a>(
    image: &'a mut RgbaImage,
    palette: &[Rgb],
) -> Result<&'a mut RgbaImage> {
    assert_rgba_image(image, "image")?;
    if palette.is_empty() {
        return Err(crate::error::Error::EmptyPalette);
    }
    for pixel in image.data.chunks_exact_mut(4) {
        let matched = similar_color([pixel[0], pixel[1], pixel[2]], palette)?;
        pixel[0] = matched[0];
        pixel[1] = matched[1];
        pixel[2] = matched[2];
    }
    Ok(image)
}

/// Non-mutating palette snap.
///
/// # Errors
///
/// Propagates validation / empty-palette errors.
pub fn convert_palette(image: &RgbaImage, palette: &[Rgb]) -> Result<RgbaImage> {
    let mut out = clone_rgba_image(image)?;
    convert_palette_in_place(&mut out, palette)?;
    Ok(out)
}
