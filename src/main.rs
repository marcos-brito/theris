use anyhow::Error;
use colored::Colorize;
use env_logger::{Builder, Env};
use human_panic::setup_panic;
use is_terminal::IsTerminal;
use log::{error, Level};
use std::io::Write;
use theris::cmd::run;

fn main() {
    setup_panic!();
    setup_log();

    if let Err(e) = run() {
        log_backtrace(&e);
        std::process::exit(1);
    }
}

fn log_backtrace(e: &Error) {
    error!("{e}");

    for cause in e.chain().skip(1) {
        error!("\tCaused by: {cause}");
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
