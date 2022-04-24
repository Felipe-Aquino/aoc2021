use std::fs;

#[allow(dead_code)]
fn lookup_for_checker_points(c: char) -> i32 {
    match c {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
    _   => unreachable!(),
    }
}

fn lookup_for_autocomplete_points(c: char) -> u64 {
    match c {
    ')' => 1,
    ']' => 2,
    '}' => 3,
    '>' => 4,
    _   => unreachable!(),
    }
}

fn get_closing(c: char) -> char {
    match c {
    '(' => ')',
    '[' => ']',
    '{' => '}',
    '<' => '>',
    _   => unreachable!(),
    }
}

#[allow(dead_code)]
fn part1(content: String) {
    let mut total_points = 0i32;

    for (i, line) in content.lines().enumerate() {
        let mut stack: Vec<char> = vec![];

        'inner: for (j, c) in line.chars().enumerate() {
            match c {
            '(' | '{' | '[' | '<' => {
                stack.push(c);
            },
            ')' | '}' | ']' | '>' => {
                if let Some(&l) = stack.last() {
                    if get_closing(l) != c {
                        println!(
                            "{}:{} - Expected '{}', but found '{}' instead",
                            i, j, get_closing(l), c
                        );

                        total_points += lookup_for_checker_points(c);

                        break 'inner;
                    } else {
                        stack.pop();
                    }
                } else {
                    println!(
                        "{}:{} - Expected nothing, but found '{}' instead",
                        i, j, c
                    );
                    break 'inner;
                }
            },
            _ => unreachable!(),
            }
        }
    }

    println!("points: {}", total_points);
}

fn part2(content: String) {
    let mut totals: Vec<u64> = Vec::new();

    for line in content.lines() {
        let mut stack: Vec<char> = vec![];
        let mut is_corrupted = false;

        'inner: for c in line.chars() {
            match c {
            '(' | '{' | '[' | '<' => {
                stack.push(c);
            },
            ')' | '}' | ']' | '>' => {
                if let Some(&l) = stack.last() {
                    if get_closing(l) != c {
                        is_corrupted = true;
                        break 'inner;
                    } else {
                        stack.pop();
                    }
                } else {
                    is_corrupted = true;
                    break 'inner;
                }
            },
            _ => unreachable!(),
            }
        }

        if !is_corrupted && stack.len() > 0 {
            let mut total_points = 0u64;

            for i in (0..stack.len()).rev() {
                let c = get_closing(stack[i]);

                total_points =
                    total_points * 5 + lookup_for_autocomplete_points(c);
            }

            totals.push(total_points);
        }
    }

    totals.sort();
    let total_points = totals[totals.len() / 2];

    println!("points: {}", total_points);
}

fn main() {
    let filename = "./inputs/day10.txt";

    let content = fs::read_to_string(filename)
        .expect("Could not open file");

    part2(content);
}

