# image-to-pixel-art

Portable Rust library that turns RGBA image buffers into pixel art (Pixel It algorithm, MIT).

Rust port of the TypeScript library [`sherotree/image-to-pixel-art`](https://github.com/sherotree/image-to-pixel-art). Upstream algorithm: [giventofly/pixelit](https://github.com/giventofly/pixelit).

Package: **`image-to-pixel-art`** on [crates.io](https://crates.io/crates/image-to-pixel-art) (**not published yet** — see [SCOPE.md](./SCOPE.md)).

## Install (after the first crates.io publish)

```toml
[dependencies]
image-to-pixel-art = "0.1"
```

Until published, depend on the git repo:

```toml
[dependencies]
image-to-pixel-art = { git = "https://github.com/sherotree/image-to-pixel-art-rs" }
```

## Quick start (portable, no image decoder)

```rust
use image_to_pixel_art::{
    create_rgba_image, image_to_pixel_art, ImageToPixelArtOptions, PALETTES,
};

fn main() -> image_to_pixel_art::Result<()> {
    // Build or decode an RGBA buffer somehow (width × height × 4).
    let mut source = create_rgba_image(64, 64)?;
    // …fill source.data…

    let nes = PALETTES.iter().find(|p| p.name == "NES").expect("NES");
    let out = image_to_pixel_art(
        &source,
        &ImageToPixelArtOptions {
            block_size: Some(7),
            grayscale: false,
            use_palette: true,
            palette: Some(nes.colors.to_vec()),
            max_width: Some(128),
            max_height: Some(128),
        },
    )?;

    println!("{}×{} ({} bytes)", out.width, out.height, out.data.len());
    Ok(())
}
```

Pipeline order: **pixelate → optional grayscale → optional palette snap → optional max width/height**.

## API

| Export | Role |
| --- | --- |
| `image_to_pixel_art(source, options)` | Full pipeline on `RgbaImage` |
| `pixelate_image(source, block_size)` | Block-size pixelation only |
| `convert_grayscale` / `convert_palette` | Color steps |
| `resize_image(image, max_width, max_height)` | Proportional caps |
| `DEFAULT_PALETTE`, `PALETTES` | Built-in palettes |
| `hex_to_rgb`, `rgb_to_hex`, `parse_palette_text` | Palette file / hex helpers |
| `color_sim`, `similar_color` | Euclidean nearest-color match |
| `create_rgba_image`, `clone_rgba_image`, … | Buffer helpers |

### Options (`ImageToPixelArtOptions`)

- `block_size` — `Some(1..=50)` (Pixel It scale; default 8 when `None`/invalid)
- `grayscale` — average RGB after pixelation
- `use_palette` — snap to nearest palette color
- `palette` — `Option<Vec<Rgb>>` (defaults to Pixel It default)
- `max_width` / `max_height` — optional output caps (height applied before width when both set)

### Buffer contract

`RgbaImage { width, height, data }` is row-major RGBA8. `data.len()` must equal `width * height * 4`. Public entry points validate this and return `Result`.

## Examples

```bash
cargo run --example basic
cargo run --example palette
cargo run --example grayscale
```

## Checks

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets -- -D warnings
cargo doc --no-deps
cargo package
cargo publish --dry-run
```

## License

MIT. Algorithm credit: [giventofly/pixelit](https://github.com/giventofly/pixelit). TypeScript source: [sherotree/image-to-pixel-art](https://github.com/sherotree/image-to-pixel-art). See [NOTICE](./NOTICE) and [LICENSE](./LICENSE).
