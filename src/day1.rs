use std::collections::HashSet;

pub fn star2(lines: &Vec<String>) -> String {
    let freq_changes = lines.into_iter().map(|l| l.parse::<i32>().unwrap()).cycle();
    let mut set = HashSet::new();
    let mut current = 0;
    for change in freq_changes {
        current += change;
        if set.contains(&current) {
            return current.to_string();
        }
        set.insert(current);
    }
    return "".to_string();
}