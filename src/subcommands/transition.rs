use clap::{App, Arg, SubCommand};

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("transition")
        .about("Transition of ticket across status.")
        .arg(
            Arg::with_name("STATUS")
                .help("Status or alias of status to move the ticket to.")
                .required_unless("transition_list")
                .index(1),
        )
        .arg(
            Arg::with_name("transition_ticket")
                .short("t")
                .long("ticket")
                .value_name("TICKET")
                .help("Ticket ID from JIRA.")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("transition_list")
                .short("l")
                .long("list")
                .help("List the possible transitions.")
                .takes_value(false),
        )
}
