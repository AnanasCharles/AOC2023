use std::collections::HashMap;
use std::fs;

fn is_possible(game_info: &Vec<HashMap<&str, usize>>, target_counts: &HashMap<&str, usize>) -> bool {
    for subset in game_info {
        for (color, count) in subset {
            if let Some(target_count) = target_counts.get(color) {
                if count > target_count {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    true
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file");

    let target_counts = [("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect::<HashMap<&str, usize>>();

    let mut possible_games = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        if parts.len() > 1 {
            let game_info: Vec<HashMap<&str, usize>> = parts[1]
                .trim()
                .split(';')
                .map(|subset| {
                    subset
                        .trim()
                        .split(',')
                        .map(|s| {
                            let kv: Vec<&str> = s.trim().split_whitespace().collect();
                            (kv[1], kv[0].parse().unwrap_or(0))
                        })
                        .collect()
                })
                .collect();

            if is_possible(&game_info, &target_counts) {
                if let Some(game_id) = parts[0].trim().split_whitespace().nth(1) {
                    possible_games.push(game_id.parse::<usize>().unwrap_or(0));
                }
            }
        }
    }

    let sum: usize = possible_games.iter().sum();
    println!("Sum of IDs of possible games: {}", sum);
}
