//! Hex / Paint.NET palette text helpers.

use crate::types::Rgb;

/// Parse `#rrggbb` / `rrggbb` into RGB, or `None` if invalid.
#[must_use]
pub fn hex_to_rgb(hex: &str) -> Option<Rgb> {
    let s = hex.strip_prefix('#').unwrap_or(hex);
    if s.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&s[0..2], 16).ok()?;
    let g = u8::from_str_radix(&s[2..4], 16).ok()?;
    let b = u8::from_str_radix(&s[4..6], 16).ok()?;
    // Reject non-hex leftovers already handled by from_str_radix; also require
    // that every character was hex (from_str_radix does). Case-insensitive.
    if !s.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    Some([r, g, b])
}

/// Format RGB as lowercase `#rrggbb`.
#[must_use]
pub fn rgb_to_hex(rgb: Rgb) -> String {
    format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2])
}

/// Parse Paint.NET / `.hex` / `.txt` palette text.
///
/// Splits on commas or newlines, skips `;` comments, takes the last 6 hex digits
/// of each token.
#[must_use]
pub fn parse_palette_text(text: &str) -> Vec<Rgb> {
    let mut out = Vec::new();
    for raw in text.split(['\r', '\n', ',']) {
        let s = raw.trim();
        if s.is_empty() || s.starts_with(';') {
            continue;
        }
        let last6 = if s.len() >= 6 { &s[s.len() - 6..] } else { s };
        if let Some(rgb) = hex_to_rgb(last6) {
            out.push(rgb);
        }
    }
    out
}
