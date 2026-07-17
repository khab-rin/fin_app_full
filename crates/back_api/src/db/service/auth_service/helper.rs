pub(crate) fn mask_phone(phone: &str) -> String {
    let chars: Vec<char> = phone.chars().collect();
    if chars.len() <= 7 {
        return "****".to_string(); // Если номер подозрительно короткий
    }
    
    let start: String = chars[..5].iter().collect();
    let end: String = chars[chars.len() - 2..].iter().collect();
    let stars = "*".repeat(chars.len() - 7);
    
    format!("{}{}{}", start, stars, end)
}

pub(crate) fn mask_email(email: &str) -> String {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return "***@***".to_string(); // На случай невалидного email
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