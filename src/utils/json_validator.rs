pub fn is_valid_json(previous_lines: &str, current_line: &str) -> bool {
    let combined_json = format!("{}{}", previous_lines, current_line);
    let parsed_json = json::parse(&combined_json);
    parsed_json.is_ok()
}
