pub mod api;
pub mod comments;
pub mod details;
mod fields;
pub mod lists;
mod new_issue;
pub mod transitions;
mod update;

extern crate clap;
use clap::ArgMatches;

pub fn handle_transition_matches(matches: &ArgMatches) {
    let ticket = matches.value_of("transition_ticket").unwrap();
    if matches.is_present("transition_list") {
        transitions::print_transition_lists(ticket.to_string());
    } else {
        let status = matches.value_of("STATUS").unwrap();
        transitions::move_ticket_status(ticket.to_string(), status.to_string());
    }
}

pub fn handle_fields_matches(matches: &ArgMatches) {
    let ticket = String::from(matches.value_of("TICKET").unwrap());
    fields::display_all_fields(ticket);
}

pub fn handle_list_matches(matches: &ArgMatches) {
    lists::list_issues(matches);
}

pub fn handle_detail_matches(matches: &ArgMatches) {
    let ticket = String::from(matches.value_of("TICKET").unwrap());
    let fields = String::from(
        matches
            .value_of("fields")
            .unwrap_or("key,summary,description"),
    );
    details::show_details(ticket, fields);
}

pub fn handle_update_matches(matches: &ArgMatches) {
    let ticket = String::from(matches.value_of("TICKET").unwrap());
    let field = String::from(matches.value_of("field").unwrap());
    let value = String::from(matches.value_of("value").unwrap());
    update::update_jira_ticket(ticket, field, value);
}

pub fn handle_new_matches(matches: &ArgMatches) {
    new_issue::handle_issue_creation(&matches);
}
