//! ECMAScript `Uint8ClampedArray` write semantics (roundTiesToEven).

/// Convert a floating-point sample to `u8` the same way JS `Uint8ClampedArray` does.
///
/// Spec behaviour: NaN / non-finite / ≤0 → 0; ≥255 → 255; otherwise round half to even.
#[inline]
pub(crate) fn clamp_u8(n: f64) -> u8 {
    if !n.is_finite() || n <= 0.0 {
        return 0;
    }
    if n >= 255.0 {
        return 255;
    }
    let floor = n.floor();
    let frac = n - floor;
    let floor_u = floor as u64;
    if frac < 0.5 {
        floor_u as u8
    } else if frac > 0.5 {
        (floor_u + 1) as u8
    } else if floor_u % 2 == 0 {
        floor_u as u8
    } else {
        (floor_u + 1) as u8
    }
}

/// JavaScript `Math.round` for non-negative values (half away from zero / toward +∞).
#[inline]
pub(crate) fn js_round_nonneg(n: f64) -> u32 {
    debug_assert!(n >= 0.0);
    n.round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_matches_uint8_clamped_array_edges() {
        assert_eq!(clamp_u8(f64::NAN), 0);
        assert_eq!(clamp_u8(-1.0), 0);
        assert_eq!(clamp_u8(0.0), 0);
        assert_eq!(clamp_u8(255.0), 255);
        assert_eq!(clamp_u8(300.0), 255);
        // 1/3 → 0; 2/3 → 1; 4/3 → 1 (roundTiesToEven / nearest)
        assert_eq!(clamp_u8(1.0 / 3.0), 0);
        assert_eq!(clamp_u8(2.0 / 3.0), 1);
        assert_eq!(clamp_u8(4.0 / 3.0), 1);
        // ties to even
        assert_eq!(clamp_u8(1.5), 2);
        assert_eq!(clamp_u8(2.5), 2);
    }
}
