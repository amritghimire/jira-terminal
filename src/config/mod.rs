use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;

mod cache;

/// Capitalize first letter of a word.
pub fn str_cap(s: String) -> String {
    format!("{}{}", (s[..1]).to_uppercase(), &s[1..])
}

/// Get the config file name regardless of platform.
/// This function will use platform independent library to fetch home directory and return file
/// name in home directory.
/// If home directory is not possible, this will return the config location from the executable
/// itself.
///
/// # Example:
/// ```
/// assert!(get_config_file_name(), "/home/user/.jira_terminal_configuration.json".to_string());
/// ```
fn get_config_file_name() -> String {
    let config_file_name: String = String::from(".jira_terminal_configuration.json");
    match home::home_dir() {
        Some(path) => format!("{}/{}", path.display(), config_file_name),
        None => config_file_name,
    }
}

/// Check if the config file already exists.
///
/// # Example
///
/// ```
/// assert!(check_config_exists());
/// ```
fn check_config_exists() -> bool {
    fs::metadata(get_config_file_name()).is_ok()
}

/// Create configuration file by asking user with the required information.
fn create_config() {
    // Ask for the config file and create a new file.
    let mut namespace = String::new();
    let mut email = String::new();
    println!("Welcome to JIRA Terminal.");
    println!("Since this is your first run, we will ask you a few questions. ");
    println!("Please enter your hostname of JIRA. (Example: example.atlassian.net): ");
    io::stdin()
        .read_line(&mut namespace)
        .expect("Failed to read input.");
    println!("Please enter your email address: ");
    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read input.");
    println!("Please create an API Token from https://id.atlassian.com/manage-profile/security/api-tokens. If your JIRA setup does not have api tokens plugin, you can enter the password too. ");
    println!("Once created, enter your API Token: (The characters will not be visible in screen.Press enter after you entered the password or token) ");
    let token = rpassword::read_password().unwrap();
    let user_password = format!("{}:{}", email.trim(), token.trim());
    let b64 = base64::encode(user_password);

    let mut configuration = json::object! {
        namespace: namespace.trim(),
        email: email.trim(),
        token: b64,
        account_id: "",
        alias: {},
        transitions: {}
    };
    let account_id = cache::get_username(&configuration);
    configuration["account_id"] = account_id.into();
    write_config(configuration);
}

/// Write the updated configuration to the file.
///
/// # Arguments
///
/// * configuration - Configuration file.
fn write_config(configuration: json::JsonValue) {
    let config_json = json::stringify_pretty(configuration, 4);
    let mut file = fs::File::create(get_config_file_name()).expect("Unable to create config file.");
    file.write_all(config_json.as_bytes())
        .expect("Failed to write to file.");
}

/// Update the single configuration.
///
/// # Arguments
///
/// * key - Config key to update.
/// * value - Value to update with.
///
/// # Example
/// ```
/// update_config("key".to_string(), "value".to_string());
/// assert_eq!("value".to_string(), get_config("key".to_string()));
/// ```
pub fn update_config(key: String, value: String) {
    let mut config_value = parse_config();
    config_value[key] = value.into();
    write_config(config_value);
}

/// Update the object structure configuration.
///
/// # Arguments
///
/// * key - Config key to update.
/// * value - Value to update with.
///
/// # Example
/// ```
/// update_config("key".to_string(), "value".to_string());
/// assert_eq!("value".to_string(), get_config("key".to_string()));
/// ```
pub fn update_config_object(key: String, value: json::JsonValue) {
    let mut config_value = parse_config();
    config_value[key] = value;
    write_config(config_value);
}

