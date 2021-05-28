use clap::{App, Arg, SubCommand};

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("fields")
        .about("List of possible Fields for details...")
        .arg(
            Arg::with_name("TICKET")
                .help("Ticket id for details.")
                .required(true)
                .index(1),
        )
}
