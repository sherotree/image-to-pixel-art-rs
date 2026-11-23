//! RGBA buffer helpers and resize filters.

use crate::clamp::{clamp_u8, js_round_nonneg};
use crate::types::RgbaImage;

/// Error returned when an RGBA buffer fails validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageBufferError {
    message: String,
}

impl ImageBufferError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ImageBufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ImageBufferError {}

/// Create an empty zeroed buffer (same as the TypeScript `createRgbaImage`).
///
/// Dimensions are floored and clamped to at least 1×1.
pub fn create_rgba_image(width: u32, height: u32) -> RgbaImage {
    let w = width.max(1);
    let h = height.max(1);
    let len = (w as usize)
        .checked_mul(h as usize)
        .and_then(|n| n.checked_mul(4))
        .expect("image buffer size overflow");
    RgbaImage {
        width: w,
        height: h,
        data: vec![0u8; len],
    }
}

/// Shallow-validate buffer dimensions vs data length.
///
/// # Errors
///
/// Returns [`ImageBufferError`] when width/height are zero or the byte length mismatches.
pub fn assert_rgba_image(image: &RgbaImage, label: &str) -> Result<(), ImageBufferError> {
    if image.width < 1 || image.height < 1 {
        return Err(ImageBufferError::new(format!(
            "{label}: width and height must be >= 1"
        )));
    }
    let expected = (image.width as usize)
        .checked_mul(image.height as usize)
        .and_then(|n| n.checked_mul(4))
        .ok_or_else(|| ImageBufferError::new(format!("{label}: size overflow")))?;
    if image.data.len() != expected {
        return Err(ImageBufferError::new(format!(
            "{label}: data length {} !== {expected}",
            image.data.len()
        )));
    }
    Ok(())
}

/// Deep-copy an RGBA buffer.
///
/// # Errors
///
/// Propagates validation errors from [`assert_rgba_image`].
pub fn clone_rgba_image(image: &RgbaImage) -> Result<RgbaImage, ImageBufferError> {
    assert_rgba_image(image, "image")?;
    Ok(RgbaImage {
        width: image.width,
        height: image.height,
        data: image.data.clone(),
    })
}

/// Sample a pixel with nearest-neighbor and clamp coordinates.
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

/// Bilinear sample (used for the downscale pass to approximate canvas `drawImage` smoothing).
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

    let mut out = [0.0; 4];
    for (c, slot) in out.iter_mut().enumerate() {
        let v00 = f64::from(image.data[i00 + c]);
        let v10 = f64::from(image.data[i10 + c]);
        let v01 = f64::from(image.data[i01 + c]);
        let v11 = f64::from(image.data[i11 + c]);
        let top = v00 * (1.0 - fx) + v10 * fx;
        let bottom = v01 * (1.0 - fx) + v11 * fx;
        *slot = top * (1.0 - fy) + bottom * fy;
    }
    out
}

/// Resize with bilinear filtering (downscale-friendly).
///
/// # Errors
///
/// Propagates validation errors from [`assert_rgba_image`].
pub fn resize_bilinear(
    source: &RgbaImage,
    width: f64,
    height: f64,
) -> Result<RgbaImage, ImageBufferError> {
    assert_rgba_image(source, "source")?;
    let w = js_round_nonneg(width).max(1);
    let h = js_round_nonneg(height).max(1);
    let mut out = create_rgba_image(w, h);
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
            out.data[i] = clamp_u8(sample[0]);
            out.data[i + 1] = clamp_u8(sample[1]);
            out.data[i + 2] = clamp_u8(sample[2]);
            out.data[i + 3] = clamp_u8(sample[3]);
        }
    }
    Ok(out)
}

/// Resize with nearest-neighbor (pixel-art upscale).
///
/// # Errors
///
/// Propagates validation errors from [`assert_rgba_image`].
pub fn resize_nearest(
    source: &RgbaImage,
    width: f64,
    height: f64,
) -> Result<RgbaImage, ImageBufferError> {
    assert_rgba_image(source, "source")?;
    let w = js_round_nonneg(width).max(1);
    let h = js_round_nonneg(height).max(1);
    let mut out = create_rgba_image(w, h);
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
