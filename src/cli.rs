use clap::{Arg, ArgAction, Command};

/// Builds the command-line interface for the application.
pub fn build_cli() -> Command {
    Command::new("base16_shell")
        .version("1.0.0")
        .author("Tinted Theming")
        .about("A tool to switch base16 colorschemes")
        // Define a global argument for specifying the repository directory
        .arg(
            Arg::new("repo-dir")
                .short('d')
                .help("Optional path to the base16-shell repository. This is used to run the colorschemes and hooks if you don't want to use the compiled versions.")
                .value_name("DIR")
                .long("repo-dir")
                .global(true)
                .action(ArgAction::Set)
        )
        // Define a global argument for specifying the repository directory
        .arg(
            Arg::new("config")
                .short('c')
                .help("Optional path to the base16-shell config directory.")
                .value_name("CONFIG")
                .long("config")
                .global(true)
                .action(ArgAction::Set)
        )
        // Define subcommands
        .subcommand(
            Command::new("init").about("Initializes base16 with the exising config. Used to Initialize exising theme for when your shell starts up.")
        )
        .subcommand(Command::new("list").about("Lists available base16 colorschemes"))
        .subcommand(
            Command::new("set").about("Sets a base16 colorscheme").arg(
                Arg::new("theme_name")
                    .help("The base16 colorscheme you want to set")
                    .required(true),
            ),
        )
}
