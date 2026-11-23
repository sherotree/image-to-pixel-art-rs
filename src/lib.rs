//! Portable image → pixel-art core (Pixel It algorithm + hosted tool palettes).
//!
//! This crate ports the buffer algorithms from the TypeScript library
//! [`sherotree/image-to-pixel-art`](https://github.com/sherotree/image-to-pixel-art).
//! Upstream algorithm: [giventofly/pixelit](https://github.com/giventofly/pixelit) (MIT).
//!
//! Pipeline order: **pixelate → optional grayscale → optional palette snap → optional max size**.
//!
//! # Quick start
//!
//! ```
//! use image_to_pixel_art::{
//!     create_rgba_image, image_to_pixel_art, ImageToPixelArtOptions, PALETTES,
//! };
//!
//! let mut source = create_rgba_image(64, 64).unwrap();
//! // …fill source.data…
//!
//! let nes = PALETTES.iter().find(|p| p.name == "NES").unwrap();
//! let out = image_to_pixel_art(
//!     &source,
//!     &ImageToPixelArtOptions {
//!         block_size: Some(7),
//!         use_palette: true,
//!         palette: Some(nes.colors.to_vec()),
//!         max_width: Some(128),
//!         max_height: Some(128),
//!         ..Default::default()
//!     },
//! )
//! .unwrap();
//!
//! assert_eq!(out.data.len(), (out.width * out.height * 4) as usize);
//! ```
//!
//! # Buffer contract
//!
//! [`RgbaImage`] is row-major RGBA8 (`width * height * 4` bytes). All public
//! entry points validate dimensions and length before processing.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod clamp;
mod color;
mod convert_palette;
mod error;
mod grayscale;
mod image_buffer;
mod palette;
mod palette_utils;
mod pixelate;
mod process;
mod resize;
mod types;

pub use color::{color_sim, similar_color};
pub use convert_palette::{convert_palette, convert_palette_in_place};
pub use error::{Error, Result};
pub use grayscale::{convert_grayscale, convert_grayscale_in_place};
pub use image_buffer::{
    assert_rgba_image, clone_rgba_image, create_rgba_image, resize_bilinear, resize_nearest,
    sample_bilinear, sample_nearest,
};
pub use palette::{DEFAULT_PALETTE, PALETTES};
pub use palette_utils::{hex_to_rgb, parse_palette_text, rgb_to_hex};
pub use pixelate::{compute_pixelate_geometry, normalize_scale, pixelate_image};
pub use process::{image_to_pixel_art, image_to_pixel_art_copy};
pub use resize::resize_image;
pub use types::{ImageToPixelArtOptions, NamedPalette, PixelateGeometry, Rgb, RgbaImage};
