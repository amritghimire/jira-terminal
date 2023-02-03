use crate::jira::api;

pub fn get_account_id(query: String) -> String {
    let url = format!("user/search?query={query}");
    let api_response = api::get_call_v3(url);
    if api_response.is_err() {
        eprintln!("Cannot search for provided assignee user. {api_response:?}");
        return String::new();
    }
    let account_response = &api_response.unwrap()[0];
    if account_response.is_null() {
        eprintln!("Cannot search for provided assignee user. ");
        return String::new();
    }
    println!("Selecting user {}", account_response["displayName"]);
    let account_id = String::from(account_response["accountId"].as_str().unwrap());
    account_id
}

pub fn get_issuetype_id(project: String, entry: Option<String>) -> Option<String> {
    let name = entry.as_ref()?;
    let url = format!("issue/createmeta?projectKeys={project}");
    let api_response = api::get_call_v3(url);
    if api_response.is_err() {
        eprintln!("Error while verifying issue type: {api_response:?}");
        return None;
    }
    let project_list = &api_response.unwrap()["projects"];
    for project in project_list.members() {
        let issuetypes = &project["issuetypes"];
        for issuetype in issuetypes.members() {
            if issuetype["name"].as_str().unwrap_or("").to_lowercase() == name.to_lowercase() {
                return Some(issuetype["id"].as_str().unwrap_or("").to_string());
            }
        }
    }
    None
}
