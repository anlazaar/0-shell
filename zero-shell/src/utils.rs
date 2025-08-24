pub fn parse_command(input: &str) -> (String, Vec<String>) {
    let input = input.trim();
    let parts: Vec<&str> = input.split_whitespace().collect();

    let command = parts[0].to_string();
    let args = parts[1..]
        .iter()
        .map(|s| s.to_string())
        .collect();

    (command, args)
}


pub fn human_readable(size: u64) -> String {
    let units = ["B", "K", "M", "G"];
    let mut size = size as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < units.len()-1 {
        size /= 1024.0;
        unit += 1;
    }
    format!("{:.0}{}", size, units[unit])
}
