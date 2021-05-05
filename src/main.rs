//! # JIRA Terminal Application
//!
//! This is a command line application that can be used as a personal productivity tool for
//! interacting with JIRA.
//!
//! # Installing
//! You can download the latest binary from [https://github.com/amritghimire/jira-terminal/releases](https://github.com/amritghimire/jira-terminal/releases)
//! After downloading the binary, place it inside `~/.local/bin`
//!
//! # Usage
//! ### First Run
//! You can open the jira terminal for the first time by just entering
//! ```
//! jira_terminal
//! ```
//! Upon first run, it will ask you with the namespace, email and token.
//! If your JIRA Dashboard starts with format https://example.atlassian.net, your namespace is
//! example.
//! Similarly, you can create a token from [https://id.atlassian.com/manage-profile/security/api-tokens](https://id.atlassian.com/manage-profile/security/api-tokens)
//!
//!
extern crate clap;
use clap::{App, Arg, SubCommand};

mod config;
mod jira;

fn main() {
    config::ensure_config();
    let matches = App::new("JIRA Terminal").version("1.0")
        .author("Amrit Ghimire <oss@amritghimire.com>")
        .about("This is a command line application that can be used as a personal productivity tool for interacting with JIRA")
       .subcommand(SubCommand::with_name("transition")
            .about("Transition of ticket across status.")
            .arg(Arg::with_name("STATUS")
                .help("Status or alias of status to move the ticket to.")
                .required_unless("transition_list")
                .index(1))
            .arg(Arg::with_name("transition_ticket")
                .short("t")
                .long("ticket")
                .value_name("TICKET")
                .help("Ticket ID from JIRA.")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("transition_list")
                .short("l")
                .long("list")
                .help("List the possible transitions.")
                .takes_value(false))
           ) 
        .get_matches();

    if let Some(transitions) = matches.subcommand_matches("transition") {
        jira::handle_transition_matches(transitions);
    }
}
