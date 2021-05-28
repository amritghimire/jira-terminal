use clap::{App, Arg, SubCommand};

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("detail")
             .about("Detail of a JIRA tickets..")
            .arg(Arg::with_name("fields")
                .short("f")
                .long("fields")
                .takes_value(true)
                .long_help("Comma separated lists of fields or alias to show.
Possible options are: 
key,summary,description,status,issuetype,priority,labels,assignee,components,creator,reporter,project,comment
You can view complete list of fields from:
jira-terminal fields KEY-XXXX

You can use all to show all fields.
Default selection are:
key,summary,description
                    ")
                )
            .arg(Arg::with_name("TICKET")
                .help("Ticket id for details.")
                .required(true)
                .index(1))
}
