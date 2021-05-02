mod config;

fn test_get_alias(alias: String) {
    let result = config::get_alias(alias);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot find by alias"),
    }
}

fn main() {
    println!("Starting JIRA Terminal.");
    config::ensure_config();
    println!("Existing key {}", config::get_config("email".to_string()));
    println!(
        "Non Existing key {}",
        config::get_config("alias".to_string())
    );
    config::update_config(
        "email".to_string(),
        "new_email@amritghimire.com".to_string(),
    );
    config::set_alias("ip".to_string(), "In Progress".to_string());
    test_get_alias("ip".to_string());
    test_get_alias("iasasad".to_string());
}
