use std::cmp;

pub fn run() {
    let input = std::fs::read_to_string("src/inputs/day05.txt").expect("Failed to read input");

    println!("Day 05, Part 1: {}", solve_part1(&input));
    println!("Day 05, Part 2: {}", solve_part2(&input));
}

type Range = (i64, i64);

fn parse_input(input: &str) -> (Vec<Range>, Vec<i64>) {
    let mut ranges: Vec<Range> = Vec::new();
    let mut ids: Vec<i64> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains("-") {
            let (start, end) = line.split_once('-').unwrap();
            ranges.push((start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap()));
            continue;
        }
        ids.push(line.parse().unwrap());
    }

    (ranges, ids)
}

fn is_in_range(num: i64, range: Range) -> bool {
    num >= range.0 && num <= range.1
}

fn combine_ranges(range1: Range, range2: Range) -> Range {
    (cmp::min(range1.0, range2.0), cmp::max(range1.1, range2.1))
}

fn do_overlap(range1: Range, range2: Range) -> bool {
    let (start1, end1) = range1;
    let (start2, end2) = range2;

    start1 <= end2 && start2 <= end1
}

fn solve_part1(input: &str) -> i64 {
    let mut result = 0;
    let (ranges, ids) = parse_input(input);

    for id in ids {
        for range in &ranges {
            if is_in_range(id, *range) {
                result += 1;
                break;
            }
        }
    }

    result
}

fn solve_part2(input: &str) -> i64 {
    let (mut ranges, _) = parse_input(input);

    while {
        let mut merged = false;
        let mut i = 0;

        while i < ranges.len() {
            let mut j = i + 1;

            while j < ranges.len() {
                if do_overlap(ranges[i], ranges[j]) {
                    ranges[i] = combine_ranges(ranges[i], ranges[j]);
                    ranges.remove(j);
                    merged = true;
                } else {
                    j += 1;
                }
            }
            i += 1;
        }

        merged
    } {}

    ranges.iter().map(|range| range.1 - range.0 + 1).sum()
}
