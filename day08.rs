use std::fs;
use std::collections::HashMap;

use std::iter::Iterator;
use std::iter::FromIterator;

fn _part1(content: String) {
    let total_unique = content
        .lines()
        .fold(0, |total, line| {
            let splits: Vec<&str> = line
                .split(" | ")
                .collect();

            let count_unique = splits[1]
                .split(" ")
                .fold(0, |acc, n| {
                    match n.len() {
                        2 |3 | 4 | 7 => acc + 1,
                        _            => acc,
                    }
                });

            total + count_unique
        });

    println!("total: {}", total_unique);
}

// Number of chars in 'a' but not in 'b'
fn str_minus_count(a: &str, b: &str) -> usize {
    let mut count = 0;

    for c1 in a.chars() {
        let mut found = false;

        for c2 in b.chars() {
            if c1 == c2 {
                found = true;
                break;
            }
        }

        if !found {
            count += 1;
        }
    }

    count
}

fn sort_chars(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));

    String::from_iter(chars)
}

fn part2(content: String) {
    let mut result = 0;

    for line in content.lines() {
        let mut map: HashMap<String, i32> = HashMap::new();

        let splits: Vec<&str> = line
            .split(" | ")
            .collect();

        let mut encoded: Vec<&str> = splits[0]
            .split(" ")
            .collect();

        encoded.sort_by(|a, b| {
            a.len().cmp(&(b.len()))
        });

        map.insert(sort_chars(encoded[0]), 1);
        map.insert(sort_chars(encoded[2]), 4);
        map.insert(sort_chars(encoded[1]), 7);
        map.insert(sort_chars(encoded[9]), 8);

        for i in 3..6 {
            if str_minus_count(encoded[2], encoded[i]) == 1 {
                map.insert(sort_chars(encoded[i]), 5);
            }

            if str_minus_count(encoded[2], encoded[i]) == 2 {
                map.insert(sort_chars(encoded[i]), 2);
            }

            if str_minus_count(encoded[0], encoded[i]) == 0 {
                map.insert(sort_chars(encoded[i]), 3);
            }
        }

        // 0, 6, 9
        for i in 6..9 {
            if str_minus_count(encoded[0], encoded[i]) != 0 {
                map.insert(sort_chars(encoded[i]), 6);
            }

            if str_minus_count(encoded[2], encoded[i]) == 0 {
                map.insert(sort_chars(encoded[i]), 9);
            }

            let test = (str_minus_count(encoded[2], encoded[i]) == 1) &&
                       (str_minus_count(encoded[0], encoded[i]) == 0);

            if test {
                map.insert(sort_chars(encoded[i]), 0);
            }
        }

        // for (key, val) in map.iter() {
        //     println!("{}: {}", key, val);
        // }

        // println!("\n----  ----\n");

        let displayed: Vec<&str> = splits[1]
            .split(" ")
            .collect();

        let mut value = 0; 

        for n in displayed.iter() {
            let sorted = sort_chars(n);
            let v = map.get(&sorted).unwrap();

            value = 10 * value + v;
        }

        // println!("value: {}", value);
    
        result += value;
    }

    println!("result: {}", result);
}
  
fn main() {
    let filename = "./inputs/day08.txt";

    let content = fs::read_to_string(filename)
        .expect("Could not read file");

    part2(content);
}
