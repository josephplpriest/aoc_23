use regex::Regex;
use regex::Match;
use std::collections::HashSet;

fn get_input() -> String {
    include_str!("input.txt").to_string()
}

fn find_symbols(input: &String) -> HashSet<i32> {

    let re = Regex::new(r"[^\d.\s]").unwrap();

    let matches = re.find_iter(input);

    let result: HashSet<i32> = HashSet::from_iter(
        matches.map(
            |x|x.start()).map(|x|i32::try_from(x).unwrap()));
    result
}

fn find_numbers(input: &String) -> Vec<Match> {

    let re = Regex::new(r"(\d+)").unwrap();

    let matches: Vec<Match> = re.find_iter(input).collect();

    matches

}

fn is_valid_num(start: i32, end: i32, symbol_locations: &HashSet<i32>, offset: i32) -> bool {

    let before_num: i32 = start - 1;
    let after_num: i32 = end;
    let previous_line_start: i32 = before_num - (offset + 1);
    let previous_line_end: i32 = after_num - (offset + 1) + 1;
    let next_line_start: i32 = before_num + (offset + 1);
    let next_line_end: i32 = after_num + (offset + 1) + 1;

    let mut previous_line_nums: Vec<i32> = Vec::from_iter(previous_line_start..previous_line_end);
    let mut next_line_nums: Vec<i32> = Vec::from_iter(next_line_start..next_line_end);

    // println!("previous_line: {:?}", &previous_line_nums);
    // println!("next_line: {:?}", &next_line_nums);

    previous_line_nums.append(&mut next_line_nums);
    previous_line_nums.append(&mut vec![before_num, after_num]);


    let search_locs: HashSet<i32> = HashSet::from_iter(previous_line_nums);



    let intersection: HashSet<_> = symbol_locations.intersection(&search_locs).collect();

    return intersection.len() > 0;
}

fn main() {
    let input = get_input();
    
    let symbol_locations: HashSet<i32> = find_symbols(&input);

    let number_matches: Vec<Match> = find_numbers(&input);

    let lines: Vec<&str> = input.lines().collect();

    let offset: i32 = lines[0].len().try_into().unwrap();

    println!("{}", input);

    let mut total: i32 = 0;

    for mat in number_matches {
        let start: i32 = mat.start().try_into().unwrap();
        let end: i32 = mat.end().try_into().unwrap();
        let num: i32 = mat.as_str().parse::<i32>().unwrap();

        let is_valid: bool = is_valid_num(start, end, &symbol_locations, offset);

        println!("match: {mat:?}, is_valid: {is_valid}");
        if is_valid {
            total += num
        }
    }

    println!("{}", total);

}