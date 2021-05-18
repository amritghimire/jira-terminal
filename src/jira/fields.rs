use crate::jira::api;

pub fn display_all_fields(ticket: String) {
    let custom_fields_response = api::get_call_v2(format!("issue/{}/editmeta", ticket));
    if custom_fields_response.is_err() {
        println!("Error occured in API Call: {:?}", custom_fields_response);
        return;
    }
    let fields = &custom_fields_response.unwrap()["fields"];
    if fields.is_null() {
        println!("Cannot fetch fields");
        return;
    }
    println!("{:35}: {}", "Key", "Field Header");
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
