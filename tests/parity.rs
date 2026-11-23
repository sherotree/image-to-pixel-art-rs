//! Parity vectors mirrored from the TypeScript library tests.

use image_to_pixel_art::{
    color_sim, compute_pixelate_geometry, convert_grayscale, create_rgba_image, hex_to_rgb,
    image_to_pixel_art, normalize_scale, parse_palette_text, pixelate_image, resize_image,
    rgb_to_hex, similar_color, ImageToPixelArtOptions, DEFAULT_PALETTE, PALETTES,
};

fn solid(w: u32, h: u32, r: u8, g: u8, b: u8, a: u8) -> image_to_pixel_art::RgbaImage {
    let mut img = create_rgba_image(w, h).unwrap();
    for px in img.data.chunks_exact_mut(4) {
        px[0] = r;
        px[1] = g;
        px[2] = b;
        px[3] = a;
    }
    img
}

#[test]
fn color_sim_identical_is_zero() {
    assert!((color_sim([10, 20, 30], [10, 20, 30])).abs() < f64::EPSILON);
}

#[test]
fn color_sim_known_distance() {
    // sqrt(3^2 + 4^2 + 0) = 5
    assert!((color_sim([0, 0, 0], [3, 4, 0]) - 5.0).abs() < 1e-9);
}

#[test]
fn similar_color_nearest_and_tie_prefers_later() {
    let palette = [[0, 0, 0], [10, 0, 0], [20, 0, 0]];
    assert_eq!(similar_color([12, 0, 0], &palette).unwrap(), [10, 0, 0]);
    // Exact midpoint between [10] and [20]: distance 5 either way → later wins
    assert_eq!(similar_color([15, 0, 0], &palette).unwrap(), [20, 0, 0]);
}

#[test]
fn similar_color_empty_palette_errors() {
    let err = similar_color([1, 2, 3], &[]).unwrap_err();
    assert!(err
        .to_string()
        .contains("Palette must contain at least one color"));
}

#[test]
fn normalize_scale_mapping() {
    assert!((normalize_scale(Some(7)) - 0.07).abs() < 1e-12);
    assert!((normalize_scale(Some(50)) - 0.5).abs() < 1e-12);
    assert!((normalize_scale(Some(1)) - 0.01).abs() < 1e-12);
    assert!((normalize_scale(Some(0)) - 0.08).abs() < 1e-12);
    assert!((normalize_scale(Some(51)) - 0.08).abs() < 1e-12);
    assert!((normalize_scale(None) - 0.08).abs() < 1e-12);
}

#[test]
fn compute_geometry_small_image() {
    let g = compute_pixelate_geometry(100, 80, Some(8));
    assert!((g.work_scale - 0.08).abs() < 1e-12);
    assert!((g.final_width - 100.0).abs() < 1e-12);
    assert!((g.final_height - 80.0).abs() < 1e-12);
}

#[test]
fn compute_geometry_border_compensation() {
    let g = compute_pixelate_geometry(400, 200, Some(10));
    // workScale 0.1; width>height → floor(1/0.1)/1.5 = 10/1.5
    assert!((g.work_scale - 0.1).abs() < 1e-12);
    assert!((g.final_width - (400.0 + 10.0 / 1.5)).abs() < 1e-9);
    assert!((g.final_height - 200.0).abs() < 1e-12);
}

#[test]
fn compute_geometry_large_halves_scale() {
    let g = compute_pixelate_geometry(1000, 500, Some(8));
    assert!((g.work_scale - 0.04).abs() < 1e-12);
}

#[test]
fn pixelate_rejects_bad_buffers() {
    assert!(pixelate_image(
        &image_to_pixel_art::RgbaImage {
            width: 0,
            height: 1,
            data: vec![],
        },
        Some(8)
    )
    .is_err());
    assert!(pixelate_image(
        &image_to_pixel_art::RgbaImage {
            width: 2,
            height: 2,
            data: vec![0; 4],
        },
        Some(8)
    )
    .is_err());
}

