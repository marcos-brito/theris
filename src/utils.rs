use crate::Theme;
use hex::FromHex;

pub fn theme() -> Theme {
    Theme {
        name: "gruvbox".to_string(),
        background: "".to_string(),
        foreground: "".to_string(),
        colors: vec![],
    }
}

pub fn hexa_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let rgb = <[u8; 3]>::from_hex(hex.strip_prefix("#").unwrap_or(&hex)).ok()?;

    Some((rgb[0], rgb[1], rgb[2]))
}
