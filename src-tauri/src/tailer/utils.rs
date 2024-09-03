use std::env;

pub fn format_path(path: &str) -> String {
    let mut formatted_path = path.to_string();
    formatted_path = formatted_path.replace("%APPDATA%", env::var("APPDATA").unwrap().as_str());
    formatted_path = formatted_path.replace("%USERPROFILE%", env::var("USERPROFILE").unwrap().as_str());
    formatted_path
}

pub fn clean_pack_name(pack_name: &str) -> String {
    let mut pack_name = pack_name.to_string();
    pack_name = pack_name.replace('!', "").trim_end_matches(".zip").to_string();
    regex::Regex::new("§.")
        .unwrap()
        .replace_all(&pack_name, "")
        .to_string()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}
