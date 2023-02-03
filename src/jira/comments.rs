use chrono::DateTime;
use clap::ArgMatches;
use regex::Captures;
use regex::Regex;
use std::io::{stdin, BufRead};

use crate::config;
use crate::jira::api;
use crate::jira::utils::get_account_id;

fn get_display_name_for_user(account_id: String) -> String {
    let config_object = config::parse_config();
    let cached_name = &config_object["accounts"][account_id.clone()];
    if !cached_name.is_empty() {
        return cached_name.as_str().unwrap().to_string();
    }
    let details_response = api::get_call_v2(format!("user/?accountId={account_id}"));
    if details_response.is_err() {
        return format!("[{account_id}]");
    }
    let display_name = &details_response.unwrap()["displayName"];
    if display_name.is_empty() {
        return format!("[{account_id}]");
    }
    let mut accounts = config_object["accounts"].clone();
    accounts[account_id] = display_name.as_str().unwrap().to_string().into();
    config::update_config_object("accounts".to_string(), accounts);
    display_name.as_str().unwrap().to_string()
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
        format!("@({}) ", get_display_name_for_user(caps[1].to_string()))
    });
    println!("{result}");
    println!("\n");
}

fn change_mentioned_users(body: String) -> String {
    let re = Regex::new(r"@\(([^)]*)\)").unwrap();
    let result = re.replace_all(&body, |caps: &Captures| {
        format!("[~accountid:{}] ", get_account_id(caps[1].to_string()))
    });
    result.to_string()
}

pub fn display_comment_list(comments: &json::JsonValue) {
    let total_comment = &comments["total"];
    println!("Total {total_comment} comment found. ");
    println!();
    let re = Regex::new(r"\[~accountid:([^\]]*)\]").unwrap();
    for comment in comments["comments"].members() {
        display_comment_object(comment, &re);
    }
}

pub fn get_all_comments(ticket: String) {
    let comments_response = api::get_call_v2(format!("issue/{ticket}/comment"));
    if comments_response.is_err() {
        eprintln!("Cannot fetch the comments.");
        std::process::exit(1);
    }
    display_comment_list(&comments_response.unwrap());
}

pub fn add_new_comment(ticket: String, matches: &ArgMatches) {
    let mut body = matches.value_of("body").unwrap_or("").to_string();
    if body.is_empty() {
        println!("Please enter the body of comment. (Use ctrl+d to end the body)");
        let input = stdin();
        let mut line = String::new();
        let mut stream = input.lock();
        while let Ok(n) = stream.read_line(&mut line) {
            if n == 0 {
                break;
            }
            body = format!("{body}\n{line}");

            line = String::new();
        }
    }
    let payload = json::object! {
        "body": change_mentioned_users(body)
    };
    let update_response = api::post_call(format!("issue/{ticket}/comment"), payload, 2);
    if update_response.is_err() {
        eprintln!("Error occurred while adding comment.");
        std::process::exit(1);
    }
    let response = json::parse(&update_response.unwrap());
    println!("Successfully Added a new comment");
    if let Ok(response_object) = response {
        let comment = &response_object;
        let re = Regex::new(r"@\(([^)]*)\)").unwrap();
        display_comment_object(comment, &re);
    }
}
