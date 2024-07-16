pub mod applier;
pub mod backup;
pub mod cli;
pub mod cmd;
pub mod config;
pub mod template;
pub mod utils;

pub use applier::{Applier, Method};
use colored::Colorize;
use log::warn;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
pub use template::Templater;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    name: String,
    colors: HashMap<String, String>,
    extra: Option<HashMap<String, String>>,
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n  ", self.name.bold())?;

        for color in self.colors.values() {
            let (r, g, b) = match utils::hexa_to_rgb(color) {
                Some(rgb) => rgb,
                None => {
                    write!(f, "{}", "   ")?;
                    continue;
                }
            };

            write!(f, "{}", "   ".on_truecolor(r, g, b))?;
        }

        match &self.extra {
            Some(extra) => {
                for (key, value) in extra.iter() {
                    write!(f, "\n  - {}: {}", key.italic(), value)?;
                }
            }
            None => (),
        }

        Ok(())
    }
}

impl Theme {
    // Maybe rename this? i don't like it
    fn format_to_stdin(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(e) => {
                warn!("Couldn't write json output: {e}");
                String::new()
            }
        }
    }
}
