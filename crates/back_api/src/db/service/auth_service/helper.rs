pub(crate) fn mask_string(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let total_len = chars.len();

    if total_len < 4 {
        return "*".repeat(total_len.max(4));
    }

    let visible_count = total_len / 4;

    let show_start = visible_count.max(1);
    let show_end = visible_count.max(1);

    let start: String = chars[..show_start].iter().collect();
    let end: String = chars[total_len - show_end..].iter().collect();
    let stars_count = total_len - show_start - show_end;

    format!("{}{}{}", start, "*".repeat(stars_count), end)
}

pub(crate) fn mask_email(email: &str) -> String {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return "***@***".to_string();
    }
    
    let name = parts[0];
    let domain = parts[1];
    let chars: Vec<char> = name.chars().collect();
    
    if chars.len() <= 2 {
        return format!("*@{}", domain);
    }
    
    let start: String = chars[..2].iter().collect();
    format!("{}***@{}", start, domain)
}