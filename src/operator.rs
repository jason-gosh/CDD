fn process_string(inp: &str) {}

fn normalize_string(inp: String) -> String {
    let is_only_alphanum: bool = inp.chars().any(|c| !c.is_alphanumeric());
    return inp.to_lowercase();
}
