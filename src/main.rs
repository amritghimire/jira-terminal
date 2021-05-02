mod config;

fn main() {
    println!("Starting JIRA Terminal.");
    config::ensure_config();
    println!("Existing key {}", config::get_config("email".to_string()));
    println!(
        "Non Existing key {}",
        config::get_config("alias".to_string())
    );
}
