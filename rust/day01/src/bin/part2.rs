use std::iter::zip;
use regex::Regex;

fn get_input() -> String {
    include_str!("input.txt").to_string()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    start: u32,
    value: i32
}

fn main() {

    let input = get_input();
    let input_lines: std::str::Lines<'_> = input.lines();
    let mut total: i32 = 0;

    for line in input_lines {
        println!("{line}");        

        // get the minimum offset and replace then recurse?

        let num_words: Vec<String> = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter().map(|x: &&str|x.to_string()).collect();

        let num_ints: Vec<i32> = Vec::from_iter(1..10);

        let mut match_positions: Vec<Position> = vec![];

        for (w, i) in zip(num_words, num_ints) {
            
            // search for the integers and push each into the match vec
            let re: Regex = Regex::new(&format!("({w})").to_owned()).unwrap();
            
            let matches: Vec<Position> = re.find_iter(line).filter_map(|m| Some(Position {
                start: m.start() as u32,
                value: i
                })).collect();

            let re2: Regex = Regex::new(&format!("({i})").to_owned()).unwrap();
            
            let matches2: Vec<Position>  = re2.find_iter(line).filter_map(|m| Some(Position {
                start: m.start() as u32,
                value: i
                })).collect();
            
            // search for the word (int as string) and push the INTEGER into the vec
            match_positions.extend(matches);
            match_positions.extend(matches2);

        }

        match_positions.sort();
        println!("{match_positions:?}");

        let first_val = match_positions[0].value;
        let second_val = match_positions.pop().unwrap().value;

        let final_num = format!("{first_val}{second_val}").parse::<i32>().unwrap();
        total += final_num;

        println!("total: {total} -- final_num: {final_num}");
    }


}