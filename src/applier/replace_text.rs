use super::{Appliable, ApplyContext};
use crate::Templater;
use anyhow::{anyhow, Context, Result};
use log::warn;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceText {
    target: String,
    replacement: String,
}

impl Appliable for ReplaceText {
    fn apply(&self, context: &ApplyContext) -> Result<()> {
        let content = fs::read_to_string(&context.config_file)
            .with_context(|| anyhow!("Failed to read {}", context.config_file.display()))?;
        let rendered = Templater::new().render_raw(&self.replacement, &context.theme)?;
        let reg = Regex::new(&self.target)?;
        let new = reg.replace(&content, &rendered);

        if new == content {
            warn!("No matches found for {}. Nothing to replace", self.target);
            return Ok(());
        }

        fs::write(&context.config_file, new.to_string())
            .with_context(|| anyhow!("Failed to write {}", context.config_file.display()))?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{utils, Applier, Method, Templater};
    use anyhow::Result;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_replace_line() -> Result<()> {
        let dir = tempdir()?;
        fs::write(
            dir.path().join("test_file"),
            "blue: #\nred: #\n theme: dracula\n # comment",
        )?;

        let applier = Applier {
            name: "test".to_string(),
            path: dir.path().join("test_file"),
            method: Method::ReplaceText(ReplaceText {
                target: "theme: (.)*".to_string(),
                replacement: "theme: {{name}}".to_string(),
            }),
        };

        applier.apply(&utils::theme(), &Templater::new())?;

        assert_eq!(
            fs::read_to_string(dir.path().join("test_file"))?,
            "blue: #\nred: #\n theme: gruvbox\n # comment"
        );

        Ok(())
    }
}
