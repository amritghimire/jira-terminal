use clap::{App, Arg, SubCommand};

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("comment")
        .about("List or add comments to a ticket. Default action is adding.")
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .takes_value(false)
                .required(false)
                .conflicts_with("body")
                .help("List all the comments of a ticket."),
        )
        .arg(
            Arg::with_name("body")
                .short("b")
                .long("body")
                .takes_value(true)
                .required(false)
                .help("Body of the comment. To mention someone, you can use @(query) The query can include jira username or display name or email address."),
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
