extern crate clap;
use clap::ArgMatches;

use crate::config;
use crate::jira::api;

fn display_content(option: &json::JsonValue, value: &json::JsonValue) {
    let mut content: String;
    if value.is_array() {
        let mut contents: Vec<String> = vec![];
        let field = option["field"].as_str().unwrap_or("name");
        for entry in value.members() {
            contents.push(String::from(entry[field].as_str().unwrap_or("-")))
        }
        content = contents.join(", ");
    } else if value.is_object() {
        let field = option["field"].as_str().unwrap_or("name");
        content = String::from(value[field].as_str().unwrap_or("-"))
    } else {
        content = String::from(value.as_str().unwrap_or("-"));
    }
    let width = option["width"].as_usize().unwrap_or(0);
    content.truncate(width);
    print!("{content:width$}|")
}

fn return_json(option: &json::JsonValue, value: &json::JsonValue) -> json::JsonValue {
    if value.is_array() {
        let mut contents = json::JsonValue::new_array();
        let field = option["field"].as_str().unwrap_or("name");
        for entry in value.members() {
            let _ = contents.push(String::from(entry[field].as_str().unwrap_or("-")));
        }
        contents
    } else if value.is_object() {
        let field = option["field"].as_str().unwrap_or("name");
        value[field].clone()
    } else {
        value.clone()
    }
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
            if let Some(jql) = jql_option {
                criterias.push(config::get_alias_or(jql.to_string()));
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
    criterias.join(" AND ")
}

pub fn list_issues(matches: &ArgMatches) {
    let show_json = matches.is_present("json");
    let jql = form_jql(matches);
    let offset_result = matches.value_of("offset").unwrap_or("0").parse::<u32>();
    if offset_result.is_err() {
        eprintln!("Invalid option passed to offset. ");
        std::process::exit(1);
    }
    let offset = offset_result.unwrap();
    let count_result = matches.value_of("count").unwrap_or("50").parse::<u32>();
    if count_result.is_err() {
        eprintln!("Invalid option passed to count. ");
        std::process::exit(1);
    }
    let count = count_result.unwrap();
    let search_response = api::get_call_v3(format!(
        "search?maxResults={count}&startAt={offset}&jql={jql}"
    ));
    if search_response.is_err() {
        eprintln!("Error occurred when searching tickets. ");
        std::process::exit(1);
    }
    if matches.is_present("alias") {
        let alias_name = matches.value_of("alias").unwrap();
        config::set_alias(alias_name.to_string(), jql);
        println!("Current filter is now set with value {alias_name}");
        println!("You can use jira-terminal list --jql \"{alias_name}\" to reuse this filter.");
    }
    let issues = &search_response.unwrap()["issues"];

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
    let headers_to_display = display;
    let headers = headers_to_display.trim().split(',');

    if show_json {
        let mut response = json::JsonValue::new_array();
        for issue in issues.members() {
            let mut data = json::JsonValue::new_object();
            for header in headers.clone() {
                if header == "key" {
                    data[header] = return_json(&display_options[header], &issue[header]);
                } else {
                    data[header] = return_json(&display_options[header], &issue["fields"][header]);
                }
            }
            let _ = response.push(data);
        }
        println!("{}", response.pretty(4));
        return;
    }

    if !issues.is_array() {
        println!("No issues found for the filter.");
        std::process::exit(0);
    }
    let mut total = 0;
    for header in headers.clone() {
        if display_options[header].is_null() {
            eprintln!("Unknown display option {header} passed. ");
            std::process::exit(1);
        }
        display_header(&display_options[header]);
        total = total + display_options[header]["width"].as_usize().unwrap_or(0) + 1;
    }
    println!();
    println!("{:->width$}", "", width = total);
    for issue in issues.members() {
        for header in headers.clone() {
            if header == "key" {
                display_content(&display_options[header], &issue[header]);
            } else {
                display_content(&display_options[header], &issue["fields"][header]);
            }
        }
        println!();
    }
}
