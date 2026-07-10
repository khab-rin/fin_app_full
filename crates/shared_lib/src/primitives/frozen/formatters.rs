
pub fn default_fmt<T: std::fmt::Display>(data: &T) -> String {
    data.to_string()
}
pub fn uppercase_fmt<T: std::fmt::Display>(data: &T) -> String {
    data.to_string().to_uppercase()
}
pub fn snils_fmt<T: std::fmt::Display>(data: &T) -> String {
    
    let s = data.to_string();
    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    
    if digits.len() == 11 {
        format!("{}-{}-{} {}", &digits[0..3], &digits[3..6], &digits[6..9], &digits[9..11])
    } else {
        s
    }
}

pub fn passport_fmt<T: std::fmt::Display>(data: &T) -> String {
    let s = data.to_string();
    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    
    if digits.len() == 10 {
        format!("{} {} {}", &digits[0..2], &digits[2..4], &digits[4..10])
    } else {
        s
    }
}