use std::fs;

fn _part1(content: String) {
    let lines = content.lines();
    let measures = lines.map(|s| {
        let n = i32::from_str_radix(s, 10)
            .expect("not a number");

        n
    })
    .collect::<Vec<i32>>();

    let mut count = 0;

    for i in 0..measures.len() - 1 {
        if measures[i + 1] > measures[i] { 
            count += 1;
            println!("(increased)");
        } else if measures[i + 1] < measures[i] {
            println!("(decreased)");
        } else {
            println!("(unchanged)");
        }
    }

    println!("number of increased: {}", count);
}

fn part2(content: String) {
    let lines = content.lines();
    let measures = lines.map(|s| {
        let n = i32::from_str_radix(s, 10)
            .expect("not a number");

        n
    })
    .collect::<Vec<i32>>();

    let mut windows: Vec<i32> = Vec::new(); //Vec::with_capacity(measures.len() - 2);

    for i in 0..measures.len() - 2 {
        windows.push(measures[i] + measures[i + 1] + measures[i + 2]);
    }

    let mut count = 0;

    for i in 0..windows.len() - 1 {
        if windows[i + 1] > windows[i] { 
            count += 1;
        }
        // if windows[i + 1] > windows[i] { 
        //     count += 1;
        //     println!("(increased)");
        // } else if windows[i + 1] < windows[i] {
        //     println!("(decreased)");
        // } else {
        //     println!("(unchanged)");
        // }
    }

    println!("number of increased: {}", count);
}

fn main() {
    let filename = "./inputs/day01-part1.txt";

    let content = fs::read_to_string(filename)
        .expect("Could not read file");

    part2(content);
}
