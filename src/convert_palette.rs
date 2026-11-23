//! Snap RGBA samples to the nearest palette color.

use crate::color::{similar_color, PaletteError};
use crate::image_buffer::{assert_rgba_image, clone_rgba_image, ImageBufferError};
use crate::types::{Rgb, RgbaImage};

/// Errors from palette conversion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConvertPaletteError {
    /// Invalid image buffer.
    Image(ImageBufferError),
    /// Empty palette.
    Palette(PaletteError),
}

impl std::fmt::Display for ConvertPaletteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Image(e) => write!(f, "{e}"),
            Self::Palette(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ConvertPaletteError {}

impl From<ImageBufferError> for ConvertPaletteError {
    fn from(value: ImageBufferError) -> Self {
        Self::Image(value)
    }
}

impl From<PaletteError> for ConvertPaletteError {
    fn from(value: PaletteError) -> Self {
        Self::Palette(value)
    }
}

/// Snap each opaque RGB sample to the nearest palette color (Pixel It `convertPalette`).
///
/// Alpha is preserved. Mutates `image` in place.
///
/// # Errors
///
/// Returns [`ConvertPaletteError`] on invalid buffers or an empty palette.
pub fn convert_palette_in_place(
    image: &mut RgbaImage,
    palette: &[Rgb],
) -> Result<(), ConvertPaletteError> {
    assert_rgba_image(image, "image")?;
    if palette.is_empty() {
        return Err(ConvertPaletteError::Palette(PaletteError));
    }
    let d = &mut image.data;
    let mut i = 0;
    while i + 3 < d.len() {
        let matched = similar_color([d[i], d[i + 1], d[i + 2]], palette)?;
        d[i] = matched[0];
        d[i + 1] = matched[1];
        d[i + 2] = matched[2];
        i += 4;
    }
    Ok(())
}

/// Non-mutating palette snap.
///
/// # Errors
///
/// Same as [`convert_palette_in_place`].
pub fn convert_palette(
    image: &RgbaImage,
    palette: &[Rgb],
) -> Result<RgbaImage, ConvertPaletteError> {
    let mut out = clone_rgba_image(image)?;
    convert_palette_in_place(&mut out, palette)?;
    Ok(out)
}
