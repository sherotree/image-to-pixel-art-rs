//! Shared types: RGB triplets, RGBA buffers, options, named palettes.

use crate::error::{Error, Result};

/// RGB triplet in the 0–255 channel range.
pub type Rgb = [u8; 3];

/// RGBA raster buffer (row-major, 4 bytes per pixel: R, G, B, A).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RgbaImage {
    /// Width in pixels (≥ 1).
    pub width: u32,
    /// Height in pixels (≥ 1).
    pub height: u32,
    /// Length must be `width * height * 4`.
    pub data: Vec<u8>,
}

impl RgbaImage {
    /// Create a validated buffer from parts.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidDimensions`] or [`Error::LengthMismatch`] when
    /// dimensions or length are inconsistent.
    pub fn from_parts(width: u32, height: u32, data: Vec<u8>) -> Result<Self> {
        let image = Self {
            width,
            height,
            data,
        };
        image.validate("image")?;
        Ok(image)
    }

    /// Validate dimensions and buffer length.
    ///
    /// # Errors
    ///
    /// Returns an error when width/height are zero or `data` length mismatches.
    pub fn validate(&self, label: &'static str) -> Result<()> {
        if self.width < 1 || self.height < 1 {
            return Err(Error::InvalidDimensions {
                label,
                message: "width and height must be >= 1",
            });
        }
        let expected = expected_len(self.width, self.height)?;
        if self.data.len() != expected {
            return Err(Error::LengthMismatch {
                label,
                actual: self.data.len(),
                expected,
            });
        }
        Ok(())
    }

    /// Pixel count as `usize`.
    #[must_use]
    pub fn pixel_count(&self) -> usize {
        (self.width as usize).saturating_mul(self.height as usize)
    }
}

/// Named built-in or custom palette.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedPalette {
    /// Display name (e.g. `"NES"`).
    pub name: &'static str,
    /// Palette colors.
    pub colors: &'static [Rgb],
}

/**
 * Options for the high-level [`crate::image_to_pixel_art`] pipeline.
 * Mirrors the hosted tool controls (block size, grayscale, palette snap, max size).
 */
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageToPixelArtOptions {
    /**
     * Block size / pixelation scale control, same as Pixel It `scale` (1–50).
     * Internally mapped to `scale * 0.01`. Defaults to `8` when `None` or invalid.
     */
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
    /// When both caps are set in the pipeline, height is applied before width.
    pub max_height: Option<u32>,
}

/// Geometry produced by Pixel It–compatible pixelation sizing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PixelateGeometry {
    /// Working scale after optional large-image halving.
    pub work_scale: f64,
    /// Intermediate downscale width (pre-round).
    pub scaled_width: f64,
    /// Intermediate downscale height (pre-round).
    pub scaled_height: f64,
    /// Final upscale width (pre-round; may be fractional).
    pub final_width: f64,
    /// Final upscale height (pre-round; may be fractional).
    pub final_height: f64,
}

/// Safe `width * height * 4` with overflow checks.
pub(crate) fn expected_len(width: u32, height: u32) -> Result<usize> {
    let w = width as usize;
    let h = height as usize;
    let pixels = w.checked_mul(h).ok_or(Error::InvalidDimensions {
        label: "image",
        message: "width * height overflows",
    })?;
    pixels.checked_mul(4).ok_or(Error::InvalidDimensions {
        label: "image",
        message: "width * height * 4 overflows",
    })
}
