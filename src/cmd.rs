use crate::cli::cli;
use anyhow::Result;

trait Cmd {
    fn run(&self) -> Result<()>;
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
        Some(("apply", submatches)) => todo!(),
        Some(("list", submatches)) => todo!(),
        Some(("restore", submatches)) => todo!(),
        _ => unreachable!(),
    }
}
