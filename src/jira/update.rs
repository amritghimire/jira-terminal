use crate::config;
use crate::jira::api;

fn get_object_by_name(list: &json::JsonValue, name: String) -> Option<json::JsonValue> {
    for member in list.members() {
        if member["name"]
            .as_str()
            .unwrap_or("")
            .to_string()
            .to_lowercase()
            == name.to_lowercase()
        {
            return Some(member.to_owned());
        }
    }
    None
}

fn get_object_lists_from_value(list: &json::JsonValue, value: String) -> Vec<json::JsonValue> {
    let mut selected_entries: Vec<json::JsonValue> = vec![];
    for name in value.split(",") {
        let object = get_object_by_name(&list, config::get_alias_or(name.to_string()));
        if object.is_some() {
            selected_entries.push(object.unwrap());
        }
    }
    selected_entries
}

pub fn update_jira_ticket(ticket: String, key: String, entry: String) {
    let value = config::get_alias_or(entry);
    let update_key = config::get_alias_or(key);
    let fields_response = api::get_call_v2(format!("issue/{}/editmeta", ticket));
    if fields_response.is_err() {
        println!("Error occured in API Call: {:?}", fields_response);
        return;
    }
    let fields = &fields_response.unwrap()["fields"][update_key.clone()];
    let mut update_json = json::object! {};
    if fields.is_null() {
        println!("Cannot fetch fields");
        return;
    }
    if !fields["autoCompleteUrl"].is_null() {
        println!("Cannot update provided key.");
        return;
    }
    if update_key == "comment" || update_key == "assignee" {
        println!("Comment and assignee has their own section. Please check help for details.");
        return;
    }

    if fields["allowedValues"].is_array() {
        if fields["schema"]["type"] == "array" {
            let update_json_value = get_object_lists_from_value(&fields["allowedValues"], value);
            update_json[update_key.clone()] = update_json_value.into();
        } else {
            let update_json_value = get_object_by_name(&fields["allowedValues"], value);
            update_json[update_key.clone()] = update_json_value.into();
        }
    } else if fields["schema"]["type"] == "array" {
        let values: Vec<&str> = value.split(",").collect();
        update_json[update_key.clone()] = values.into();
    } else {
        update_json[update_key.clone()] = value.into();
    }

    let payload = json::object! {
        "fields": update_json
    };
    let update_response = api::put_call(format!("issue/{}", ticket), payload, 3);
    if update_response.is_err() {
        println!("Error occured in API Call: {:?}", update_response);
        return;
    }
    let response = update_response.unwrap();
    println!("Successfully Updated {}", response);
}
