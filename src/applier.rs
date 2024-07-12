mod replace_text;
mod script;

use crate::{Templater, Theme};
use anyhow::Result;
use log::info;
use replace_text::ReplaceText;
use script::Script;
use serde::{Deserialize, Serialize};
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

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Method {
    ReplaceText(ReplaceText),
    Script(Script),
}

impl Applier {
    pub fn apply(&self, theme: &Theme, templater: &Templater) -> Result<()> {
        let context = ApplyContext::new(self.path.clone(), theme.clone(), templater.clone());
        info!("Applying {} to {}", theme.name, self.name);

        // Can't "or" this. The enum needs the struct itself (not the trait) so the yaml can be deserialized.
        match &self.method {
            Method::ReplaceText(appliable) => appliable.apply(&context)?,
            Method::Script(appliable) => appliable.apply(&context)?,
            _ => todo!(),
        }

        Ok(())
    }
}
