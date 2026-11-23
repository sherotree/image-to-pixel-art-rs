//! Pixelate then convert to grayscale.
//!
//! ```bash
//! cargo run --example grayscale
//! ```

use image_to_pixel_art::{create_rgba_image, image_to_pixel_art, ImageToPixelArtOptions};

fn main() {
    let mut source = create_rgba_image(16, 16).expect("alloc");
    for px in source.data.chunks_exact_mut(4) {
        px[0] = 255;
        px[1] = 0;
        px[2] = 0;
        px[3] = 255;
    }

    let out = image_to_pixel_art(
        &source,
        &ImageToPixelArtOptions {
            block_size: Some(4),
            grayscale: true,
            ..Default::default()
        },
    )
    .expect("convert");

    let r = out.data[0];
    let g = out.data[1];
    let b = out.data[2];
    println!("grayscale channels r={r} g={g} b={b}");
    println!("equal? {}", r == g && g == b);
}
