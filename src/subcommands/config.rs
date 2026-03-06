use clap::{App, Arg, SubCommand};

/// Configure user-specific settings for this CLI.
/// For example, use `jira-version` to set the Jira API version used for requests.
pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("config")
        .about("Update configuration values.")
        .arg(
            Arg::with_name("KEY")
                .help("Configuration key to update (e.g. jira-version)")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("VALUE")
                .help("Value to set (e.g. 2, v2, 3, v3)")
                .required(true)
                .index(2),
        )
}

/// Parse a jira-version string (2, v2, 3, v3) into a canonical version number string.
fn parse_jira_version(value: &str) -> Option<&'static str> {
    match value.to_lowercase().trim_start_matches('v') {
        "2" => Some("2"),
        "3" => Some("3"),
        _ => None,
    }
}

pub fn handle(matches: &clap::ArgMatches) {
    let key = matches.value_of("KEY").unwrap();
    let value = matches.value_of("VALUE").unwrap();

    match key {
        "jira-version" => match parse_jira_version(value) {
            Some(version) => {
                crate::config::update_config("version".to_string(), version.to_string());
                println!("Jira API version set to {version}.");
            }
            None => {
                eprintln!("Invalid jira-version '{value}'. Accepted values: 2, v2, 3, v3.");
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Unknown config key '{key}'.");
            std::process::exit(1);
        }
    }
}
