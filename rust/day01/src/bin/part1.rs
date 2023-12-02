fn get_input() -> String {
    include_str!("input.txt").to_string()
}

fn main() {

    let input = get_input();
    let input_lines: std::str::Lines<'_> = input.lines();
    let mut total: u32 = 0;

    for line in input_lines {
        
        let nums: Vec<u32> = line.chars().filter_map(|x: char| x.to_digit(10)).collect();
        
        let first_num: u32 = nums[0];
        let second_num: u32 = nums[nums.len()-1];

        let n = format!("{first_num}{second_num}").parse::<u32>().unwrap();


        total += n;
        println!("{nums:?} \n {total}");
    }

}