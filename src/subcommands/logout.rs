use clap::{App, SubCommand};

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("logout").about("Erase configuration and log out of Jira")
}
