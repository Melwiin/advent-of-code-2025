pub fn run() {
    let input = std::fs::read_to_string("src/inputs/day03.txt").expect("Failed to read input");

    println!("Day 03, Part 1: {}", solve_part1(&input));
    println!("Day 03, Part 2: {}", solve_part2(&input));
}

fn turn_on_n_batteries(line: &str, n: usize) -> i64 {
    let digits: Vec<u32> = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c as u32 - '0' as u32)
        .collect();

    let mut found_indices: Vec<usize> = Vec::new();

    for i in 0..n {
        let lower_bound = found_indices.last().copied().unwrap_or(0);
        let upper_bound = digits.len() - n + 1 + i;

        let mut sub_digits = digits[lower_bound..upper_bound].to_vec();

        let mut highest = 0;
        let mut highest_index = 0;
        for (index, digit) in sub_digits.iter().enumerate() {
            if *digit > highest {
                highest = *digit;
                highest_index = index;
            }
        }

        found_indices.push(lower_bound + highest_index + 1);
    }

    let mut result_str = String::new();
    for index in found_indices {
        result_str.push_str(digits.get(index - 1).unwrap().to_string().as_str());
    }

    result_str.parse::<i64>().unwrap()
}

fn solve_part1(input: &str) -> i64 {
    input.lines().map(|line| turn_on_n_batteries(line, 2)).sum()
}

fn solve_part2(input: &str) -> i64 {
    input.lines().map(|line| turn_on_n_batteries(line, 12)).sum()
}
