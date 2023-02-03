use crate::config;
use crate::jira::api;

/// Get the project code from ticket id.
/// Example for ticket id: ABC-123, project code will be ABC.
///
/// # Arguments
///
/// * `ticket` - Ticket id for jira.
///
/// # Example
/// ```
/// let project_code = get_project_code("ABC-123".to_string());
/// ```
fn get_project_code(ticket: String) -> String {
    String::from(ticket.split('-').next().unwrap())
}

/// Get the transition Object itself from JIRA server.
///
/// # Arguments
///
/// * `ticket` - Ticket ID for JIRA.
///
/// # Example
/// ```
/// let transition_object = get_transitions_object("ABC".to_string());
/// match transition_object {
///     Some(x) => println!("{:?}", x),
///     None => println!("API Error. Transition Object not returned.")
/// }
/// ```
fn get_transitions(ticket: String) -> Option<json::JsonValue> {
    let transitions_response = api::get_call_v3(format!("issue/{ticket}/transitions"));
    if transitions_response.is_err() {
        return None;
    }
    let transitions = &transitions_response.unwrap()["transitions"];
    if !transitions.is_array() {
        return None;
    }
    let mut transition_object = json::object! {};
    for transition in transitions.members() {
        let name = String::from(transition["name"].as_str().unwrap()).to_lowercase();
        let id: u16 = transition["id"].as_str().unwrap().parse().unwrap();
        transition_object[name] = id.into();
    }
    let project_code = get_project_code(ticket);
    config::set_transitions(project_code, transition_object);
    Some(transitions.clone())
}

/// Get the transition code used in JIRA.
///
/// # Arguments
///
/// * `ticket` - JIRA Ticket ID
/// * `transition_name` - Case insensitive name for transition. You could also pass alias for
/// transition name.
///
/// # Example
/// ```
/// assert!(get_transition_code("ABC-123", "In Progress").is_some());
/// assert!(get_transition_code("ABC-123", "ip").is_some());
/// assert!(get_transition_code("ABC-123", "Non Existing status").is_none())
/// ```
pub fn get_transition_code(ticket: String, transition_name: String) -> Option<u16> {
    let project_code = get_project_code(ticket.clone());
    let aliased_name = config::get_alias_or(transition_name.to_lowercase()).to_lowercase();
    if !config::transition_exists(project_code.clone(), aliased_name.clone()) {
        get_transitions(ticket);
    }
    let transitioned_object = &config::get_transitions(project_code)[aliased_name];
    if (!transitioned_object.is_null()) && transitioned_object.is_number() {
        return transitioned_object.as_u16();
    }
    None
}

/// Print the list of possible transitions.
///
/// # Arguments
///
/// * `ticket` - Ticket ID
///
/// # Example
///
/// ```
/// print_transition_lists("ABC-1234");
/// ```
pub fn print_transition_lists(ticket: String) {
    let transition_object_response = get_transitions(ticket.clone());
    if transition_object_response.is_none() {
        eprintln!("Cannot find transitions for {ticket}");
        std::process::exit(1);
    }
    let transitions = transition_object_response.unwrap();
    println!("Allowed transitions for {ticket} are as below: ");
    for transition in transitions.members() {
        let name = String::from(transition["name"].as_str().unwrap());
        println!("- {name}");
    }
}

/// Perform transition for the JIRA Ticket.
///
/// # Arguments
///
/// * `ticket` - Ticket ID
/// * `status` - Status
///
/// # Example
///
/// ```
/// move_ticket_status("ABC-1234", "In Progress");
/// ```
pub fn move_ticket_status(ticket: String, status: String) {
    let transition_options = get_transition_code(ticket.clone(), status);
    if transition_options.is_none() {
        eprintln!("Invalid status...");
        std::process::exit(1);
    }
    let transition_code = transition_options.unwrap();
    let json_object = json::object! {
        "transition": {
            "id": transition_code
        }
    };
    let transitions_response =
        api::post_call(format!("issue/{ticket}/transitions"), json_object, 3);
    if transitions_response.is_err() {
        eprintln!("Unable to perform transition.");
        std::process::exit(1);
    }
    let response = transitions_response.unwrap();
    println!("Successfully Completed {response}");
}
