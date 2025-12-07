pub fn run() {
    let input = std::fs::read_to_string("src/inputs/day04.txt").expect("Failed to read input");

    println!("Day 04, Part 1: {}", solve_part1(&input));
    println!("Day 04, Part 2: {}", solve_part2(&input));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn solve_part1(input: &str) -> i64 {
    let mut result = 0;
    let mut grid: Vec<Vec<char>> = parse_input(input);

    let dirs = vec![
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == '@' {
                let mut paper_rolls = 0;

                for &(dr, dc) in &dirs {
                    let new_r = r as isize + dr;
                    let new_c = c as isize + dc;

                    if new_r >= 0
                        && new_r < grid.len() as isize
                        && new_c >= 0
                        && new_c < grid[0].len() as isize
                    {
                        if grid[new_r as usize][new_c as usize] == '@' {
                            paper_rolls += 1;
                        }
                    }
                }

                if paper_rolls < 4 {
                    result += 1;
                }
            }
        }
    }

    result
}

fn solve_part2(input: &str) -> i64 {
    let mut result = 0;
    let mut grid: Vec<Vec<char>> = parse_input(input);

    let dirs = vec![
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let mut removed = 0;
    while {
        removed = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == '@' {
                    let mut paper_rolls = 0;

                    for &(dr, dc) in &dirs {
                        let new_r = r as isize + dr;
                        let new_c = c as isize + dc;

                        if new_r >= 0
                            && new_r < grid.len() as isize
                            && new_c >= 0
                            && new_c < grid[0].len() as isize
                        {
                            let (nr, nc) = (new_r as usize, new_c as usize);

                            if grid[nr][nc] == '@' {
                                paper_rolls += 1;
                            }
                        }
                    }

                    if paper_rolls < 4 {
                        result += 1;
                        grid[r][c] = '.';
                        removed += 1;
                    }
                }
            }
        }

        removed != 0
    } {}

    result
}
