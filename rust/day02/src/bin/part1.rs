use regex::Regex;
use std::collections::HashMap;
use std::cmp;

fn get_input() -> String {
    include_str!("input.txt").to_string()
}

fn get_game_id(hay: &str) -> u8 {
    let re = Regex::new(r"(\d+)").unwrap();
    let result = re.find(hay).unwrap();
    let game_id = result.as_str().parse::<u8>().unwrap();
    return game_id;
}

fn main() {
    let input = get_input();
    let input_lines = input.lines();

    let mut id_counter: i32 = 0;

    for line in input_lines {
        let separated: Vec<String> = line.split(": ").map(|x| x.to_string()).collect();
        let game_num = get_game_id(&separated[0]);
        let game_str = &separated[1];

        // let games = game_str.split("; ");
        println!("{game_num}");

        let mut max_colors: HashMap<String, u8> = HashMap::from([
            ("red".to_string(), 0),
            ("green".to_string(), 0),
            ("blue".to_string(), 0)
        ]);

        let valid_colors: Vec<String> = max_colors.keys().map(|k|k.clone()).collect();

        let max_values: HashMap<String, u8> = HashMap::from([
            ("red".to_string(), 12),
            ("green".to_string(), 13),
            ("blue".to_string(), 14)
        ]);

        for color in &valid_colors {
            let search_string = r"(\d+ ".to_owned() + &color + ")";
            let re = Regex::new(&search_string).unwrap();
            let hay = game_str;
            let matches: Vec<String> = re
                .find_iter(hay)
                .filter_map(|draws| draws.as_str().parse().ok()
                )
                .collect();
            for m in matches {
                let re = Regex::new(r"(\d+)").unwrap();
                let count = re.find(&m).unwrap().as_str().parse::<u8>().unwrap();
                max_colors.insert(color.clone(), cmp::max(count, max_colors[color]));
            }
        }
        
        if valid_colors.iter().all(|c| max_colors[c] <= max_values[c]) {
            id_counter += game_num as i32
        }

        println!("{id_counter}");

    }
}
