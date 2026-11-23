//! Grayscale after pixelation.
//!
//! ```bash
//! cargo run --example grayscale
//! ```

use image_to_pixel_art::{create_rgba_image, image_to_pixel_art, ImageToPixelArtOptions};

fn main() {
    let mut source = create_rgba_image(32, 32);
    for px in source.data.chunks_exact_mut(4) {
        px[0] = 255;
        px[1] = 64;
        px[2] = 32;
        px[3] = 255;
    }

    let out = image_to_pixel_art(
        &source,
        &ImageToPixelArtOptions {
            block_size: Some(6),
            grayscale: true,
            ..Default::default()
        },
    )
    .expect("grayscale");

    println!(
        "grayscale pixel R=G=B={} alpha={}",
        out.data[0], out.data[3]
    );
}
