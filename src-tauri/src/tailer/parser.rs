use crate::tailer::utils::clean_pack_name;

pub fn parse_server_address(log_line: &str) -> Option<&str> {
    let start_ip_idx = log_line.find("Connecting to ")? + "Connecting to ".len();
    let end_ip_idx = log_line[start_ip_idx..].find(',')?;
    Some(&log_line[start_ip_idx..start_ip_idx + end_ip_idx])
}

pub fn parse_packs(log_line: &str) -> Vec<String> {
    let pack = &log_line[60..];

    if pack == "Default" {
        return vec![pack.to_string()];
    }

    let packs = pack.split(',');
    let mut parsed_packs = Vec::new();

    for raw_pack_name in packs {
        let pack_name = clean_pack_name(raw_pack_name);

        if pack_name != "textures" && pack_name != "Default" {
            parsed_packs.push(pack_name);
        }
    }

    parsed_packs
}
