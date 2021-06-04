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
#[macro_use]
extern crate clap;
extern crate rpassword;
use clap::App;

pub mod api;
pub mod config;
pub mod jira;
pub mod subcommands;

fn main() {
    config::ensure_config();
    let app = App::new("JIRA Terminal")
        .version(crate_version!())
        .author("Amrit Ghimire <oss@amritghimire.com>")
        .about("This is a command line application that can be used as a personal productivity tool for interacting with JIRA")
        .subcommand(subcommands::transition::subcommand())
        .subcommand(subcommands::list::subcommand())
        .subcommand(subcommands::detail::subcommand())
        .subcommand(subcommands::alias::subcommand())
        .subcommand(subcommands::fields::subcommand())
        .subcommand(subcommands::assign::subcommand())
        .subcommand(subcommands::comments::subcommand())
        .subcommand(subcommands::update::subcommand())
        .subcommand(subcommands::autocompletion::subcommand())
        .subcommand(subcommands::new_subcommand::subcommand());

    subcommands::handle_matches(app);
}
