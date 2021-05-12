extern crate clap;
use clap::ArgMatches;

#[path = "./api.rs"]
mod api;

#[path = "../config/mod.rs"]
mod config;

fn display_content(option: &json::JsonValue, value: &json::JsonValue, header: String) {
    let mut content: String;
    if value.is_array() {
        let mut contents: Vec<String> = vec![];
        let field = option["field"].as_str().unwrap_or("name");
        for entry in value.members() {
            contents.push(String::from(entry[field].as_str().unwrap_or("-")))
        }
        content = String::from(contents.join(", "));
    } else if value.is_object() {
        let field = option["field"].as_str().unwrap_or("name");
        content = String::from(value[field].as_str().unwrap_or("-"))
    } else {
        content = String::from(value.as_str().unwrap_or("-"));
    }
    let width = option["width"].as_usize().unwrap_or(0);
    content.truncate(width);
    print!("{value:width$}|", value = content, width = width)
}

fn display_header(option: &json::JsonValue) {
    print!(
        "{title:width$}|",
        title = option["title"].as_str().unwrap_or(" "),
        width = option["width"].as_usize().unwrap_or(0)
    )
}

fn form_jql(matches: &ArgMatches) -> String {
    let mut criterias: Vec<String> = vec![];
    let fields = vec![
        "assignee",
        "component",
        "labels",
        "parent",
        "filter",
        "priority",
        "project",
        "reporter",
        "sprint",
        "status",
        "type",
        "epic",
        "jql",
        "text",
    ];
    if matches.is_present("me") {
        criterias.push("assignee = currentUser()".to_string());
    }
    for field in fields {
        if field == "jql" {
            let jql_option = matches.value_of("jql");
            if jql_option.is_some() {
                criterias.push(config::get_alias_or(jql_option.unwrap().to_string()));
            }
        } else if field == "text" {
            let jql_option = matches.value_of("text");
            if jql_option.is_some() {
                criterias.push(format!(
                    "text ~ \"{}\"",
                    config::get_alias_or(jql_option.unwrap().to_string())
                ));
            }
        } else if let Some(values) = matches.values_of(field) {
            let mut options: Vec<String> = vec![];
            for value in values {
                options.push(format!("\"{}\"", config::get_alias_or(value.to_string())));
            }
            if field == "epic" {
                criterias.push(format!("\"epic link\" in ({})", options.join(",")));
            } else {
                criterias.push(format!("{} in ({})", field, options.join(",")));
            }
        }
    }
    let jql = criterias.join(" AND ");
    jql
}

pub fn list_issues(matches: &ArgMatches) {
    let jql = form_jql(matches);
    let search_response = api::get_call_v3(format!("search?jql={}", jql));
    if search_response.is_err() {
        println!("Error occured when searching tickets. ");
        println!("{:?}", search_response);
        return;
    }
    if matches.is_present("alias") {
        let alias_name = matches.value_of("alias").unwrap();
        config::set_alias(alias_name.to_string(), jql);
        println!("Current filter is now set with value {}", alias_name);
        println!(
            "You can use jira-terminal list --jql \"{}\" to reuse this filter.",
            alias_name
        );
    }
    let issues = &search_response.unwrap()["issues"];
    if !issues.is_array() {
        println!("No issues found for the filter.");
        return;
    }
    let display: String = String::from(
        matches
            .value_of("display")
            .unwrap_or("key,summary,status,assignee"),
    );
    let display_options = json::object! {
        "key": {"title": "Key", "width": 10},
        "resolution": {"title": "Resolution", "width": 10, "field": "name"},
        "priority": {"title": "Priority", "width": 10, "field": "name"},
        "assignee": {"title": "Assignee", "width": 20, "field": "displayName"},
        "status": {"title": "Status", "width": 15, "field": "name"},
        "components": {"title": "Components", "width": 30, "field": "name"},
        "creator": {"title": "Creator", "width": 15, "field": "displayName"},
        "reporter": {"title": "Reporter", "width": 15, "field": "displayName"},
        "issuetype": {"title": "Issue Type", "width": 10, "field": "name"},
        "project": {"title": "Project", "width": 15, "field": "name"},
        "summary": {"title": "Summary", "width": 100}
    };
    let headers_to_display = display.clone();
    let headers = headers_to_display.trim().split(",");
    let mut total = 0;
    for header in headers.clone() {
        if display_options[header].is_null() {
            eprintln!("Unknown display option {} passed. ", header);
            return;
        }
        display_header(&display_options[header]);
        total = total + display_options[header]["width"].as_usize().unwrap_or(0) + 1;
    }
    println!("");
    println!("{:->width$}", "", width = total);
    for issue in issues.members() {
        for header in headers.clone() {
            if header == "key" {
                display_content(
                    &display_options[header],
                    &issue[header],
                    String::from(header),
                );
            } else {
                display_content(
                    &display_options[header],
                    &issue["fields"][header],
                    String::from(header),
                );
            }
        }
        println!("");
    }
}
