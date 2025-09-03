pub fn parse_command(input: &str) -> (String, Vec<String>) {
    let input = input.trim();
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return (String::new(), Vec::new());
    }

    let command = parts[0].to_string();
    let args = parts[1..]
        .iter()
        .map(|s| s.to_string())
        .collect();

    (command, args)
}
