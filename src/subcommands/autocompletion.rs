use clap::{App, Arg, SubCommand};

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("autocompletion")
        .about("Generate autocompletion script..")
        .arg(
            Arg::with_name("shell")
                .short("s")
                .required(true)
                .long("shell")
                .takes_value(true)
                .long_help("Name of shell to create the autocompletion. Possible values are:   bash, fish, zsh, powershell, elvish"),
        )
}
