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
                .args([Arg::new("package_name").help("name of the package.").required(true)])
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
}
