use super::{App, Cmd};
use crate::backup::Backup;
use crate::{Applier, Theme};
use anyhow::{bail, Result};
use clap::ArgMatches;
use colored::Colorize;
use log::info;

pub struct Apply<'a> {
    app: &'a App,
    should_backup: bool,
    except: bool,
    theme_to_apply: String,
    appliers_names: Option<Vec<String>>,
}

impl Apply<'_> {
    pub fn new<'a>(matches: &ArgMatches, app: &'a App) -> Apply<'a> {
        Apply {
            app,
            should_backup: matches.get_flag("backup"),
            except: matches.get_flag("except"),
            theme_to_apply: matches.get_one::<String>("theme").unwrap().to_string(),
            appliers_names: matches
                .get_many::<String>("appliers")
                .and_then(|names| Some(names.map(|n| n.to_string()).collect::<Vec<String>>())),
        }
    }

    fn apply_all(&self, theme: &Theme) -> Result<()> {
        if self.should_backup {
            self.create_backup(&self.app.config.appliers())?;
        }

        for applier in self.app.config.appliers() {
            applier.apply(&theme, &self.app.templater)?;
        }

        Ok(())
    }

    fn apply_all_except(&self, theme: &Theme, exceptions: &Vec<Applier>) -> Result<()> {
        let appliers = self
            .app
            .config
            .appliers()
            .into_iter()
            .filter_map(|applier| {
                if exceptions.contains(&applier) {
                    return None;
                }

                Some(applier)
            })
            .collect();

        self.create_backup(&appliers)?;

        for applier in appliers {
            applier.apply(&theme, &self.app.templater)?;
        }

        Ok(())
    }

    fn appliers_from_names(&self, names: &Vec<String>) -> Result<Vec<Applier>> {
        let mut appliers = Vec::new();

        for name in names.iter() {
            let applier = self.app.find_applier(&name);

            if applier.is_none() {
                bail!("No applier named {name}");
            }

            appliers.push(applier.unwrap());
        }

        Ok(appliers)
    }

    pub fn create_backup(&self, appliers: &Vec<Applier>) -> Result<()> {
        let mut backup = Backup::new();

        for applier in appliers.iter() {
            backup.add(applier.path());
        }

        backup.save_as_last(&self.app.backup_path)?;

        Ok(())
    }
}

impl Cmd for Apply<'_> {
    fn run(&self) -> Result<()> {
        let theme = self.app.find_theme(&self.theme_to_apply);

        if theme.is_none() {
            bail!("No theme named {}", &self.theme_to_apply);
        }
        let theme = theme.unwrap();

        match &self.appliers_names {
            None => self.apply_all(&theme)?,
            Some(names) => {
                let appliers = self.appliers_from_names(&names)?;

                if self.except {
                    self.apply_all_except(&theme, &appliers)?;
                } else {
                    for applier in appliers.iter() {
                        if self.should_backup {
                            self.create_backup(&appliers)?;
                        }

                        applier.apply(&theme, &self.app.templater)?;
                    }
                }
            }
        }

        info!("Done applying {}", theme.name.magenta());
        Ok(())
    }
}
