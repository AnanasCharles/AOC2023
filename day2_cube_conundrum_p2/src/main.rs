use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut sum_of_products = 0;

    if let Ok(file) = File::open("input.txt") {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let split_line: Vec<&str> = line.split(':').collect();
            let index_part = split_line[0].trim();
            let index: Vec<&str> = index_part.split_whitespace().collect();
            let data_part = split_line[1].trim();
            let data_separated: Vec<&str> = data_part.split(';').collect();

            let mut sets: Vec<HashMap<&str, usize>> = Vec::new();

            for set in data_separated {
                let values_raw: Vec<&str> = set.split(',').collect();
                let values_stripped: Vec<&str> = values_raw.iter().map(|value| value.trim()).collect();
                let mut color_number = HashMap::new();

                for item in &values_stripped {
                    let parts: Vec<&str> = item.split_whitespace().collect();
                    let number: usize = parts[0].parse().unwrap_or(0);
                    let color = parts[1];
                    color_number.insert(color, number);
                }

                sets.push(color_number);
            }

            let mut highest_values: HashMap<&str, usize> = HashMap::new();

            for dict in &sets {
                for (color, value) in dict.iter() {
                    let entry = highest_values.entry(color).or_insert(0);
                    *entry = (*entry).max(*value);
                }
            }

            let product: usize = highest_values.values().product();
            sum_of_products += product;
        }
    }

    println!("Sum of the power of minimum sets: {}", sum_of_products);

    Ok(())
}
