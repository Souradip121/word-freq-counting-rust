use std::collections::HashMap;

pub fn display(scoreboard: HashMap<String, usize>) {
    let mut list: Vec<(String, usize)> = scoreboard.into_iter().collect();

    list.sort_by(|a, b| b.1.cmp(&a.1));

    for (word, count) in list {
        println!("{:<15} → {}", word, count);
    }
}