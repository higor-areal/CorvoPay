#[allow(dead_code)]
pub fn seconds_to_iso8601(total_secs: u32) -> String {
    let days = total_secs / 86_400;
    let hours = (total_secs % 86_400) / 3_600;
    let minutes = (total_secs % 3_600) / 60;
    let seconds = total_secs % 60;

    let mut iso = String::from("P");

    if days > 0 {
        iso.push_str(&format!("{}D", days));
    }
    if hours > 0 || minutes > 0 || seconds > 0 {
        iso.push('T');
        if hours > 0   { iso.push_str(&format!("{}H", hours)); }
        if minutes > 0 { iso.push_str(&format!("{}M", minutes)); }
        if seconds > 0 { iso.push_str(&format!("{}S", seconds)); }
    }

    iso
}