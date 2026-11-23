//! ECMAScript `ToUint8Clamp` for parity with `Uint8ClampedArray` stores.

/// Clamp / round a float into `u8` the same way `Uint8ClampedArray` assignment does
/// (ECMAScript `ToUint8Clamp`: half ties round to even).
#[inline]
pub(crate) fn to_u8_clamp(n: f64) -> u8 {
    if n.is_nan() || n <= 0.0 {
        return 0;
    }
    if n >= 255.0 {
        return 255;
    }
    let f = n.floor();
    let f_int = f as u32;
    let half = f + 0.5;
    // ECMAScript ToUint8Clamp: round half to even.
    #[allow(clippy::if_same_then_else)]
    if n < half {
        f_int as u8
    } else if n > half {
        (f_int + 1) as u8
    } else if f_int % 2 == 1 {
        (f_int + 1) as u8
    } else {
        f_int as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_edges_and_nan() {
        assert_eq!(to_u8_clamp(f64::NAN), 0);
        assert_eq!(to_u8_clamp(-1.0), 0);
        assert_eq!(to_u8_clamp(0.0), 0);
        assert_eq!(to_u8_clamp(255.0), 255);
        assert_eq!(to_u8_clamp(300.0), 255);
    }

    #[test]
    fn half_ties_to_even() {
        // 1.5 → 2 (odd floor 1 → round up); 2.5 → 2 (even floor 2 → stay)
        assert_eq!(to_u8_clamp(1.5), 2);
        assert_eq!(to_u8_clamp(2.5), 2);
        assert_eq!(to_u8_clamp(84.5), 84);
        assert_eq!(to_u8_clamp(85.5), 86);
    }
}
