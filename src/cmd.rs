mod apply;
mod list;
mod restore;

use crate::cli::cli;
use crate::config::Config;
use crate::{Applier, Templater, Theme};
use anyhow::Result;
use log::warn;
use std::path::PathBuf;

trait Cmd {
    fn run(&self) -> Result<()>;
}

struct App {
    config: Config,
    templater: Templater,
    backup_dir: PathBuf,
}

impl App {
    fn new(config: PathBuf, templates: PathBuf, backup: PathBuf) -> Result<Self> {
        for path in [&config, &templates, &backup] {
            if !path.exists() {
                warn!("{} does not exists. Make sure to create it", path.display());
            }
        }

        let config = Config::new(config)?;
        // TODO: remove the unwrap
        let templater = Templater::new(templates.join("**").to_str().unwrap())?;

        Ok(Self {
            config,
            templater,
            backup_dir: backup,
        })
    }

    fn default_backup_path() -> PathBuf {
        dirs::data_dir()
            .unwrap_or(PathBuf::from("."))
            .join("theris")
            .join("backup")
    }

    fn default_templates_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or(PathBuf::from("."))
            .join("theris")
            .join("templates")
    }

    fn default_config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or(PathBuf::from("."))
            .join("theris")
    }

    pub fn find_theme(&self, name: &str) -> Option<Theme> {
        self.config.themes().into_iter().find_map(|theme| {
            if theme.name == name {
                return Some(theme);
            }
            None
        })
    }

    pub fn find_applier(&self, name: &str) -> Option<Applier> {
        self.config.appliers().into_iter().find_map(|applier| {
            if applier.name() == name {
                return Some(applier);
            }
            None
        })
    }
}

pub fn run() -> Result<()> {
    let matches = cli().get_matches();
    let config_path = matches
        .get_one::<PathBuf>("config_path")
        .and_then(|path| Some(path.clone()))
        .unwrap_or(App::default_config_path());
    let templates_path = matches
        .get_one::<PathBuf>("templates_path")
        .and_then(|path| Some(path.clone()))
        .unwrap_or(App::default_templates_path());
    let backup_path = matches
        .get_one::<PathBuf>("backup_path")
        .and_then(|path| Some(path.clone()))
        .unwrap_or(App::default_backup_path());

    let app = App::new(config_path, templates_path, backup_path)?;

    match matches.subcommand() {
        Some(("apply", submatches)) => apply::Apply::new(submatches, &app).run(),
        Some(("list", submatches)) => list::List::new(submatches, &app).run(),
        Some(("restore", submatches)) => restore::Restore::new(submatches, &app).run(),
        _ => unreachable!(),
    }
}
