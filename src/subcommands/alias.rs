use clap::{App, Arg, SubCommand};

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("alias")
        .about("Configuration for alias. One of add,list or remove is required.")
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .takes_value(false)
                .required_unless_one(&["add", "remove"])
                .conflicts_with_all(&["add", "remove"])
                .help("List the alias saved."),
        )
        .arg(
            Arg::with_name("add")
                .short("a")
                .long("add")
                .takes_value(true)
                .required_unless_one(&["list", "remove"])
                .conflicts_with_all(&["list", "remove"])
                .help("Value to associate with provided alias name."),
        )
        .arg(
            Arg::with_name("remove")
                .short("r")
                .long("remove")
                .takes_value(false)
                .required_unless_one(&["list", "add"])
                .conflicts_with_all(&["list", "add"])
                .help("List the alias saved."),
        )
        .arg(
            Arg::with_name("NAME")
                .help("Name of alias. (Required except for list option)")
                .required_unless("list")
                .index(1),
        )
}