#[test]
fn pixelate_1x1_preserves_channels() {
    let mut src = create_rgba_image(1, 1).unwrap();
    src.data = vec![12, 34, 56, 255];
    let out = pixelate_image(&src, Some(8)).unwrap();
    assert_eq!(out.width, 1);
    assert_eq!(out.height, 1);
    assert_eq!(out.data[0], 12);
    assert_eq!(out.data[3], 255);
}

#[test]
fn pixelate_output_length_matches() {
    let src = create_rgba_image(64, 48).unwrap();
    let out = pixelate_image(&src, Some(7)).unwrap();
    assert_eq!(out.data.len(), (out.width * out.height * 4) as usize);
}

#[test]
fn resize_no_caps_keeps_size() {
    let img = create_rgba_image(10, 10).unwrap();
    let out = resize_image(&img, None, None).unwrap();
    assert_eq!(out.width, 10);
    assert_eq!(out.height, 10);
}

#[test]
fn resize_max_width_only() {
    let img = create_rgba_image(100, 50).unwrap();
    let out = resize_image(&img, Some(50), None).unwrap();
    assert_eq!(out.width, 50);
    assert_eq!(out.height, 25);
}

#[test]
fn resize_height_ratio_wins_when_both() {
    let img = create_rgba_image(100, 100).unwrap();
    // width would want 0.5, height wants 0.25 → height wins
    let out = resize_image(&img, Some(50), Some(25)).unwrap();
    assert_eq!(out.width, 25);
    assert_eq!(out.height, 25);
}

#[test]
fn resize_within_caps_noop_size() {
    let img = create_rgba_image(20, 20).unwrap();
    let out = resize_image(&img, Some(100), Some(100)).unwrap();
    assert_eq!(out.width, 20);
    assert_eq!(out.height, 20);
}

#[test]
fn grayscale_average_channels() {
    let img = solid(1, 1, 255, 0, 0, 255);
    let out = convert_grayscale(&img).unwrap();
    // (255+0+0)/3 = 85 → Uint8ClampedArray store
    assert_eq!(out.data[0], 85);
    assert_eq!(out.data[1], 85);
    assert_eq!(out.data[2], 85);
    assert_eq!(out.data[3], 255);
}

#[test]
fn pipeline_grayscale_equal_channels() {
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
}

#[test]
fn pipeline_palette_lands_on_entry() {
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
}

#[test]
fn pipeline_empty_palette_errors() {
    let err = image_to_pixel_art(
        &solid(8, 8, 1, 2, 3, 255),
        &ImageToPixelArtOptions {
            use_palette: true,
            palette: Some(vec![]),
            ..Default::default()
        },
    )
    .unwrap_err();
    assert!(err.to_string().contains("Palette"));
}

#[test]
fn pipeline_max_caps() {
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
}

#[test]
fn pipeline_preserves_alpha_on_palette() {
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

#[test]
fn hex_roundtrip_and_invalid() {
    assert_eq!(hex_to_rgb("#ff7755"), Some([255, 119, 85]));
    assert_eq!(hex_to_rgb("ff7755"), Some([255, 119, 85]));
    assert_eq!(hex_to_rgb(""), None);
    assert_eq!(hex_to_rgb("#fff"), None);
    assert_eq!(hex_to_rgb("notahex"), None);
    assert_eq!(rgb_to_hex([0, 15, 255]), "#000fff");
}

#[test]
fn parse_palette_text_parity() {
    let text = "; Paint.NET Palette\nFF112233\n; skip\naabbcc, #ddeeff\ngarbage\n";
    assert_eq!(
        parse_palette_text(text),
        vec![[0x11, 0x22, 0x33], [0xaa, 0xbb, 0xcc], [0xdd, 0xee, 0xff]]
    );
    assert!(parse_palette_text("; only\n; comments").is_empty());
    assert!(parse_palette_text("").is_empty());
}
