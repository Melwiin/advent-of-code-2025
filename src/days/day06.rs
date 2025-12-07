pub fn run() {
    let input = std::fs::read_to_string("src/inputs/day06.txt").expect("Failed to read input");

    println!("Day 06, Part 1: {}", solve_part1(&input));
    println!("Day 06, Part 2: {}", solve_part2(&input));
}

fn parse_input(input: &str) -> Vec<(Vec<i64>, char)> {
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return Vec::new();
    }

    let columns: Vec<Vec<&str>> = lines
        .iter()
        .map(|line| line.split_whitespace().collect())
        .collect();

    if columns.is_empty() || columns[0].is_empty() {
        return Vec::new();
    }

    let num_columns = columns[0].len();
    let mut calculations: Vec<(Vec<i64>, char)> = Vec::new();

    for col_index in 0..num_columns {
        let mut numbers = Vec::new();
        let mut operator = ' ';

        for row in &columns {
            if col_index < row.len() {
                let item = row[col_index];

                if let Ok(num) = item.parse::<i64>() {
                    numbers.push(num);
                } else if item.len() == 1 {
                    operator = item.chars().next().unwrap();
                }
            }
        }

        calculations.push((numbers, operator));
    }

    calculations
}

fn solve_part1(input: &str) -> i64 {
    let mut result = 0;
    let calculations = parse_input(input);

    for calc in calculations {
        if calc.1 == '*' {
            result += calc.0.iter().product::<i64>();
        } else if calc.1 == '+' {
            result += calc.0.iter().sum::<i64>();
        }
    }

    result
}

fn solve_part2(input: &str) -> i64 {
    // Your solution here
    0
}
