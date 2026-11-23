//! Full pipeline: pixelate → grayscale → palette → max size.

use crate::convert_palette::convert_palette_in_place;
use crate::error::Result;
use crate::grayscale::convert_grayscale_in_place;
use crate::image_buffer::{assert_rgba_image, clone_rgba_image};
use crate::palette::DEFAULT_PALETTE;
use crate::pixelate::pixelate_image;
use crate::resize::resize_image;
use crate::types::{ImageToPixelArtOptions, Rgb, RgbaImage};

/// Full pipeline matching the hosted tool / TypeScript library render order:
/// pixelate → optional grayscale → optional palette snap → optional max size.
///
/// Operates on [`RgbaImage`] buffers (no DOM / image decoder required).
///
/// # Errors
///
/// Returns an error on invalid buffers or an empty palette when `use_palette` is set.
pub fn image_to_pixel_art(
    source: &RgbaImage,
    options: &ImageToPixelArtOptions,
) -> Result<RgbaImage> {
    assert_rgba_image(source, "source")?;

    let mut out = pixelate_image(source, options.block_size)?;

    if options.grayscale {
        convert_grayscale_in_place(&mut out)?;
    }

    if options.use_palette {
        let palette: &[Rgb] = options
            .palette
            .as_deref()
            .unwrap_or(DEFAULT_PALETTE.as_slice());
        convert_palette_in_place(&mut out, palette)?;
    }

    if let Some(max_height) = options.max_height {
        if max_height > 0 {
            out = resize_image(&out, None, Some(max_height))?;
        }
    }
    if let Some(max_width) = options.max_width {
        if max_width > 0 {
            out = resize_image(&out, Some(max_width), None)?;
        }
    }

    Ok(out)
}

/// Same as [`image_to_pixel_art`], but starts from a deep copy of `source`.
///
/// # Errors
///
/// Propagates the same errors as [`image_to_pixel_art`].
pub fn image_to_pixel_art_copy(
    source: &RgbaImage,
    options: &ImageToPixelArtOptions,
) -> Result<RgbaImage> {
    let copy = clone_rgba_image(source)?;
    image_to_pixel_art(&copy, options)
}
