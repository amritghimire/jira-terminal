use crate::config;
use crate::jira::{api, utils};

pub fn assign_task(ticket: String, user: String) {
    let aliased_query = config::get_alias_or(user);
    let account_id = utils::get_account_id(aliased_query);
    let payload = json::object! {
        "accountId": account_id
    };
    let update_response = api::put_call(format!("issue/{ticket}/assignee"), payload, 3);
    if update_response.is_err() {
        eprintln!("Error occurred While assigning the ticket.");
        std::process::exit(1);
    }
    let response = update_response.unwrap();
    println!("Successfully Assigned {response}");
}
