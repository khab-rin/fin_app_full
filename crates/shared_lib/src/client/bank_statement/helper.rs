use std::collections::HashMap;

pub fn make_statement_block_map(
    s: &str
) -> HashMap<&str, &str> {
    let mut res: HashMap<&str, &str> = HashMap::new();

    for line in s.lines() {
        if let Some((key, value)) = line.split_once('=') {
            res.insert(key.trim(), value.trim());
        }
    }

    res
}