use std::fs;

use std::sync::mpsc;
use std::thread;

fn _part1(content: String) {
    let mut lanternfish_ages: Vec<u8> =
        content
        .split(",")
        .map(|v| {
            u8::from_str_radix(v.trim_end(), 10)
                .expect(format!("{} is not a valid number", v).as_str())
        })
        .collect();

    let mut days_count = 0;

    // println!("{}: {:?}", days_count, lanternfish_ages);

    while days_count != 80 {
        let mut new_lanternfish_count = 0;

        for i in 0..lanternfish_ages.len() {
            let mut age = lanternfish_ages[i];
            
            if age == 0 {
                age = 6;
                new_lanternfish_count += 1;
            } else {
                age -= 1;
            }

            lanternfish_ages[i] = age;
        }

        if new_lanternfish_count > 0 {
            lanternfish_ages.append(&mut vec![8u8; new_lanternfish_count]);
        }
        
        days_count += 1;
        // println!("{}: {:?}", days_count, lanternfish_ages);
    }

    println!("count: {}", lanternfish_ages.len());
}

fn part2(content: String) {
    let lanternfish_ages: Vec<u8> =
        content
        .split(",")
        .map(|v| {
            u8::from_str_radix(v.trim_end(), 10)
                .expect(format!("{} is not a valid number", v).as_str())
        })
        .collect();

    // println!("{}: {:?}", days_count, lanternfish_ages);

    let (tx, rx) = mpsc::channel();

    for j in 1..6 {
        let txc = tx.clone();

        thread::spawn(move || {
            let mut days_count = 0;
            
            let mut generations_mult: Vec<usize> = vec![1];
            let mut generations = vec![j as u8];

            while days_count != 256 {
                let mut new_lanternfish_count = 0;

                for i in 0..generations.len() {
                    let mut age = generations[i];
                    
                    if age == 0 {
                        age = 6;
                        new_lanternfish_count += generations_mult[i];
                    } else {
                        age -= 1;
                    }

                    generations[i] = age;
                }

                if new_lanternfish_count > 0 {
                    generations_mult.push(new_lanternfish_count);
                    generations.push(8);
                    // println!("{}  - {}: {}", days_count, j, new_lanternfish_count);
                }

                days_count += 1;
            }

            let sum = generations_mult.iter().sum::<usize>();
            txc.send((j - 1, sum)).unwrap();
        });
    }

    let mut generations_count = [0usize; 5];
    let mut finished_count = 0;

    for received in rx {
        // println!("Got: {:?}", received);

        let (pos, value) = received;
        generations_count[pos] = value;

        finished_count += 1;

        if finished_count == 5 {
            break;
        }
    }

    let mut total_lanternfish_count = 0;

    for &v in lanternfish_ages.iter() {
        total_lanternfish_count += generations_count[(v - 1) as usize];
    }

    println!("count: {}", total_lanternfish_count);
}

fn main() {
    let filename = "./inputs/day06.txt";

    let content = fs::read_to_string(filename)
        .expect("Could not read file");

    part2(content);
}
