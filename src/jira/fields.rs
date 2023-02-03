use crate::jira::api;

pub fn display_all_fields(ticket: String) {
    let fields_response = api::get_call_v2(format!("issue/{ticket}/editmeta"));
    if fields_response.is_err() {
        eprintln!("Error occurred in API Call: {fields_response:?}");
        std::process::exit(1);
    }
    let fields = &fields_response.unwrap()["fields"];
    if fields.is_null() {
        eprintln!("Cannot fetch fields");
        std::process::exit(1);
    }
    println!("{:35}: Field Header", "Key");
    println!("{:-<65}", "-");
    for (field, value) in fields.entries() {
        println!("{:35}: {}", field, value["name"]);
    }

    println!("{:=<65}", "=");
    println!(
        "\n\nNote: If you want to use custom fields as alias, you can add an alias as
jira-terminal alias --add \"customfield_XXXXX\" new_field

After that, you can pass new_field as options for field in details.
        "
    )
}
