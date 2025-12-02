pub fn run() {
    let input = std::fs::read_to_string("src/inputs/day02.txt")
        .expect("Failed to read input");

    println!("Day 02, Part 1: {}", solve_part1(&input));
    println!("Day 02, Part 2: {}", solve_part2(&input));
}

type Range = (i64, i64);

fn parse_ranges(input: &str) -> Vec<Range> {
    input
        .split(',')
        .map(|r| {
            let (start, end) = r.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect()
}

fn solve_part1(input: &str) -> i64 {
    let mut result = 0;
    let ranges = parse_ranges(input);
    
    for range in ranges {
        let (start, end) = range;
        
        for i in start..(end + 1) {
            let number: String = i.to_string();
            if number.len() % 2 != 0 {
                continue;
            }
            let (first_half, second_half) = number.split_at(number.len() / 2);
            if (first_half == second_half) {
                result += i;
            }
        }
    }
    
    result
}

fn solve_part2(input: &str) -> i64 {
    let mut result = 0;
    let ranges = parse_ranges(input);
    
    for range in ranges {
        let (start, end) = range;
        for i in start..(end + 1) {
            let number: String = i.to_string();
            
            for len in 1..(number.len() / 2 + 1) {
                let replaced = number.replacen(number.split_at(len).0, "", number.len() / len);
                if replaced.is_empty() {
                    result += i;
                    break;
                }
            }
        }
    }
    
    result
}
