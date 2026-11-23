//! Hex / Paint.NET palette helpers.

use crate::types::Rgb;

/// Parse `#rrggbb` / `rrggbb` into RGB, or `None` if invalid.
pub fn hex_to_rgb(hex: &str) -> Option<Rgb> {
    let s = hex.strip_prefix('#').unwrap_or(hex);
    if s.len() != 6 || !s.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let r = u8::from_str_radix(&s[0..2], 16).ok()?;
    let g = u8::from_str_radix(&s[2..4], 16).ok()?;
    let b = u8::from_str_radix(&s[4..6], 16).ok()?;
    Some([r, g, b])
}

/// Format RGB as lowercase `#rrggbb`.
pub fn rgb_to_hex(rgb: Rgb) -> String {
    format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2])
}

/// Parse Paint.NET / `.hex` / `.txt` palette text.
///
/// Splits on commas or newlines, skips `;` comments, takes the last 6 hex digits of each token.
pub fn parse_palette_text(text: &str) -> Vec<Rgb> {
    let mut out = Vec::new();
    for part in text.split(['\r', '\n', ',']) {
        let s = part.trim();
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
