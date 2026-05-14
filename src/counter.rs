use std::collections::HashMap;

pub fn count(text: &str) -> HashMap<String, usize> {
    let mut scoreboard = HashMap::new();

    for word in text.split_whitespace() {
        let clean: String = word
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect();

        if clean.is_empty() { continue; }

        *scoreboard.entry(clean).or_insert(0) += 1;
    }

    scoreboard
}