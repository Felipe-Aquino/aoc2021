use std::fs;
use std::collections::HashMap;

struct Letter {
    value: char,
    count: usize,
}

#[allow(dead_code)]
fn part1(content: String, iterations: usize) {
    let mut lines = content
        .lines()
        .collect::<Vec<&str>>();

    let mut input = String::from(lines[0]);

    lines.drain(..2);

    let mut pairs = HashMap::new();

    for line in lines.iter() {
        let splited: Vec<&str> = line
            .split(" -> ")
            .collect();

        pairs.insert(
            splited[0].to_string(),
            splited[1].to_string()
        );
    }

    for _ in 0..iterations {
        let mut result = String::new();

        for i in 0..input.len() - 1 {
            if let Some(s) = input.get(i..=i + 1) {
                if let Some(m) = pairs.get(s) {
                    let chr = s.get(0..1).unwrap();

                    result.push_str(chr);
                    result.push_str(m.as_str());
                }
            }
        }

        let chr = input
            .get(input.len() - 1..)
            .unwrap();

        result.push_str(chr);

        input = result;

        println!("---> {}", input);
    }

    let mut letters: Vec<Letter> = vec![];

    for c in input.chars() {
        let r = letters
            .iter_mut()
            .find(|l| l.value == c);

        if let Some(letter) = r {
            letter.count += 1;
        } else {
            let letter = Letter {
                value: c,
                count: 1
            };

            letters.push(letter);
        }
    }

    letters.sort_by(|a, b| {
        a.count.partial_cmp(&b.count).unwrap()
    });

    let min_count =
        match letters.first() {
            Some(l) => l.count,
            None => 0,
        };

    let max_count =
        match letters.last() {
            Some(l) => l.count,
            None => 0,
        };

    println!("min: {}, max: {}, diff: {}", min_count, max_count, max_count - min_count);
}

struct State {
    pairs: HashMap<String, String>,
    iters: Vec<(String, usize, HashMap<String, usize>)>,
}

impl State {
    fn new(lines: Vec<&str>) -> State {
        let mut pairs = HashMap::new();

        for line in lines.iter() {
            let splited: Vec<&str> = line
                .split(" -> ")
                .collect();

            pairs.insert(
                splited[0].to_string(),
                splited[1].to_string()
            );
        }

        State {
            pairs,
            iters: vec![],
        }
    }

    fn add_countings(c1: &mut HashMap<String, usize>, c2: &mut HashMap<String, usize>) {
        for (key, val) in c2.iter() {
            if let Some(&mut x) = c1.get_mut(key) {
                c1.insert(key.clone(), x + val);
            } else {
                c1.insert(key.clone(), *val);
            }
        }
    }

    fn test(&mut self, top_counting: &mut HashMap<String, usize>, v: String, max_iter: usize) {
        if max_iter == 0 {
            return;
        }

        let exists = self.iters.iter_mut().find(|(l, iter, _)| { 
            l.as_str() == v.as_str() && *iter == max_iter
        });

        if let Some(rule) = exists {
            // println!("-- {:?}", rule.2);
            State::add_countings(top_counting, &mut rule.2);
            return;
        }

        let mut rule_counting: HashMap<String, usize> = HashMap::new();

        let mut lhs = String::new();
        let mut rhs = String::new();

        {
            let l = self.pairs.get_mut(&v).unwrap();

            rule_counting.insert(l.clone(), 1);

            lhs.push_str(v.get(0..1).unwrap());
            lhs.push_str(l.as_str());

            rhs.push_str(l.as_str());
            rhs.push_str(v.get(1..2).unwrap());
        }

        self.test(&mut rule_counting, lhs, max_iter - 1);

        self.test(&mut rule_counting, rhs, max_iter - 1);

        State::add_countings(top_counting, &mut rule_counting);
        // println!("** {:?}", rule_counting);
        // println!("-- {:?}", top_counting);

        self.iters.push((v, max_iter, rule_counting));
    }

    fn process(&mut self, input: String) {
        let mut counting: HashMap<String, usize> = HashMap::new();

        for i in 0..input.len() - 1 {
            if let Some(s) = input.get(i..=i + 1) {
                self.test(&mut counting, s.to_string(), 40);
            }
        }

        for i in 0..input.len() {
            if let Some(s) = input.get(i..i + 1) {
                // println!(":: {}", s);
                if let Some(&mut c) = counting.get_mut(s) {
                    counting.insert(s.to_string(), c + 1);
                } else {
                    counting.insert(s.to_string(), 1);
                }
            }
        }

        // println!("-- {:?}", counting);

        let values = counting
            .values()
            .map(|&v| v)
            .collect::<Vec<usize>>();

        let mut min_value = values[0];
        let mut max_value = values[0];

        for i in 1..values.len() {
            if values[i] > max_value {
                max_value = values[i];
            }

            if values[i] < min_value {
                min_value = values[i];
            }
        }

        println!("min: {}, max: {}, diff: {}", min_value, max_value, max_value - min_value);
    }
}

fn part2(content: String) {
    let mut lines = content
        .lines()
        .collect::<Vec<&str>>();

    let input = String::from(lines[0]);

    lines.drain(..2);

    let mut state = State::new(lines);

    state.process(input);
}

fn main() {
    // let filename = "./inputs/day14-example.txt";
    let filename = "./inputs/day14.txt";

    let content = fs::read_to_string(filename)
        .expect("Could not read file");

    part2(content);
}
