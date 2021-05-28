use clap::{App, Arg, SubCommand};

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("assign")
        .about("Assign a ticket to user.")
        .arg(
            Arg::with_name("user")
                .short("u")
                .long("user")
                .takes_value(true)
                .required(true)
                .help("Assign the ticket to the provided user."),
        )
        .arg(
            Arg::with_name("ticket")
                .short("t")
                .long("ticket")
                .takes_value(true)
                .required(true)
                .help("Ticket to use."),
        )
}
