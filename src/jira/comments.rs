use chrono::DateTime;
use json;
use regex::Captures;
use regex::Regex;

use crate::config;
use crate::jira::api;

fn get_display_name_for_user(account_id: String) -> String {
    let config_object = config::parse_config().clone();
    let cached_name = &config_object["accounts"][account_id.clone()];
    if !cached_name.is_empty() {
        return cached_name.as_str().unwrap().to_string();
    }
    let details_response = api::get_call_v2(format!("user/?accountId={}", account_id.clone()));
    if details_response.is_err() {
        return format!("[{}]", account_id);
    }
    let display_name = &details_response.unwrap()["displayName"];
    if display_name.is_empty() {
        return format!("[{}]", account_id);
    }
    let mut accounts = config_object["accounts"].clone();
    accounts[account_id] = display_name.as_str().unwrap().to_string().into();
    config::update_config_object("accounts".to_string(), accounts);
    format!("{}", display_name.as_str().unwrap())
}

fn display_comment_object(comment: &json::JsonValue, re: &Regex) {
    println!(
        "{}",
        comment["author"]["displayName"].as_str().unwrap_or("")
    );
    let rfc3339 = DateTime::parse_from_str(comment["created"].as_str().unwrap_or(""), "%FT%T%.f%z");
    if rfc3339.is_ok() {
        println!("({})", rfc3339.unwrap().format("%v %r"));
        println!("============================\n");
    }
    let comment_body = comment["body"].as_str().unwrap();
    let result = re.replace_all(comment_body, |caps: &Captures| {
        format!("@{} ", get_display_name_for_user(caps[1].to_string()))
    });
    println!("{}", result);
    println!("\n");
}

pub fn display_comment_list(comments: &json::JsonValue) {
    let total_comment = &comments["total"];
    println!("Total {} comment found. ", total_comment);
    println!("");
    let re = Regex::new(r"\[~accountid:([^\]]*)\]").unwrap();
    for comment in comments["comments"].members() {
        display_comment_object(comment, &re);
    }
}
