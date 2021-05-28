use clap::{App, Arg, SubCommand};

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("update")
             .about("Update a field for a ticket")
               .arg(Arg::with_name("field")
                .short("f")
                .long("field")
                .takes_value(true)
                .required(true)
                .help("Key of field to update. You can use jira-terminal fields <TICKET> to see possible set of keys.")
                )
               .arg(Arg::with_name("value")
                .short("v")
                .long("value")
                .takes_value(true)
                .required(true)
                .help("Value of the field to update.")
                )
            .arg(Arg::with_name("TICKET")
                .help("Ticket ID to update")
                .required(true)
                .index(1))
}
