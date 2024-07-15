use super::{App, Cmd};
use crate::backup::Backup;
use anyhow::Result;
use clap::ArgMatches;
use colored::Colorize;
use log::info;
use std::path::PathBuf;

pub struct Restore<'a> {
    app: &'a App,
    backup_path: PathBuf,
    dest: Option<PathBuf>,
}

impl Restore<'_> {
    pub fn new<'a>(matches: &ArgMatches, app: &'a App) -> Restore<'a> {
        Restore {
            app,
            backup_path: matches
                .get_one::<String>("backup_path")
                .and_then(|p| Some(PathBuf::from(p)))
                .unwrap(),
            dest: matches
                .get_one::<String>("dest")
                .and_then(|p| Some(PathBuf::from(p))),
        }
    }
}

impl Cmd for Restore<'_> {
    fn run(&self) -> Result<()> {
        let backup_path = match &self.backup_path.to_string_lossy() == "last" {
            true => self.app.backup_dir.join("last"),
            false => self.backup_path.to_path_buf(),
        };

        match &self.dest {
            Some(dest) => Backup::restore_to(&backup_path, &dest)?,
            None => {
                Backup::restore(&backup_path)?;
            }
        }

        info!(
            "Done restoring {} ",
            backup_path.display().to_string().purple()
        );
        Ok(())
    }
}
