pub fn run() {
    let input = std::fs::read_to_string("src/inputs/day01.txt")
        .expect("Failed to read input");

    println!("Day 01, Part 1: {}", solve_part1(&input));
    println!("Day 01, Part 2: {}", solve_part2(&input));
}

fn parse_value_from_line(line: &str) -> i64 {
    let (direction, value) = line.split_at(1);
    let value = value.parse::<i64>().expect("Could not parse dial value.");

    match direction {
        "L" => -value,
        "R" =>  value,
        x    => panic!("Expeccted either L or R. Got: '{}'", x),
    }
}

fn solve_part1(input: &str) -> i64 {
    let mut result = 0;
    let mut dial = 50;

    for line in input.lines() {
        let value = parse_value_from_line(line);
        dial = (dial + value) % 100;

        if dial == 0 {
            result += 1;
        }
    }

    result
}

fn solve_part2(input: &str) -> i64 {
    let mut result = 0;
    let mut dial = 50;

    for line in input.lines() {
        let value = parse_value_from_line(line);
        let step: i8 = if value < 0 { -1 } else { 1 };

        for _ in 0..value.abs() {
            dial = (dial + step) % 100;
            if dial == 0 {
                result += 1;
            }
        }
    }

    result
}
