use clap::{crate_authors, crate_description, crate_version, Arg, ArgAction, Command};

pub fn cli() -> Command {
    Command::new("theris")
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .about(crate_description!())
        .subcommand_required(true)
        .arg(
            Arg::new("backup_path")
                .long("backup-path")
                .value_name("DIRECTORY")
                .help("Directory where backups should be saved"),
        )
        .arg(
            Arg::new("config_path")
                .long("config-path")
                .value_name("DIRECTORY")
                .help("Directory where config files should be read from"),
        )
        .arg(
            Arg::new("templates_path")
                .long("templates-path")
                .value_name("DIRECTORY")
                .help("Directory where templates files should be read from"),
        )
        .subcommand(
            Command::new("apply")
                .about("Apply a theme")
                .arg(
                    Arg::new("backup")
                        .long("no-backup")
                        .action(ArgAction::SetFalse)
                        .help("Do not save backups"),
                )
                .arg(Arg::new("theme").help("Theme to be applied").required(true))
                .arg(
                    Arg::new("except")
                        .long("except")
                        .action(ArgAction::SetTrue)
                        .help("Use all appliers except"),
                )
                .arg(Arg::new("appliers").help("List of appliers to be used")),
        )
        .subcommand(
            Command::new("restore")
                .about("Restore files from a backup")
                .arg(
                    Arg::new("backup_path")
                        .help("Path to the backup file")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("list").about("List a collection").arg(
                Arg::new("collection")
                    .help("Collection to be listed")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("generate")
                .about("Generates a theme using a image")
                .arg(Arg::new("save").help("Saves the theme"))
                .arg(Arg::new("discard").help("Discard the theme"))
                .arg(Arg::new("Image path").help("The path to a image")),
        )
}
