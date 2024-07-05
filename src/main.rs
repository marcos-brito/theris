use anyhow::Result;
use clap::{crate_authors, crate_description, crate_version, Arg, Command};
use colored::Colorize;
use env_logger::{Builder, Env};
use human_panic::setup_panic;
use is_terminal::IsTerminal;
use log::{error, Level};
use std::io::Write;

fn main() {
    setup_panic!();
    setup_log();

    if let Err(e) = run() {
        error!("{e}");
        std::process::exit(1);
    }
}

fn cli() -> Command {
    Command::new("theris")
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .about(crate_description!())
        .subcommand_required(true)
        .subcommand(
            Command::new("apply")
                .about("Apply a theme")
                .arg(Arg::new("theme").help("The theme to be applied"))
                .arg(Arg::new("except").help("Apply all except"))
                .arg(Arg::new("Appliers").help("Appliers to be used")),
        )
        .subcommand(
            Command::new("restore")
                .about("Restore files from a backup")
                .arg(Arg::new("Backup's path").help("The path to the backup file")),
        )
}

fn run() -> Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("apply", _submatches)) => todo!(),
        Some(("restore", _submatches)) => todo!(),
        _ => unreachable!(),
    }
}

fn setup_log() {
    if !std::io::stdout().is_terminal() {
        env_logger::init();
        return;
    }

    let env = Env::default().default_filter_or("info");
    Builder::from_env(env)
        .format(|buf, record| match record.level() {
            Level::Info => writeln!(buf, "{} {}", ">".green().bold(), record.args()),
            Level::Error => writeln!(buf, "{} {}", "error:".red().bold(), record.args()),
            Level::Warn => writeln!(buf, "{} {}", "warn:".yellow().bold(), record.args()),
            _ => writeln!(
                buf,
                "{}: {}",
                record.level().to_string().magenta().bold(),
                record.args()
            ),
        })
        .init();
}
