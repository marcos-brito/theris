use super::{Appliable, ApplyContext};
use anyhow::{bail, Result};
use log::warn;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    path: PathBuf,
}

impl Appliable for Script {
    fn apply(&self, context: ApplyContext) -> Result<()> {
        let out = Command::new(&self.path)
            .arg(context.theme.format_to_stdin())
            .output()?;

        if !out.status.success() {
            let err = match String::from_utf8(out.stderr) {
                Ok(err) => err,
                Err(e) => {
                    warn!("stderr of {} has invalid utf8: {e}", self.path.display());
                    String::new()
                }
            };

            bail!("{} failed: {}", self.path.display(), err)
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{utils, Applier, Method, Templater};
    use anyhow::Result;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn test_run_script() -> Result<()> {
        let dir = tempfile::tempdir_in(".")?;
        let script_path = dir.path().join("script.sh");
        let applier = Applier {
            name: "test".to_string(),
            path: dir.path().join("test_file"),
            method: Method::Script(Script {
                path: script_path.clone(),
            }),
        };

        fs::write(
            &script_path,
            "#!/bin/sh\n\necho \"Some error\" >&2\n exit 1".as_bytes(),
        )?;

        let mut perm = fs::metadata(&script_path)?.permissions();
        perm.set_mode(0o777);
        fs::set_permissions(&script_path, perm)?;

        let result = applier.apply(&utils::theme(), &Templater::default());
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
