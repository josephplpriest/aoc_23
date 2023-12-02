use regex::Regex;
use std::cmp;
use std::collections::HashMap;

// Generic function to read in text
fn get_input() -> String {
    include_str!("input.txt").to_string()
}

// get the game id via regex
fn get_game_id(hay: &str) -> u8 {
    let re = Regex::new(r"(\d+)").unwrap();
    let result = re.find(hay).unwrap();
    let game_id = result.as_str().parse::<u8>().unwrap();
    return game_id;
}

fn main() {
    let input: String = get_input();
    let input_lines: std::str::Lines<'_> = input.lines();

    let mut id_counter: i32 = 0;

    for line in input_lines {
        // Rust has nice functional programming support and lambdas (like the code on the next line after 'map')

        let separated: Vec<String> = line.split(": ").map(|x| x.to_string()).collect();
        let game_num: u8 = get_game_id(&separated[0]);

        // get the rest of the line with the game info
        let game_str: &String = &separated[1];

        // Rust's version of a python dictionary, not too different though you have to specify var types
        let mut max_colors: HashMap<String, u8> = HashMap::from([
            ("red".to_string(), 0),
            ("green".to_string(), 0),
            ("blue".to_string(), 0),
        ]);

        // More functional code and rust lambda
        let valid_colors: Vec<String> = max_colors.keys().map(|k| k.clone()).collect();

        let max_values: HashMap<String, u8> = HashMap::from([
            ("red".to_string(), 12),
            ("green".to_string(), 13),
            ("blue".to_string(), 14),
        ]);

        // do a regex for each color over the entire line
        for color in &valid_colors {
            let search_string = r"(\d+ ".to_owned() + &color + ")";
            let re = Regex::new(&search_string).unwrap();
            let hay = game_str;

            // get an iterable of each match ie. ["7 red", "5 red", "10 red"]
            let matches: Vec<String> = re
                .find_iter(hay)
                .filter_map(|draws| draws.as_str().parse().ok())
                .collect();

            // iterate through matches and add the largest value to the hashmap
            for m in matches {
                let re = Regex::new(r"(\d+)").unwrap();
                let count = re.find(&m).unwrap().as_str().parse::<u8>().unwrap();
                max_colors.insert(color.clone(), cmp::max(count, max_colors[color]));
            }
        }

        // Rust version of python's list.all() to check if all color counts are less than the max allowed
        if valid_colors.iter().all(|c| max_colors[c] <= max_values[c]) {
            id_counter += game_num as i32
        }

        println!("{id_counter}");
    }
}
