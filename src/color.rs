//! Euclidean RGB distance and nearest-palette matching (Pixel It semantics).

use crate::error::{Error, Result};
use crate::types::Rgb;

/// Euclidean distance between two RGB colors (lower is closer).
///
/// Range roughly `[0, ~441.67]`.
#[must_use]
pub fn color_sim(a: Rgb, b: Rgb) -> f64 {
    let mut s = 0.0_f64;
    for i in 0..3 {
        let d = f64::from(a[i]) - f64::from(b[i]);
        s += d * d;
    }
    s.sqrt()
}

/// Pick the palette color closest to `color`.
///
/// Ties prefer **later** entries, matching Pixel It (`d <= bestD`).
///
/// # Errors
///
/// Returns [`Error::EmptyPalette`] when `palette` is empty.
pub fn similar_color(color: Rgb, palette: &[Rgb]) -> Result<Rgb> {
    let first = *palette.first().ok_or(Error::EmptyPalette)?;
    let mut best = first;
    let mut best_d = color_sim(color, best);
    for &cur in &palette[1..] {
        let d = color_sim(color, cur);
        if d <= best_d {
            best = cur;
            best_d = d;
        }
    }
    Ok(best)
}
