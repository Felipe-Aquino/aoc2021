use std::fs;

fn _part1(content: String) {
    let mut positions: Vec<i32> = content
        .trim_end()
        .split(",")
        .map(|v| {
            i32::from_str_radix(v, 10).unwrap()
        })
        .collect();

    positions.sort();

    let dest_position: i32 = 
        if positions.len() % 2 == 0 {
            let i = positions.len() / 2;

            (positions[i - 1] + positions[i]) / 2
        } else {
            positions[positions.len() / 2]
        };

    let total_fuel: i32 = positions
        .iter()
        .fold(0, move |acc, p| {
            acc + (p - dest_position).abs()
        });

    println!("{}: {} - {:?}", dest_position, total_fuel, positions);
}

fn triangular(n: i32) -> i32 {
    n * (n + 1) / 2
}

/*
 * sum of 0.5 * (|p - ai| + 1) * |p - ai|
 * minimizing -> sum of ai = n * (p +/- 0.5)
 *            -> p = avg +/- 0.5
 */
fn part2(content: String) {
    let mut positions: Vec<i32> = content
        .trim_end()
        .split(",")
        .map(|v| {
            i32::from_str_radix(v, 10).unwrap()
        })
        .collect();

    positions.sort();

    let sum: i32 = positions
        .iter()
        .sum();

    let avg = (sum as f32) / (positions.len() as f32) + 0.5;

    let dest_position = avg.floor() as i32;

    let total_fuel: i32 = positions
        .iter()
        .fold(0, move |acc, p| {
            acc + triangular((p - dest_position).abs())
        });

    println!("{}: {}", dest_position, total_fuel);

    let dest_position = avg.floor() as i32 - 1;

    let total_fuel: i32 = positions
        .iter()
        .fold(0, move |acc, p| {
            acc + triangular((p - dest_position).abs())
        });

    println!("{}: {}", dest_position, total_fuel);
}

fn main() {
    let filename = "./inputs/day07.txt";

    let content = fs::read_to_string(filename)
        .expect("Could not read file");

    part2(content);
}
