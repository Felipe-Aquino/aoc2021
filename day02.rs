use std::fs;

fn _part1(content: String) {
    let mut pos: i32 = 0;
    let mut depth: i32 = 0;

    let lines = content.lines();

    for line in lines {
        let splited = line
            .split(' ')
            .collect::<Vec<&str>>();

        let value = i32::from_str_radix(splited[1], 10)
            .expect("not a number");

        match splited[0] {
            "forward" => pos += value,
            "up"      => depth -= value,
            "down"    => depth += value,
            _         => unreachable!(),
        }
    }

    println!("pos: {}, depth: {}, pos*depth: {}", pos, depth, pos * depth);
}

fn part2(content: String) {
    let mut pos: i64 = 0;
    let mut aim: i64 = 0;
    let mut depth: i64 = 0;

    let lines = content.lines();

    for line in lines {
        let splited = line
            .split(' ')
            .collect::<Vec<&str>>();

        let value = i64::from_str_radix(splited[1], 10)
            .expect("not a number");

        match splited[0] {
            "forward" => {
                pos += value;
                depth += aim * value;
            },
            "up"      => aim -= value,
            "down"    => aim += value,
            _         => unreachable!(),
        }
    }

    println!("pos: {}, depth: {}, aim: {}, pos*depth: {}", pos, depth, aim, pos * depth);
}

fn main() {
    let filename = "./inputs/day02-part1.txt";

    let content = fs::read_to_string(filename)
        .expect("Could not read file");

    part2(content);
}

