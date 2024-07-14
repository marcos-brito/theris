use super::{Appliable, ApplyContext};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    template: String,
}

impl Appliable for Template {
    fn apply(&self, context: ApplyContext) -> Result<()> {
        let rendered = context.templater.render(&self.template, &context.theme)?;

        fs::write(&context.config_file, &rendered)
            .with_context(|| anyhow!("Failed to write {}", context.config_file.display()))?;

        Ok(())
    }
}
