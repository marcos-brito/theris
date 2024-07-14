pub mod applier;
pub mod backup;
pub mod cli;
pub mod cmd;
pub mod config;
pub mod template;
pub mod utils;

pub use applier::{Applier, Method};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt;
pub use template::Templater;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
// TODO: remove bg and fg and just use a hashmap for colors
pub struct Theme {
    name: String,
    background: String,
    foreground: String,
    colors: Vec<String>,
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.name.bold())?;

        for color in self.colors.iter() {
            let (r, g, b) = match utils::hexa_to_rgb(color) {
                Some(rgb) => rgb,
                None => {
                    write!(f, "{}", "   ")?;
                    continue;
                }
            };

            write!(f, "{}", "   ".on_truecolor(r, g, b))?;
        }

        Ok(())
    }
}

impl Theme {
    // Maybe rename this? i don't like it
    fn format_to_stdin(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}",
            self.name,
            self.background,
            self.foreground,
            self.colors.join("\n")
        )
    }
}
