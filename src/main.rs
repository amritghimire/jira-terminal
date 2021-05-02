//! # JIRA Terminal Application
//!
//! This is a command line application that can be used as a personal productivity tool for
//! interacting with JIRA.
//!
//! # Installing
//! You can download the latest binary from [https://github.com/amritghimire/jira-terminal/releases](https://github.com/amritghimire/jira-terminal/releases)
//! After downloading the binary, place it inside `~/.local/bin`
//!
//! # Usage
//! ### First Run
//! You can open the jira terminal for the first time by just entering
//! ```
//! jira_terminal
//! ```
//! Upon first run, it will ask you with the namespace, email and token.
//! If your JIRA Dashboard starts with format https://example.atlassian.net, your namespace is
//! example.
//! Similarly, you can create a token from [https://id.atlassian.com/manage-profile/security/api-tokens](https://id.atlassian.com/manage-profile/security/api-tokens)
//!
//!
mod config;
mod jira;

fn main() {
    println!("Starting JIRA Terminal.");
    config::ensure_config();
}
