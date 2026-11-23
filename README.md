# image-to-pixel-art

**Try the reference web tool:** [Image to Pixel Art](https://www.uwarp.design/image-to-pixel-art)

Portable Rust library that turns RGBA image buffers into pixel art (Pixel It algorithm, MIT). Zero runtime dependencies beyond `std`.

Crate: **`image-to-pixel-art`** on [crates.io](https://crates.io/crates/image-to-pixel-art) (name reserved by availability check; **not published yet** — see [PUBLISH.md](./PUBLISH.md)).

Repository: <https://github.com/sherotree/image-to-pixel-art-rs>

## Install (after the first crates.io publish)

```toml
[dependencies]
image-to-pixel-art = "0.1"
```

Until published, depend on the GitHub repo:

```toml
[dependencies]
image-to-pixel-art = { git = "https://github.com/sherotree/image-to-pixel-art-rs" }
```

## Quick start

```rust
use image_to_pixel_art::{
    create_rgba_image, image_to_pixel_art, ImageToPixelArtOptions, PALETTES,
};

fn main() {
    let mut source = create_rgba_image(64, 64);
    // …fill source.data (width × height × 4)…

    let nes = PALETTES.iter().find(|p| p.name == "NES").unwrap();
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
    )
    .expect("pipeline");

    println!("{}×{} ({} bytes)", out.width, out.height, out.data.len());
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

- `block_size` — 1–50 (Pixel It scale; default 8)
- `grayscale` — average RGB after pixelation
- `use_palette` — snap to nearest palette color
- `palette` — `Vec<[u8; 3]>` (defaults to Pixel It default)
- `max_width` / `max_height` — optional output caps (height applied before width when both set)

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
cargo package --allow-dirty
cargo publish --dry-run
```

## License

MIT. Algorithm credit: [giventofly/pixelit](https://github.com/giventofly/pixelit). See [NOTICE](./NOTICE) and [LICENSE](./LICENSE).
