use clap::{App, Arg, SubCommand};

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("new")
                .about("Create a new ticket.")
                .arg(Arg::with_name("project")
                        .short("P")
                        .long("project")
                        .required_unless("main")
                        .conflicts_with("main")
                        .help("Project Key to create the ticket.")
                        .takes_value(true)
                    )
                 .arg(Arg::with_name("main")
                        .short("m")
                        .long("main")
                        .help("Main ticket to create the sub-ticket.")
                        .required_unless("project")
                        .conflicts_with("project")
                        .takes_value(true)
                    )
                 .arg(Arg::with_name("type")
                        .short("t")
                        .long("type")
                        .takes_value(true)
                        .help("Issue type for new ticket.")
                     )
                 .arg(Arg::with_name("labels")
                        .short("l")
                        .long("labels")
                        .takes_value(true)
                        .help("Comma separated list of labels.")
                     )
                 .arg(Arg::with_name("priority")
                        .short("p")
                        .long("priority")
                        .takes_value(true)
                        .help("Priority Of the ticket.")
                     )
                .arg(Arg::with_name("summary")
                        .short("s")
                        .long("summary")
                        .takes_value(true)
                        .help("Summary of ticket")
                     )
                .arg(Arg::with_name("description")
                        .short("d")
                        .long("description")
                        .takes_value(true)
                        .help("Description of ticket")
                     )
                .arg(Arg::with_name("components")
                        .short("c")
                        .long("components")
                        .takes_value(true)
                        .help("Comma separated list of components of ticket")
                     )
                .arg(Arg::with_name("assignee")
                        .short("a")
                        .long("assignee")
                        .takes_value(true)
                        .help("Assignee email of ticket")
                     )
                .arg(Arg::with_name("custom")
                        .short("C")
                        .long("custom")
                        .takes_value(true)
                        .help("Comma separated value pair for custom fields. You can use alias in value or key itself. Example- \"customfield_12305:value,alias_to_key:value2. You can use fields subcommand to check the list of custom fields available. ")
                     )
                .arg(Arg::with_name("minimal")
                        .short("M")
                        .long("minimal")
                        .help("Only summary and description will be asked if not available.")
                     )
                .arg(Arg::with_name("quiet")
                        .short("q")
                        .long("quiet")
                        .requires("summary")
                        .help("Do not ask for missing options.")
                     )
}
