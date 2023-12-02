fn get_input() -> String {
    include_str!("input.txt").to_string()
}

fn get_integers_in_string(line: &str) -> Vec<u32> {
    let nums: Vec<u32> = line.chars().filter_map(|x: char| x.to_digit(10)).collect();
    nums
}

fn recursive_num_replace(line: &str) -> &str {
    
}

fn main() {

    let input = get_input();
    let input_lines: std::str::Lines<'_> = input.lines();
    let mut total: u32 = 0;

    for line in input_lines {
        
        //  regex replace first and last numbers with integers

        // get the minimum offset and replace then recurse?

        let number_strings = ["_", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];



        let first_num: u32 = nums[0];
        let second_num: u32 = nums[nums.len()-1];

        let n = format!("{first_num}{second_num}").parse::<u32>().unwrap();

        total += n;
        println!("{nums:?} \n {total}");
    }

}