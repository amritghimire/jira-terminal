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
       .subcommand(SubCommand::with_name("list")
           .about("List the issues from JIRA.")
           .arg(Arg::with_name("project")
               .help("Project Code to filter with.")
               .short("p")
               .long("project")
               .value_name("PROJECT")
               .takes_value(true)
               .multiple(true)
               )
            .arg(Arg::with_name("assignee")
               .help("Assignee username or email to filter with.")
               .short("a")
               .long("assignee")
               .value_name("ASIGNEE")
               .takes_value(true)
               .multiple(true)
               )
             .arg(Arg::with_name("me")
               .help("Issues assigned to you.")
               .short("M")
               .long("me")
               .value_name("ME")
               .takes_value(false)
               )
            .arg(Arg::with_name("component")
               .help("Component name or ID to filter with.")
               .short("c")
               .long("component")
               .value_name("COMPONENT")
               .takes_value(true)
               .multiple(true)
               )
             .arg(Arg::with_name("display")
               .short("d")
               .long("display")
               .long_help(" Comma separated list of fields to display.
Possible options for fields are: 
key,resolution,priority,assignee,status,components,creator,reporter,issuetype,project,summary

You can pass alias as option for display. You can save alias using alias subcommand for the application.

 Default options are
 key,summary,status,assignee
                   ")
               .value_name("DISPLAY")
               .takes_value(true)
               )
            .arg(Arg::with_name("epic")
               .help("EPIC name or issue key of epic to filter with.")
               .short("e")
               .long("epic")
               .value_name("EPIC")
               .takes_value(true)
               .multiple(true)
               )
            .arg(Arg::with_name("filter")
               .help("Filter name or filter id that you saved in JIRA.")
               .short("f")
               .long("filter")
               .value_name("FILTER")
               .takes_value(true)
               .multiple(true)
               )
            .arg(Arg::with_name("jql")
               .help("JQL Query or alias to JQL query to filter with.")
               .short("j")
               .long("jql")
               .value_name("JQL")
               .takes_value(true)
               )
            .arg(Arg::with_name("labels")
               .help("Search for issues with a label or list of labels.")
               .short("l")
               .long("label")
               .value_name("LABEL")
               .takes_value(true)
               .multiple(true)
               )
            .arg(Arg::with_name("parent")
               .help("Search for subtask of a particular issue.")
               .short("m")
               .long("main")
               .value_name("PARENT")
               .takes_value(true)
               .multiple(true)
               )
            .arg(Arg::with_name("priority")
               .help("Search for issues with a particular priority.")
               .short("P")
               .long("priority")
               .value_name("PRIORITY")
               .takes_value(true)
               .multiple(true)
               )
            .arg(Arg::with_name("reporter")
               .help("Search for issues that were reported by a particular user.")
               .short("r")
               .long("reporter")
               .value_name("REPORTER")
               .takes_value(true)
               .multiple(true)
               )
           .arg(Arg::with_name("sprint")
               .help("Search for issues that are assigned to a particular sprint.")
               .short("s")
               .long("sprint")
               .value_name("SPRINT")
               .takes_value(true)
               .multiple(true)
               )
            .arg(Arg::with_name("status")
               .help("Search for issues that have a particular status.")
               .short("S")
               .long("status")
               .value_name("STATUS")
               .takes_value(true)
               .multiple(true)
               )
            .arg(Arg::with_name("type")
               .help("Search for issues that have a particular issue type. ")
               .short("t")
               .long("type")
               .value_name("TYPE")
               .takes_value(true)
               .multiple(true)
               )
            .arg(Arg::with_name("text")
               .help("This is a master-field that allows you to search all text fields for issues.")
               .short("T")
               .long("text")
               .value_name("TEXT")
               .takes_value(true)
               )
            .arg(Arg::with_name("alias")
               .help("Save the applied options as an alias. You can use it with jql option later.")
               .short("A")
               .long("alias")
               .value_name("ALIAS")
               .takes_value(true)
               )
           )
        .get_matches();

    if let Some(transitions) = matches.subcommand_matches("transition") {
        jira::handle_transition_matches(transitions);
    } else if let Some(lists) = matches.subcommand_matches("list") {
        jira::handle_list_matches(lists);
    }
}
