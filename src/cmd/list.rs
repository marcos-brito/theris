use super::{App, Cmd};
use anyhow::{bail, Result};
use clap::ArgMatches;

pub struct List<'a> {
    app: &'a App,
    collection: String,
}

impl List<'_> {
    pub fn new<'a>(matches: &ArgMatches, app: &'a App) -> List<'a> {
        List {
            app,
            collection: matches.get_one::<String>("collection").unwrap().to_string(),
        }
    }
}

impl Cmd for List<'_> {
    fn run(&self) -> Result<()> {
        match self.collection.as_str() {
            "themes" => {
                for theme in self.app.config.themes().iter() {
                    println!("{theme}");
                }
            }
            "appliers" => {
                for applier in self.app.config.appliers().iter() {
                    println!("{applier}");
                }
            }
            "templates" => println!("{}", self.app.templater),
            _ => bail!(
                "Unknow collection \"{}\". Try \"themes\", \"appliers\" or \"templates\" ",
                self.collection
            ),
        };

        Ok(())
    }
}
