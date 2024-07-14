use super::{Appliable, ApplyContext};
use anyhow::{bail, Result};
use log::warn;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Delimiter {
    template: String,
    start: String,
    end: String,
}

impl Delimiter {
    fn delimiter_status(&self, text: &str) -> (bool, bool) {
        let start_exists = text
            .lines()
            .find(|line| line.trim() == &self.start)
            .is_some();
        let end_exists = text
            .lines()
            .skip_while(|line| line.trim() == &self.start)
            .find(|line| line.trim() == &self.end)
            .is_some();

        (start_exists, end_exists)
    }
}

impl Appliable for Delimiter {
    fn apply(&self, context: ApplyContext) -> Result<()> {
        let content = fs::read_to_string(&context.config_file)?;
        let (start_exists, end_exists) = self.delimiter_status(&content);

        if !start_exists && !end_exists {
            warn!("No delimiters found at {}", context.config_file.display());
            return Ok(());
        }

        if !start_exists {
            bail!("Missing top delimiter at {}", context.config_file.display())
        }

        if !end_exists {
            bail!(
                "Missing bottom delimiter at {}",
                context.config_file.display()
            )
        }

        let head = content
            .lines()
            .take_while(|line| line.trim() != self.start)
            .collect::<String>();
        let tail = content
            .lines()
            .skip_while(|line| line.trim() != self.end)
            .collect::<String>();

        let rendered = context.templater.render(&self.template, &context.theme)?;

        Ok(fs::write(
            &context.config_file,
            format!("{head}\n{}\n{rendered}\n{}\n{tail}", &self.start, &self.end),
        )?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{utils, Applier, Method, Templater};
    use anyhow::Result;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn applier(path: PathBuf) -> Applier {
        Applier {
            name: "test".to_string(),
            path,
            method: Method::Delimiter(Delimiter {
                template: "template".to_string(),
                start: "#start#".to_string(),
                end: "#end#".to_string(),
            }),
        }
    }

    #[test]
    fn test_replace_delimiter() -> Result<()> {
        let dir = tempdir()?;
        fs::write(
            dir.path().join("test_file"),
            r#"some text
            #start#
            replacement
            goes
            here
            #end#
            more text"#,
        )?;

        let mut templater = Templater::default();
        templater.add_raw_template("template", "current theme: {{name}}")?;

        applier(dir.path().join("test_file")).apply(&utils::theme(), &templater)?;

        assert!(
            fs::read_to_string(dir.path().join("test_file"))?.contains("current theme: gruvbox"),
        );

        Ok(())
    }

    #[test]
    fn test_replace_delimiter_missing() -> Result<()> {
        let dir = tempdir()?;
        fs::write(dir.path().join("test_file"), "replacement\ngoes\nhere")?;

        let mut templater = Templater::default();
        templater.add_raw_template("template", "current theme: {{name}}")?;

        applier(dir.path().join("test_file")).apply(&utils::theme(), &templater)?;

        assert_eq!(
            fs::read_to_string(dir.path().join("test_file"))?,
            "replacement\ngoes\nhere"
        );

        Ok(())
    }

    #[test]
    fn test_replace_delimiter_missing_part() -> Result<()> {
        let dir = tempdir()?;
        let mut templater = Templater::default();
        templater.add_raw_template("template", "current theme: {{name}}")?;

        let tests = vec![("top", "", "#end#"), ("bottom", "#start#", "")];
        for test in tests {
            fs::write(
                dir.path().join(test.0),
                format!("{}\nreplace\nthis\n{}", test.1, test.2),
            )?;

            let result = applier(dir.path().join(test.0)).apply(&utils::theme(), &templater);

            assert_eq!(
                result.err().unwrap().to_string(),
                format!(
                    "Missing {} delimiter at {}",
                    test.0,
                    dir.path().join(test.0).display()
                )
            );
        }

        Ok(())
    }
}
