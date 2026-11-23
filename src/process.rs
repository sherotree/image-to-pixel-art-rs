//! Full image → pixel-art pipeline.

use crate::convert_palette::{convert_palette_in_place, ConvertPaletteError};
use crate::grayscale::convert_grayscale_in_place;
use crate::image_buffer::{assert_rgba_image, clone_rgba_image, ImageBufferError};
use crate::palette::DEFAULT_PALETTE;
use crate::pixelate::pixelate_image;
use crate::resize::resize_image;
use crate::types::{ImageToPixelArtOptions, RgbaImage};

/// Errors from the high-level pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessError {
    /// Invalid image buffer or resize failure.
    Image(ImageBufferError),
    /// Palette conversion failure.
    Palette(ConvertPaletteError),
}

impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Image(e) => write!(f, "{e}"),
            Self::Palette(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ProcessError {}

impl From<ImageBufferError> for ProcessError {
    fn from(value: ImageBufferError) -> Self {
        Self::Image(value)
    }
}

impl From<ConvertPaletteError> for ProcessError {
    fn from(value: ConvertPaletteError) -> Self {
        Self::Palette(value)
    }
}

/// Full pipeline matching the hosted tool render order:
/// pixelate → optional grayscale → optional palette snap → optional max size.
///
/// Does not require I/O; operate on [`RgbaImage`] buffers.
///
/// # Errors
///
/// Returns [`ProcessError`] on invalid buffers or empty palettes when snapping.
pub fn image_to_pixel_art(
    source: &RgbaImage,
    options: &ImageToPixelArtOptions,
) -> Result<RgbaImage, ProcessError> {
    assert_rgba_image(source, "source")?;

    let mut out = pixelate_image(source, options.block_size)?;

    if options.grayscale {
        convert_grayscale_in_place(&mut out)?;
    }

    if options.use_palette {
        let palette: &[crate::types::Rgb] = match options.palette.as_deref() {
            Some(p) => p,
            None => &DEFAULT_PALETTE,
        };
        convert_palette_in_place(&mut out, palette)?;
    }

    if options.max_height.is_some() || options.max_width.is_some() {
        // Hosted UI applies height then width as separate passes when both are set.
        if let Some(mh) = options.max_height {
            if mh > 0 {
                out = resize_image(&out, None, Some(mh))?;
            }
        }
        if let Some(mw) = options.max_width {
            if mw > 0 {
                out = resize_image(&out, Some(mw), None)?;
            }
        }
    }

    Ok(out)
}

/// Same as [`image_to_pixel_art`], but starts from a deep copy of `source`.
///
/// # Errors
///
/// Same as [`image_to_pixel_art`].
pub fn image_to_pixel_art_copy(
    source: &RgbaImage,
    options: &ImageToPixelArtOptions,
) -> Result<RgbaImage, ProcessError> {
    let cloned = clone_rgba_image(source)?;
    image_to_pixel_art(&cloned, options)
}
