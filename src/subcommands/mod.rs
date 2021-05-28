pub mod alias;
pub mod assign;
pub mod detail;
pub mod fields;
pub mod list;
pub mod new_subcommand;
pub mod transition;
pub mod update;

use crate::{config, jira};
use clap::ArgMatches;

pub fn handle_matches(matches: ArgMatches) {
    if let Some(transitions) = matches.subcommand_matches("transition") {
        jira::handle_transition_matches(transitions);
    } else if let Some(lists) = matches.subcommand_matches("list") {
        jira::handle_list_matches(lists);
    } else if let Some(aliases) = matches.subcommand_matches("alias") {
        if aliases.is_present("list") {
            config::list_all_alias();
        } else {
            let alias_name = aliases.value_of("NAME").unwrap();
            if aliases.is_present("remove") {
                config::remove_alias(alias_name.to_string());
            } else if aliases.is_present("add") {
                let value = aliases.value_of("add").unwrap();
                config::set_alias(alias_name.to_string(), value.to_string());
                println!("Added new config for {} with value: {}", alias_name, value);
            }
        }
    } else if let Some(details) = matches.subcommand_matches("detail") {
        jira::handle_detail_matches(details);
    } else if let Some(fields) = matches.subcommand_matches("fields") {
        jira::handle_fields_matches(fields);
    } else if let Some(updates) = matches.subcommand_matches("update") {
        jira::handle_update_matches(updates);
    } else if let Some(new_matches) = matches.subcommand_matches("new") {
        jira::handle_new_matches(new_matches);
    } else if let Some(assign) = matches.subcommand_matches("assign") {
        jira::handle_assign_matches(assign);
    }
}
