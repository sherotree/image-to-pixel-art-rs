//! Pixelate a synthetic gradient without an image decoder.
//!
//! ```bash
//! cargo run --example basic
//! ```

use image_to_pixel_art::{
    compute_pixelate_geometry, create_rgba_image, image_to_pixel_art, ImageToPixelArtOptions,
};

fn main() {
    const W: u32 = 120;
    const H: u32 = 80;
    let mut source = create_rgba_image(W, H).expect("alloc");

    for y in 0..H {
        for x in 0..W {
            let i = ((y * W + x) * 4) as usize;
            source.data[i] = ((f64::from(x) / f64::from(W)) * 255.0).floor() as u8;
            source.data[i + 1] = ((f64::from(y) / f64::from(H)) * 255.0).floor() as u8;
            source.data[i + 2] = 128;
            source.data[i + 3] = 255;
        }
    }

    let block_size = 7_u32;
    let geom = compute_pixelate_geometry(W, H, Some(block_size));
    let out = image_to_pixel_art(
        &source,
        &ImageToPixelArtOptions {
            block_size: Some(block_size),
            ..Default::default()
        },
    )
    .expect("pixelate");

    println!("blockSize {block_size}");
    println!("geometry {geom:?}");
    println!(
        "output width={} height={} bytes={}",
        out.width,
        out.height,
        out.data.len()
    );
    println!(
        "sample pixel [0] {} {} {} {}",
        out.data[0], out.data[1], out.data[2], out.data[3]
    );
}
