//! Portable image → pixel-art core (Pixel It algorithm + hosted tool palettes).
//!
//! Reference tool: <https://www.uwarp.design/image-to-pixel-art>
//! Upstream algorithm: <https://github.com/giventofly/pixelit> (MIT)
//!
//! Pipeline order for [`image_to_pixel_art`]:
//! **pixelate → optional grayscale → optional palette snap → optional max size**.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod clamp;
mod color;
mod convert_palette;
mod grayscale;
mod image_buffer;
mod palette;
mod palette_utils;
mod pixelate;
mod process;
mod resize;
mod types;

pub use color::{color_sim, similar_color, PaletteError};
pub use convert_palette::{convert_palette, convert_palette_in_place, ConvertPaletteError};
pub use grayscale::{convert_grayscale, convert_grayscale_in_place};
pub use image_buffer::{
    assert_rgba_image, clone_rgba_image, create_rgba_image, resize_bilinear, resize_nearest,
    sample_bilinear, sample_nearest, ImageBufferError,
};
pub use palette::{DEFAULT_PALETTE, PALETTES};
pub use palette_utils::{hex_to_rgb, parse_palette_text, rgb_to_hex};
pub use pixelate::{compute_pixelate_geometry, normalize_scale, pixelate_image, PixelateGeometry};
pub use process::{image_to_pixel_art, image_to_pixel_art_copy, ProcessError};
pub use resize::resize_image;
pub use types::{ImageToPixelArtOptions, NamedPalette, Rgb, RgbaImage};
