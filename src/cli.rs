use clap::{Arg, arg, Command};

pub fn cli() -> Command {
    Command::new("bl")
        .about("Fast package manager for Node.js")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("install")
                .about("Installs dependencies")
                .args([Arg::new("package_name")
                    .help("name of the package.")
                    .required(false)
                    .num_args(0..=200)
                ])
                .args(
                    [arg!(--dev)
                        .help("Installs the package as a development dependency")
                        .long("dev")
                        .short('D')
                    ]
                )
        )
        .subcommand(
            Command::new("init")
                .about("Initializes a new project")
                .args([arg!(--name <WHEN>)
                    .num_args(0..=1)
                    .id("package_name")
                    .help("Sets the name of the project")
                ])
        )
        .subcommand(
            Command::new("cache")
                .about("Manages the cache")
                .subcommand(
                    Command::new("clear")
                        .about("Clears the cache")
                )
                .subcommand(
                    Command::new("path")
                        .about("Prints the path to the cache")
                )
        )
        .subcommand(
            Command::new("run")
                .about("Runs a script defined in package.json")
                .args([Arg::new("script_name")
                    .help("name of the script.")
                    .required(true)
                    .num_args(1)
                ])
        )
        .subcommand(
            Command::new("start")
                .about("Runs the start script defined in package.json")
        )
}
