//! Snap to the built-in NES palette after pixelation.
//!
//! ```bash
//! cargo run --example palette
//! ```

use image_to_pixel_art::{create_rgba_image, image_to_pixel_art, ImageToPixelArtOptions, PALETTES};

fn main() {
    let mut source = create_rgba_image(64, 64);
    for (i, px) in source.data.chunks_exact_mut(4).enumerate() {
        px[0] = ((i * 17) % 256) as u8;
        px[1] = ((i * 29) % 256) as u8;
        px[2] = ((i * 47) % 256) as u8;
        px[3] = 255;
    }

    let nes = PALETTES
        .iter()
        .find(|p| p.name == "NES")
        .expect("NES palette");

    let out = image_to_pixel_art(
        &source,
        &ImageToPixelArtOptions {
            block_size: Some(7),
            use_palette: true,
            palette: Some(nes.colors.to_vec()),
            max_width: Some(128),
            max_height: Some(128),
            ..Default::default()
        },
    )
    .expect("pipeline");

    println!(
        "NES-snapped {}×{} first pixel [{}, {}, {}, {}]",
        out.width, out.height, out.data[0], out.data[1], out.data[2], out.data[3]
    );
}
