pub mod applier;
pub mod template;

pub use applier::{Applier, Method};
use serde::{Deserialize, Serialize};
pub use template::Templater;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    name: String,
    background: String,
    foreground: String,
    colors: Vec<String>,
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
