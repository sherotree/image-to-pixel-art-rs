//! Parity vectors against the TypeScript / Pixel It reference implementation.

use image_to_pixel_art::{
    color_sim, compute_pixelate_geometry, convert_grayscale_in_place, create_rgba_image,
    hex_to_rgb, image_to_pixel_art, normalize_scale, parse_palette_text, pixelate_image,
    resize_bilinear, resize_image, rgb_to_hex, similar_color, ImageToPixelArtOptions, RgbaImage,
    DEFAULT_PALETTE, PALETTES,
};

fn solid(w: u32, h: u32, r: u8, g: u8, b: u8, a: u8) -> RgbaImage {
    let mut img = create_rgba_image(w, h);
    for px in img.data.chunks_exact_mut(4) {
        px[0] = r;
        px[1] = g;
        px[2] = b;
        px[3] = a;
    }
    img
}

#[test]
fn normalize_scale_maps_1_to_50() {
    assert!((normalize_scale(Some(7)) - 0.07).abs() < 1e-12);
    assert!((normalize_scale(Some(50)) - 0.5).abs() < 1e-12);
    assert!((normalize_scale(Some(1)) - 0.01).abs() < 1e-12);
    assert!((normalize_scale(Some(0)) - 0.08).abs() < 1e-12);
    assert!((normalize_scale(Some(51)) - 0.08).abs() < 1e-12);
    assert!((normalize_scale(None) - 0.08).abs() < 1e-12);
}

#[test]
fn geometry_small_keeps_final_size() {
    let g = compute_pixelate_geometry(100, 80, Some(8));
    assert!((g.work_scale - 0.08).abs() < 1e-12);
    assert!((g.final_width - 100.0).abs() < 1e-12);
    assert!((g.final_height - 80.0).abs() < 1e-12);
}

#[test]
fn geometry_over_300_adds_border_compensation() {
    let g = compute_pixelate_geometry(400, 200, Some(10));
    assert!((g.work_scale - 0.1).abs() < 1e-12);
    assert!((g.final_width - (400.0 + 10.0 / 1.5)).abs() < 1e-9);
    assert!((g.final_height - 200.0).abs() < 1e-12);
}

#[test]
fn geometry_over_900_halves_work_scale() {
    let g = compute_pixelate_geometry(1000, 500, Some(8));
    assert!((g.work_scale - 0.04).abs() < 1e-12);
    assert!((g.scaled_width - 40.0).abs() < 1e-12);
    assert!((g.final_width - (1000.0 + 25.0 / 1.5)).abs() < 1e-9);
    assert!((g.final_height - 525.0).abs() < 1e-12);
}

#[test]
fn pixelate_rejects_mismatched_buffer() {
    let bad = RgbaImage {
        width: 2,
        height: 2,
        data: vec![0; 4],
    };
    assert!(pixelate_image(&bad, Some(8)).is_err());
}

#[test]
fn pixelate_1x1_preserves_color() {
    let mut src = create_rgba_image(1, 1);
    src.data = vec![12, 34, 56, 255];
    let out = pixelate_image(&src, Some(8)).unwrap();
    assert_eq!(out.width, 1);
    assert_eq!(out.height, 1);
    assert_eq!(out.data[0], 12);
    assert_eq!(out.data[3], 255);
}

#[test]
fn pixelate_checker_8x8_parity() {
    let mut chk = create_rgba_image(8, 8);
    for y in 0..8 {
        for x in 0..8 {
            let on = (x + y) % 2 == 0;
            let i = (y * 8 + x) * 4;
            chk.data[i] = if on { 255 } else { 0 };
            chk.data[i + 1] = if on { 0 } else { 255 };
            chk.data[i + 2] = if on { 128 } else { 64 };
            chk.data[i + 3] = 255;
        }
    }
    let out = pixelate_image(&chk, Some(4)).unwrap();
    assert_eq!(out.width, 8);
    assert_eq!(out.height, 8);
    // Golden: every pixel averages to [128,128,96,255]
    for px in out.data.chunks_exact(4) {
        assert_eq!(px, [128, 128, 96, 255]);
    }
}

