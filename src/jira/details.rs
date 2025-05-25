use crate::config;
use crate::jira::api;
use crate::jira::comments;

fn show_detail_field(value: &json::JsonValue, field: String) {
    let hide_keys = [
        "key".to_string(),
        "summary".to_string(),
        "description".to_string(),
    ];
    if value.is_null() {
        return;
    }
    if field == "comment" {
        println!();
        comments::display_comment_list(value);
        return;
    }
    if field == "parent" {
        println!(
            "Parent: {}: {}",
            value["key"].as_str().unwrap_or(""),
            value["fields"]["summary"].as_str().unwrap_or("-")
        );
        return;
    }
    if value.is_object() {
        let mut name = &value["name"];
        if name.is_null() {
            name = &value["displayName"];
        }
        println!(
            "{}: {}",
            config::str_cap(field),
            name.as_str().unwrap_or("-")
        );
        return;
    }
    if value.is_array() {
        let mut contents: Vec<String> = vec![];
        for entry in value.members() {
            if entry.is_object() {
                contents.push(String::from(entry["name"].as_str().unwrap_or("-")))
            } else {
                contents.push(String::from(entry.as_str().unwrap_or("-")))
            }
        }
        println!("{}: {}", config::str_cap(field), contents.join(", "));
        return;
    }
    if value.is_number() {
        let n: f64 = value.as_number().unwrap().into();
        println!("{}: {}", config::str_cap(field), n);
        return;
    }
    if !hide_keys.contains(&field) {
        print!("{}: ", config::str_cap(field));
    }
    println!("{}", value.as_str().unwrap_or("-"));
}

pub fn show_details(ticket: String, fields: String) {
    let fields_expanded = if fields == "all" {
        String::from("key,summary,description,status,issuetype,priority,labels,assignee,components,creator,reporter,project,comment")
    } else {
        config::get_alias_or(fields)
    };
    let details_response = api::get_call_v2(format!("issue/{ticket}"));
    if details_response.is_err() {
        eprintln!("Error occurred when searching tickets. ");
        std::process::exit(1);
    }

    let fields_list = fields_expanded.trim().split(',');
    let detail_object = details_response.unwrap();
    for field in fields_list {
        if field == "key" {
            show_detail_field(&detail_object["key"], "key".to_string());
        } else {
            let mut value = &detail_object["fields"][field];
            if value.is_null() {
                value = &detail_object["fields"][config::get_alias_or(field.to_string())];
            }
            show_detail_field(value, field.to_string());
        }
    }
}
