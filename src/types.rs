//! Shared types for RGB colors and RGBA image buffers.

/// RGB triplet in the 0–255 channel range.
pub type Rgb = [u8; 3];

/// RGBA raster buffer (row-major, 4 bytes per pixel).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RgbaImage {
    /// Width in pixels (≥ 1).
    pub width: u32,
    /// Height in pixels (≥ 1).
    pub height: u32,
    /// Length must be `width * height * 4`.
    pub data: Vec<u8>,
}

/// Named built-in or custom palette.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedPalette {
    /// Display name (matches the reference tool UI).
    pub name: &'static str,
    /// Palette colors.
    pub colors: &'static [Rgb],
}

/// Options for the high-level [`crate::image_to_pixel_art`] pipeline.
///
/// Mirrors the hosted tool controls (block size, grayscale, palette snap, max size).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageToPixelArtOptions {
    /// Block size / pixelation scale control, same as Pixel It `scale` (1–50).
    ///
    /// Internally mapped to `scale * 0.01`. Defaults to `8` when omitted / invalid.
    pub block_size: Option<u32>,
    /// When true, convert to grayscale after pixelation.
    pub grayscale: bool,
    /// When true, snap each pixel to the nearest palette color after pixelation.
    pub use_palette: bool,
    /// Palette used when `use_palette` is true. Defaults to Pixel It default palette.
    pub palette: Option<Vec<Rgb>>,
    /// Optional max width (px); applied after pixelation / color steps.
    pub max_width: Option<u32>,
    /// Optional max height (px); applied after pixelation / color steps.
    ///
    /// When both caps are set, height is applied before width (hosted UI order).
    pub max_height: Option<u32>,
}
