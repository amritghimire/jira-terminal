use crate::config;
use crate::jira::api;
use clap::ArgMatches;
use json;
use std::io::{stdin, BufRead};

#[derive(Debug, Default)]
struct CreationPayload {
    project: String,
    parent: Option<String>,
    issuetype: Option<String>,
    labels: Option<Vec<String>>,
    priority: Option<String>,
    summary: String,
    description: Option<String>,
    components: Option<Vec<String>>,
    assignee: String,
    custom: Option<String>,
}

impl CreationPayload {
    fn json(self) -> json::JsonValue {
        let mut payload = json::object! {
            "project": {
                "key": self.project
            },
            "summary": self.summary,
            "assignee": {
                "accountId": self.assignee
            }
        };
        if let Some(parent) = self.parent {
            payload["parent"]["key"] = parent.into();
        }
        if let Some(issuetype) = self.issuetype {
            payload["issuetype"]["id"] = issuetype.into();
        }
        if let Some(labels) = self.labels {
            payload["labels"] = labels.into();
        }
        if let Some(priority) = self.priority {
            payload["priority"] = priority.into();
        }
        if let Some(description) = self.description {
            payload["description"] = description.into();
        }
        if let Some(components) = self.components {
            let mut component_lists: Vec<json::JsonValue> = vec![];
            for component in components {
                component_lists.push(json::object! {
                    "name": component
                });
            }
            payload["components"] = component_lists.into();
        }
        if let Some(custom) = self.custom {
            let custom_fields = custom.split(",");
            for custom_field in custom_fields {
                if let Some((key, value)) = custom_field.split_once(":") {
                    payload[config::get_alias_or(key.to_string())] =
                        config::get_alias_or(value.to_string()).into();
                }
            }
        }

        json::object! {
            "fields": payload
        }
    }
}

fn split_and_apply_alias(entries: Option<String>) -> Option<Vec<String>> {
    if entries.is_none() {
        return None;
    }
    let entry_value = entries.unwrap();
    let entry_list = entry_value.split(",");
    let mut entry_vector: Vec<String> = vec![];
    for entry in entry_list {
        entry_vector.push(config::get_alias_or(entry.to_string()));
    }
    return Some(entry_vector);
}

fn get_account_id(query: String) -> String {
    let url = format!("user/search?query={}", query);
    let api_response = api::get_call_v3(url);
    if api_response.is_err() {
        eprintln!(
            "Cannot search for provided assignee user. {:?}",
            api_response
        );
        return String::new();
    }
    let account_response = &api_response.unwrap()[0];
    if account_response.is_null() {
        eprintln!("Cannot search for provided assignee user. ");
        return String::new();
    }
    println!(
        "Assigning the ticket to {}",
        account_response["displayName"]
    );
    let account_id = String::from(account_response["accountId"].as_str().unwrap());
    account_id
}

fn get_issuetype_id(project: String, entry: Option<String>) -> Option<String> {
    if entry.is_none() {
        return None;
    }
    let name = entry.unwrap();
    let url = format!("issue/createmeta?projectKeys={}", project);
    let api_response = api::get_call_v3(url);
    if api_response.is_err() {
        eprintln!("Error while verifying issue type: {:?}", api_response);
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

fn get_or_ask(matches: &ArgMatches, key: &str, message: &str) -> Option<String> {
    if matches.is_present(key) {
        return Some(matches.value_of(key).unwrap().to_string());
    }
    if matches.is_present("quiet") {
        return None;
    }
    if matches.is_present("minimal") && key != "summary" {
        return None;
    }
    let mut entry = String::new();
    println!("{} (Press enter to leave it to default.)", message);
    stdin()
        .read_line(&mut entry)
        .expect("Failed to read input.");
    if entry.trim().is_empty() {
        return None;
    }
    Some(config::get_alias_or(entry.trim().to_string()))
}

pub fn handle_issue_creation(matches: &ArgMatches) {
    let mut project: String = matches.value_of("project").unwrap_or("").to_string();
    let mut parent: Option<String> = None;
    if matches.is_present("main") {
        let main = matches.value_of("main").unwrap();
        let split = main.split_once("-");
        if split.is_none() {
            eprintln!("Invalid ticket id passed as main option.");
            return;
        }
        project = split.unwrap().0.to_string();
        parent = Some(main.to_string());
    }
    if project.is_empty() {
        eprintln!("Cannot determine project. ");
        return;
    }
    let summary = get_or_ask(
        &matches,
        "summary",
        "Please enter the summary of the project: ",
    );
    if summary.is_none() {
        println!("Summary is a required field.");
        return;
    }
    let mut description = matches.value_of("description").unwrap_or("").to_string();
    if !matches.is_present("quiet") && description.is_empty() {
        println!("Please enter the description of issue. (Use ctrl+d to end the description)");
        let input = stdin();
        let mut line = String::new();
        let mut stream = input.lock();
        while let Ok(n) = stream.read_line(&mut line) {
            if n == 0 {
                break;
            }
            description = format!("{}\n{}", description, line);

            line = String::new();
        }
    }
    let assignee = if matches.is_present("assignee") {
        let assignee_query = matches.value_of("assignee").unwrap();
        get_account_id(assignee_query.to_string())
    } else {
        config::get_config("account_id".to_string())
    };
    if assignee.is_empty() {
        println!("Please provide appropriate user email to continue.");
        return;
    }
    let issuetype = get_or_ask(matches, "type", "Please enter type of issue: ");
    let labels = get_or_ask(
        matches,
        "labels",
        "Please enter comma separated list of labels to assign: ",
    );
    let priority = get_or_ask(
        matches,
        "priority",
        "Please enter the priority of the ticket: ",
    );
    let components = get_or_ask(
        matches,
        "components",
        "Please enter Comma separated list of components of ticket: ",
    );
    let custom = if matches.is_present("custom") {
        Some(matches.value_of("custom").unwrap_or("").to_string())
    } else {
        None
    };
    let payload = CreationPayload {
        project: project.clone(),
        parent,
        issuetype: get_issuetype_id(project, issuetype),
        priority,
        custom,
        summary: summary.unwrap(),
        labels: split_and_apply_alias(labels),
        components: split_and_apply_alias(components),
        assignee: assignee.clone(),
        description: if description.is_empty() {
            None
        } else {
            Some(description)
        },
        ..Default::default()
    };
    let json_node = payload.json();
    let created_api_response = api::post_call(format!("issue"), json_node, 2);
    if created_api_response.is_err() {
        eprintln!("Unable to create ticket. {:?}", created_api_response);
    }
    let response = json::parse(&created_api_response.unwrap());
    if response.is_ok() {
        let key = &response.unwrap()["key"];
        println!("New Ticket KEY: {} Created.", key);
    }
}