#[test]
fn bilinear_4x4_to_2x2_parity() {
    let mut src = create_rgba_image(4, 4);
    for i in 0..16 {
        src.data[i * 4] = (i * 16) as u8;
        src.data[i * 4 + 1] = (255 - i * 16) as u8;
        src.data[i * 4 + 2] = ((i * 40) % 256) as u8;
        src.data[i * 4 + 3] = 255;
    }
    let out = resize_bilinear(&src, 2.0, 2.0).unwrap();
    assert_eq!(
        out.data,
        vec![40, 215, 100, 255, 72, 183, 116, 255, 168, 87, 100, 255, 200, 55, 116, 255]
    );
}

#[test]
fn grayscale_rounding_parity() {
    let mut g = create_rgba_image(1, 3);
    g.data = vec![1, 0, 0, 255, 2, 0, 0, 255, 4, 0, 0, 255];
    convert_grayscale_in_place(&mut g).unwrap();
    assert_eq!(g.data, vec![0, 0, 0, 255, 1, 1, 1, 255, 1, 1, 1, 255]);
}

#[test]
fn color_sim_and_tie_break() {
    assert_eq!(color_sim([10, 20, 30], [10, 20, 30]), 0.0);
    assert!((color_sim([0, 0, 0], [3, 4, 0]) - 5.0).abs() < 1e-12);
    let palette = [[0, 0, 0], [10, 0, 0], [20, 0, 0]];
    assert_eq!(similar_color([12, 0, 0], &palette).unwrap(), [10, 0, 0]);
    assert_eq!(similar_color([15, 0, 0], &palette).unwrap(), [20, 0, 0]);
    assert!(similar_color([1, 2, 3], &[]).is_err());
}

#[test]
fn resize_caps_parity() {
    let img = create_rgba_image(100, 50);
    let out = resize_image(&img, Some(50), None).unwrap();
    assert_eq!((out.width, out.height), (50, 25));

    let img = create_rgba_image(100, 100);
    let out = resize_image(&img, Some(50), Some(25)).unwrap();
    assert_eq!((out.width, out.height), (25, 25));
}

#[test]
fn palette_utils_parity() {
    assert_eq!(hex_to_rgb("#ff7755"), Some([255, 119, 85]));
    assert_eq!(hex_to_rgb("ff7755"), Some([255, 119, 85]));
    assert_eq!(hex_to_rgb("#fff"), None);
    assert_eq!(rgb_to_hex([0, 15, 255]), "#000fff");
    let text = "; Paint.NET Palette\nFF112233\n; skip\naabbcc, #ddeeff\ngarbage\n";
    assert_eq!(
        parse_palette_text(text),
        vec![[0x11, 0x22, 0x33], [0xaa, 0xbb, 0xcc], [0xdd, 0xee, 0xff]]
    );
}

#[test]
fn pipeline_grayscale_palette_and_caps() {
    let out = image_to_pixel_art(
        &solid(16, 16, 255, 0, 0, 255),
        &ImageToPixelArtOptions {
            block_size: Some(4),
            grayscale: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(out.data[0], out.data[1]);
    assert_eq!(out.data[1], out.data[2]);
    assert_eq!(out.data[0], 85);

    let out = image_to_pixel_art(
        &solid(16, 16, 140, 143, 174, 255),
        &ImageToPixelArtOptions {
            block_size: Some(4),
            use_palette: true,
            palette: Some(DEFAULT_PALETTE.to_vec()),
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!([out.data[0], out.data[1], out.data[2]], DEFAULT_PALETTE[0]);

    assert!(image_to_pixel_art(
        &solid(8, 8, 1, 2, 3, 255),
        &ImageToPixelArtOptions {
            use_palette: true,
            palette: Some(vec![]),
            ..Default::default()
        },
    )
    .is_err());

    let out = image_to_pixel_art(
        &solid(200, 100, 10, 20, 30, 255),
        &ImageToPixelArtOptions {
            block_size: Some(8),
            max_width: Some(50),
            max_height: Some(40),
            ..Default::default()
        },
    )
    .unwrap();
    assert!(out.width <= 50);
    assert!(out.height <= 40);

    let out = image_to_pixel_art(
        &solid(8, 8, 200, 50, 50, 128),
        &ImageToPixelArtOptions {
            block_size: Some(8),
            use_palette: true,
            palette: Some(vec![[255, 0, 0], [0, 0, 0]]),
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(out.data[3], 128);
}

#[test]
fn palettes_include_expected_names() {
    let names: Vec<_> = PALETTES.iter().map(|p| p.name).collect();
    assert!(names.contains(&"Pixelit Default"));
    assert!(names.contains(&"NES"));
    assert!(names.contains(&"Endesga 16"));
}
