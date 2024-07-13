mod delimeter;
mod replace_text;
mod script;
mod template;

use crate::{Templater, Theme};
use anyhow::Result;
use colored::Colorize;
use delimeter::Delimiter;
use log::info;
use replace_text::ReplaceText;
use script::Script;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use template::Template;

trait Appliable {
    fn apply(&self, context: &ApplyContext) -> Result<()>;
}

struct ApplyContext {
    config_file: PathBuf,
    theme: Theme,
    templater: Templater,
}

impl ApplyContext {
    fn new(config_file: PathBuf, theme: Theme, templater: Templater) -> Self {
        Self {
            config_file,
            theme,
            templater,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Applier {
    name: String,
    path: PathBuf,
    method: Method,
}

impl fmt::Display for Applier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}",
            self.name.magenta().bold(),
            format!("  {}: {}", "config file".bold(), self.path.display()),
            format!("  {}: {}", "method".bold(), self.method),
        )
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Method {
    ReplaceText(ReplaceText),
    Script(Script),
    Template(Template),
    Delimeter(Delimiter),
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Method::ReplaceText(_) => "replace text",
                Method::Script(_) => "script",
                Method::Delimeter(_) => "delimiter",
                Method::Template(_) => "template",
            }
        )
    }
}

impl Applier {
    pub fn apply(&self, theme: &Theme, templater: &Templater) -> Result<()> {
        let context = ApplyContext::new(self.path.clone(), theme.clone(), templater.clone());
        info!("Applying {} to {}", theme.name, self.name);

        // Can't "or" this. The enum needs the struct itself (not the trait) so the yaml can be deserialized.
        match &self.method {
            Method::ReplaceText(appliable) => appliable.apply(&context)?,
            Method::Script(appliable) => appliable.apply(&context)?,
            Method::Template(appliable) => appliable.apply(&context)?,
            Method::Delimeter(appliable) => appliable.apply(&context)?,
        }

        Ok(())
    }
}
