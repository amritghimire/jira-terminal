use home;
use json;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;

mod cache;

fn get_config_file_name() -> String {
    let config_file_name: String = String::from(".jira_terminal_configuration.json");
    match home::home_dir() {
        Some(path) => return format!("{}/{}", path.display(), config_file_name),
        None => return config_file_name,
    }
}

fn check_config_exists() -> bool {
    fs::metadata(get_config_file_name()).is_ok()
}

pub fn get_authentication() -> (String, String) {
    (
        get_config("username".to_string()),
        get_config("password".to_string()),
    )
}

// fn fetch_account_id(credentials: (String, String)) -> String {}

fn create_config() {
    // Ask for the config file and create a new file.
    let mut namespace = String::new();
    let mut email = String::new();
    let mut token = String::new();
    let mut project_code = String::new();
    println!("Welcome to JIRA Terminal.");
    println!("Since this is your first run, we will ask you a few questions. ");
    println!("Please enter your namespace of JIRA. (<namespace>.atlassian.net): ");
    io::stdin()
        .read_line(&mut namespace)
        .expect("Failed to read input.");
    println!("Please enter your email address: ");
    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read input.");
    println!("Please create an API Token from https://id.atlassian.com/manage-profile/security/api-tokens ");
    println!("Once created, enter your API Token: ");
    io::stdin()
        .read_line(&mut token)
        .expect("Failed to read input.");
    println!("Please enter your default project code: ");
    io::stdin()
        .read_line(&mut project_code)
        .expect("Failed to read input.");
    let mut configuration = json::object! {
        namespace: namespace.trim(),
        email: email.trim(),
        token: token.trim(),
        project_code: project_code.trim(),
        account_id: "",
        alias: {},
        transitions: {}
    };
    let account_id = cache::get_username(&configuration);
    configuration["account_id"] = account_id.into();
    let config_json = json::stringify_pretty(configuration, 4);
    let mut file = fs::File::create(get_config_file_name()).expect("Unable to create config file.");
    file.write_all(config_json.as_bytes())
        .expect("Failed to write to file.");
    println!("Configuration file created.");
}

pub fn update_config() {
    // Update the existing config.
}

pub fn parse_config() -> json::JsonValue {
    let mut file = fs::File::open(get_config_file_name()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    json::parse(&contents).unwrap()
}

pub fn get_config(config: String) -> String {
    let config_value = &parse_config()[config];
    if config_value.is_string() {
        return String::from(config_value.as_str().unwrap());
    }
    String::from("")
}

pub fn get_alias() {}

pub fn set_alias() {}

pub fn set_transitions() {}

pub fn set_config() {}

pub fn ensure_config() {
    // This function will check if the provided config exists or not and will create a config if it
    // doesnot exists.
    let config_exists = check_config_exists();
    if !config_exists {
        create_config();
    }
}
