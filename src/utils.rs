use crate::Theme;
use hex::FromHex;
use std::collections::HashMap;

pub fn theme() -> Theme {
    Theme {
        name: "gruvbox".to_string(),
        colors: HashMap::from([("background".to_string(), "".to_string())]),
        extra: None,
    }
}

pub fn hexa_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let rgb = <[u8; 3]>::from_hex(hex.strip_prefix("#").unwrap_or(&hex)).ok()?;

    Some((rgb[0], rgb[1], rgb[2]))
}
