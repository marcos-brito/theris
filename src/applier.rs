use crate::{Templater, Theme};
use anyhow::{anyhow, bail, Context, Result};
use log::warn;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Applier {
    name: String,
    path: PathBuf,
    method: Method,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Method {
    ReplaceText {
        target: String,
        replacement: String,
    },
    Script {
        path: PathBuf,
    },
    Template {
        template: String,
    },
    Delimeter {
        template: String,
        start: String,
        end: String,
    },
}

impl Applier {
    pub fn apply(&self, theme: Theme) -> Result<()> {
        match &self.method {
            Method::ReplaceText {
                target,
                replacement,
            } => self.replace_text(theme, target.clone(), replacement.clone()),
            Method::Script { path } => self.run_script(theme, path.clone()),
            _ => todo!(),
        }
    }

    fn replace_text(&self, theme: Theme, target: String, replacement: String) -> Result<()> {
        let content = fs::read_to_string(&self.path)
            .with_context(|| anyhow!("Failed to read {}", self.path.display()))?;
        let rendered = Templater::render_raw(&replacement, theme)?;
        let reg = Regex::new(&target)?;
        let new = reg.replace(&content, &rendered);

        if new == content {
            warn!("No matches found for {target}. Nothing to replace");
            return Ok(());
        }

        fs::write(&self.path, new.to_string())
            .with_context(|| anyhow!("Failed to write {}", self.path.display()))?;

        Ok(())
    }

    fn run_script(&self, theme: Theme, path: PathBuf) -> Result<()> {
        let out = Command::new(&path).arg(theme.format_to_stdin()).output()?;

        if !out.status.success() {
            let err = match String::from_utf8(out.stderr) {
                Ok(err) => err,
                Err(e) => {
                    warn!("stderr of {} has invalid utf8: {e}", path.display());
                    String::new()
                }
            };

            bail!("{} failed: {}", path.display(), err)
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use tempfile::tempdir;

    fn get_theme() -> Theme {
        Theme {
            name: "gruvbox".to_string(),
            background: "".to_string(),
            foreground: "".to_string(),
            colors: vec![],
        }
    }

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
            method: Method::ReplaceText {
                target: "theme: (.)*".to_string(),
                replacement: "theme: {{name}}".to_string(),
            },
        };

        applier.apply(get_theme())?;

        assert_eq!(
            fs::read_to_string(dir.path().join("test_file"))?,
            "blue: #\nred: #\n theme: gruvbox\n # comment"
        );

        Ok(())
    }

    #[test]
    fn test_run_script() -> Result<()> {
        let dir = tempfile::tempdir_in(".")?;
        let script_path = dir.path().join("script.sh");
        let applier = Applier {
            name: "test".to_string(),
            path: dir.path().join("test_file"),
            method: Method::Script {
                path: script_path.clone(),
            },
        };

        fs::write(
            &script_path,
            "#!/bin/sh\n\necho \"Some error\" >&2\n exit 1".as_bytes(),
        )?;

        let mut perm = fs::metadata(&script_path)?.permissions();
        perm.set_mode(0o777);
        fs::set_permissions(&script_path, perm)?;

        let result = applier.apply(get_theme());
        assert_eq!(
            result.err().unwrap().to_string(),
            format!(
                "{} failed: Some error\n",
                dir.path().join("script.sh").display()
            )
        );

        Ok(())
    }
}
