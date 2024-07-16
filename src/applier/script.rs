use super::{Appliable, ApplyContext};
use anyhow::{bail, Result};
use log::warn;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    path: PathBuf,
}

impl Script {
    fn write_input_to_child(&self, child: &mut Child, context: &ApplyContext) {
        let mut stdin = child.stdin.take().expect("Child has stdin");

        if let Err(e) = stdin.write_all(context.theme.format_to_stdin().as_bytes()) {
            warn!(
                "Couldn't write to stdin after spawning {}: {}",
                &self.path.display(),
                e
            );
        }
    }
}

impl Appliable for Script {
    fn apply(&self, context: ApplyContext) -> Result<()> {
        let mut child = Command::new(&self.path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        self.write_input_to_child(&mut child, &context);

        let out = child.wait_with_output()?;
        if !out.status.success() {
            let mut err = String::from_utf8_lossy(&out.stderr);

            if err.is_empty() {
                err = String::from_utf8_lossy(&out.stdout);

                warn!(
                    "stderr of {} is empty. Showing stdout instead",
                    self.path.display()
                );
            }

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
    fn test_run_script_error() -> Result<()> {
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
            "#!/bin/sh\n\n(echo \"Some error\" >&2) &&\nexit 1".as_bytes(),
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