/// Parse the config file to json object.
pub fn parse_config() -> json::JsonValue {
    let mut file = fs::File::open(get_config_file_name()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    json::parse(&contents).unwrap()
}

/// Get the configuration for specified key. If the key does not exist, empty string is returned.
///
/// # Arguments:
/// * config - Configuration key.
///
/// # Example:
/// ```
/// let value = get_config("email".to_string());
/// ```
///
pub fn get_config(config: String) -> String {
    let config_value = &parse_config()[config];
    if config_value.is_string() {
        return String::from(config_value.as_str().unwrap());
    }
    String::from("")
}

/// Get the alias stored in configuration.
///
/// # Arguments
/// * alias - Alias value.
///
/// # Example
/// ```
/// assert!(get_alias("exists".to_string()).is_some());
/// assert!(get_alias("not_exists".to_string()).is_none());
/// ```
pub fn get_alias(alias: String) -> Option<String> {
    let config_value = &parse_config()["alias"][alias.to_lowercase()];
    if config_value.is_null() {
        None
    } else {
        Some(config_value.as_str().unwrap().to_string())
    }
}

/// Replace the value with alias value if it is alias, otherwise it will return the string as it
/// is.
///
/// # Arguments
/// * alias - Alias to replace or return as it is.
///
/// # Example
/// ```
/// assert_eq!(get_alias_or("ip".to_string()), "In Progress".to_string());
/// assert_eq!(get_alias_or("IP".to_string()), "In Progress".to_string());
/// assert_eq!(get_alias_or("In Progress".to_string(), "In Progress".to_string()));
/// ```
///
pub fn get_alias_or(alias: String) -> String {
    let alias_value = get_alias(alias.clone());
    match alias_value {
        Some(x) => x,
        None => alias,
    }
}

/// Set the alias to provided value and update the configuration.
///
/// # Arguments
///
/// * alias - Case insensitive alias to store or update.
/// * value - Value to associate with alias.
///
/// # Example
/// ```
/// set_alias("ip".to_string(), "In Progress".to_string());
/// ```
pub fn set_alias(alias: String, value: String) {
    let mut config_value = parse_config();
    config_value["alias"][alias.to_lowercase()] = value.into();
    write_config(config_value);
}

/// Remove the alias from configuration.
///
/// # Arguments
///
/// * alias - Name of alias
///
/// # Example
/// ```
/// remove_alias("name".to_string());
/// ```
pub fn remove_alias(alias: String) {
    let mut config_value = parse_config();
    let mut alias_object = config_value["alias"].clone();
    println!(
        "Removing alias ({}) with value: {}",
        alias,
        alias_object[alias.clone()]
    );
    alias_object.remove(alias.to_lowercase().as_str());
    config_value["alias"] = alias_object;
    write_config(config_value);
}

/// Completely replace the transition object with new value.
/// This function will be used to update or store transition codes for a project code.
///
/// # Arguments
///
/// * project_code - Project Code for JIRA. For a ticket ABC-123, project code is ABC.
/// * transitions - Json object for transitions.
///
/// # Example
/// ```
/// use json;
///
/// let transition = json::object! {
///     "backlog": 21,
///     "in progress": 31
/// }
/// set_transitions("ABC".to_string(), transition);
/// ```
pub fn set_transitions(project_code: String, transitions: json::JsonValue) {
    let mut config_value = parse_config();
    config_value["transitions"][project_code] = transitions;
    write_config(config_value);
}

/// Get the transitions for provided project code.
///
/// # Arguments
///
/// * project_code - Project Code for JIRA. For a ticket ABC-123, project code is ABC.
/// # Example
/// ```
/// let transitions = get_transitions("ABC".to_string());
/// ```
pub fn get_transitions(project_code: String) -> json::JsonValue {
    let config_value = &parse_config()["transitions"][project_code];
    config_value.clone()
}

/// Check if the transition exists for provided transition code in config file already.
///
/// # Arguments
///
/// * project_code - Project Code for JIRA. For a ticket ABC-123, project code is ABC.
/// * transition_name - Name of transition.
///
/// # Example
/// ```
/// assert!(transition_exists("ABC".to_string(), "in progress".to_string()));
/// ```

pub fn transition_exists(project_code: String, transition_name: String) -> bool {
    let config_value = &parse_config()["transitions"][project_code][transition_name];
    !config_value.is_null()
}

/// Ensure the config exists.
/// It will first check the config file exists.
/// If it does not, it will ask the user to create one.
pub fn ensure_config() {
    let config_exists = check_config_exists();
    if !config_exists {
        create_config();
    }
}

/// List all the provided alias.
pub fn list_all_alias() {
    let config_value = parse_config();
    println!("Listing alias saved for you: ");
    for (alias, value) in config_value["alias"].entries() {
        println!("* {:20} => {:?}", alias, value.as_str().unwrap_or(""));
    }
}
