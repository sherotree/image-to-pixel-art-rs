//! RGBA buffer helpers: create, clone, validate, resize filters.

use crate::clamp::to_u8_clamp;
use crate::error::Result;
use crate::types::{expected_len, RgbaImage};

/// Create a zero-filled buffer (transparent black), matching `new Uint8ClampedArray(...)`.
///
/// Width/height are floored to integers and clamped to at least 1.
///
/// # Errors
///
/// Returns an error if `width * height * 4` would overflow `usize`.
pub fn create_rgba_image(width: u32, height: u32) -> Result<RgbaImage> {
    let w = width.max(1);
    let h = height.max(1);
    let len = expected_len(w, h)?;
    Ok(RgbaImage {
        width: w,
        height: h,
        data: vec![0; len],
    })
}

/// Shallow-validate buffer dimensions vs data length.
///
/// # Errors
///
/// Returns [`crate::Error`] when dimensions or length are invalid.
pub fn assert_rgba_image(image: &RgbaImage, label: &'static str) -> Result<()> {
    image.validate(label)
}

/// Deep-copy an RGBA buffer.
///
/// # Errors
///
/// Propagates validation errors from `image`.
pub fn clone_rgba_image(image: &RgbaImage) -> Result<RgbaImage> {
    assert_rgba_image(image, "image")?;
    Ok(image.clone())
}

/// Sample a pixel with nearest-neighbor and clamp coordinates.
#[must_use]
pub fn sample_nearest(image: &RgbaImage, x: f64, y: f64) -> [u8; 4] {
    let max_x = image.width.saturating_sub(1);
    let max_y = image.height.saturating_sub(1);
    let sx = x.floor().clamp(0.0, f64::from(max_x)) as u32;
    let sy = y.floor().clamp(0.0, f64::from(max_y)) as u32;
    let i = ((sy as usize) * (image.width as usize) + (sx as usize)) * 4;
    [
        image.data[i],
        image.data[i + 1],
        image.data[i + 2],
        image.data[i + 3],
    ]
}

/// Bilinear sample (downscale pass approximating canvas `drawImage` smoothing).
#[must_use]
pub fn sample_bilinear(image: &RgbaImage, x: f64, y: f64) -> [f64; 4] {
    let max_x = image.width.saturating_sub(1);
    let max_y = image.height.saturating_sub(1);
    let x0 = x.floor().clamp(0.0, f64::from(max_x)) as u32;
    let y0 = y.floor().clamp(0.0, f64::from(max_y)) as u32;
    let x1 = (x0 + 1).min(max_x);
    let y1 = (y0 + 1).min(max_y);
    let fx = x - f64::from(x0);
    let fy = y - f64::from(y0);

    let w = image.width as usize;
    let i00 = (y0 as usize * w + x0 as usize) * 4;
    let i10 = (y0 as usize * w + x1 as usize) * 4;
    let i01 = (y1 as usize * w + x0 as usize) * 4;
    let i11 = (y1 as usize * w + x1 as usize) * 4;

    let mut out = [0.0_f64; 4];
    for (channel, sample) in out.iter_mut().enumerate() {
        let v00 = f64::from(image.data[i00 + channel]);
        let v10 = f64::from(image.data[i10 + channel]);
        let v01 = f64::from(image.data[i01 + channel]);
        let v11 = f64::from(image.data[i11 + channel]);
        let top = v00 * (1.0 - fx) + v10 * fx;
        let bottom = v01 * (1.0 - fx) + v11 * fx;
        *sample = top * (1.0 - fy) + bottom * fy;
    }
    out
}

/// Resize with bilinear filtering (downscale-friendly).
///
/// # Errors
///
/// Returns an error if `source` is invalid or the destination size overflows.
pub fn resize_bilinear(source: &RgbaImage, width: u32, height: u32) -> Result<RgbaImage> {
    assert_rgba_image(source, "source")?;
    let w = round_u32_at_least_one(f64::from(width));
    let h = round_u32_at_least_one(f64::from(height));
    // Callers already pass integers; still clamp like TS `Math.max(1, Math.round(...))`.
    let w = w.max(1);
    let h = h.max(1);
    let mut out = create_rgba_image(w, h)?;
    let x_ratio = f64::from(source.width) / f64::from(w);
    let y_ratio = f64::from(source.height) / f64::from(h);
    for y in 0..h {
        for x in 0..w {
            let sample = sample_bilinear(
                source,
                (f64::from(x) + 0.5) * x_ratio - 0.5,
                (f64::from(y) + 0.5) * y_ratio - 0.5,
            );
            let i = ((y as usize) * (w as usize) + (x as usize)) * 4;
            out.data[i] = to_u8_clamp(sample[0]);
            out.data[i + 1] = to_u8_clamp(sample[1]);
            out.data[i + 2] = to_u8_clamp(sample[2]);
            out.data[i + 3] = to_u8_clamp(sample[3]);
        }
    }
    Ok(out)
}

/// Resize with nearest-neighbor (pixel-art upscale).
///
/// # Errors
///
/// Returns an error if `source` is invalid or the destination size overflows.
pub fn resize_nearest(source: &RgbaImage, width: u32, height: u32) -> Result<RgbaImage> {
    assert_rgba_image(source, "source")?;
    let w = round_u32_at_least_one(f64::from(width)).max(1);
    let h = round_u32_at_least_one(f64::from(height)).max(1);
    let mut out = create_rgba_image(w, h)?;
    let x_ratio = f64::from(source.width) / f64::from(w);
    let y_ratio = f64::from(source.height) / f64::from(h);
    for y in 0..h {
        for x in 0..w {
            let sample = sample_nearest(source, f64::from(x) * x_ratio, f64::from(y) * y_ratio);
            let i = ((y as usize) * (w as usize) + (x as usize)) * 4;
            out.data[i] = sample[0];
            out.data[i + 1] = sample[1];
            out.data[i + 2] = sample[2];
            out.data[i + 3] = sample[3];
        }
    }
    Ok(out)
}

/// `Math.round` for non-negative sizes used by the TS port, then at least 1.
pub(crate) fn round_dim(value: f64) -> u32 {
    round_u32_at_least_one(value).max(1)
}

fn round_u32_at_least_one(value: f64) -> u32 {
    if !value.is_finite() || value <= 0.0 {
        return 1;
    }
    // JS Math.round: half away from zero for positive numbers (.5 → up).
    let rounded = (value + 0.5).floor();
    if rounded >= f64::from(u32::MAX) {
        u32::MAX
    } else {
        rounded as u32
    }
}

/// Resize helper that accepts already-rounded positive integer dims from float geometry.
pub(crate) fn resize_bilinear_f(source: &RgbaImage, width: f64, height: f64) -> Result<RgbaImage> {
    resize_bilinear(source, round_dim(width), round_dim(height))
}

pub(crate) fn resize_nearest_f(source: &RgbaImage, width: f64, height: f64) -> Result<RgbaImage> {
    resize_nearest(source, round_dim(width), round_dim(height))
}
