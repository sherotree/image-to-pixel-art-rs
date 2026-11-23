//! Pixelate then snap to the NES palette.
//!
//! ```bash
//! cargo run --example palette
//! ```

use image_to_pixel_art::{
    create_rgba_image, image_to_pixel_art, rgb_to_hex, ImageToPixelArtOptions, PALETTES,
};

fn main() {
    let nes = PALETTES
        .iter()
        .find(|p| p.name == "NES")
        .expect("NES palette missing");

    let mut source = create_rgba_image(32, 32).expect("alloc");
    for px in source.data.chunks_exact_mut(4) {
        px[0] = 200;
        px[1] = 40;
        px[2] = 60;
        px[3] = 255;
    }

    let out = image_to_pixel_art(
        &source,
        &ImageToPixelArtOptions {
            block_size: Some(10),
            use_palette: true,
            palette: Some(nes.colors.to_vec()),
            ..Default::default()
        },
    )
    .expect("convert");

    let rgb = [out.data[0], out.data[1], out.data[2]];
    println!("snapped color {}", rgb_to_hex(rgb));
    println!("palette size {}", nes.colors.len());
}
