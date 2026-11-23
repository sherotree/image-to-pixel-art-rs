//! Pixelate a solid RGBA buffer.
//!
//! ```bash
//! cargo run --example basic
//! ```

use image_to_pixel_art::{create_rgba_image, image_to_pixel_art, ImageToPixelArtOptions};

fn main() {
    let mut source = create_rgba_image(64, 48);
    for px in source.data.chunks_exact_mut(4) {
        px[0] = 200;
        px[1] = 100;
        px[2] = 50;
        px[3] = 255;
    }

    let out = image_to_pixel_art(
        &source,
        &ImageToPixelArtOptions {
            block_size: Some(7),
            ..Default::default()
        },
    )
    .expect("pixelate");

    println!(
        "pixelated {}×{} ({} bytes)",
        out.width,
        out.height,
        out.data.len()
    );
}
