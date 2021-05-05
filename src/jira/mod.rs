pub mod api;
pub mod transitions;

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
