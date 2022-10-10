use std::fs;

#[derive(PartialEq, Eq, Debug)]
enum TokenType {
    Num(u32),
    LeftBracket,
    RightBracket,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Marker {
    NumNum,
    NumList,
    ListNum,
    ListList,
    End,
    Value(u32),
}

fn marker_to_value(m: &Marker) -> i32 {
    match m {
        Marker::NumNum => -1,
        Marker::NumList => -2,
        Marker::ListNum => -3,
        Marker::ListList => -4,
        Marker::End => -5,
        Marker::Value(n) => *n as i32,
    }
}

fn read_tokens(input: &str) -> Vec<TokenType> {
    let mut tokens: Vec<TokenType> = vec![];

    let mut chs = input.chars();

    let mut next_ch = chs.next();

    while !next_ch.is_none() {
        let mut c = next_ch.unwrap();

        if c == '[' {
            tokens.push(TokenType::LeftBracket);
        } else if c == ']' {
            tokens.push(TokenType::RightBracket);
        } else if c.is_ascii_digit() {
            let mut num_str = String::new();

            while !next_ch.is_none() {
                c = next_ch.unwrap();

                if c.is_ascii_digit() {
                    num_str.push(c);
                    next_ch = chs.next();
                } else {
                    break;
                }
            }

            let n = u32::from_str_radix(num_str.as_str(), 10).unwrap();

            tokens.push(TokenType::Num(n));
            continue;
        }

        next_ch = chs.next();
    }

    return tokens;
}

/*
 * -1 -> [number, number]
 * -2 -> [number, list]
 * -3 -> [list,   number]
 * -4 -> [list,   list]
 */

struct Snailfish {
    number: Vec<Marker>,
    current_pos: i32,
    print_steps: bool,
}

impl Snailfish {
    fn new() -> Self {
        Snailfish {
            number: vec![],
            current_pos: 1,
            print_steps: false,
        }
    }

    fn copy(&self) -> Self {
        let mut number = vec![];

        for i in 0..self.number.len() {
            number.push(self.number[i]);
        }

        Snailfish {
            number: number,
            current_pos: 1,
            print_steps: false,
        }
    }

    fn parse(&mut self, tokens: &Vec<TokenType>, start: usize) -> Option<usize> {
        use TokenType::*;

        if tokens[start] == LeftBracket {
            let pos = self.number.len();

            self.number.push(Marker::NumNum);

            let n1 = match tokens[start + 1] {
                Num(n) => {
                    self.number.push(Marker::Value(n));
                    start + 2
                }
                LeftBracket => {
                    self.number[pos] = Marker::ListNum;
                    self.parse(tokens, start + 1).unwrap()
                }
                RightBracket => unreachable!(),
            };

            let n2 = match tokens[n1] {
                Num(n) => {
                    self.number.push(Marker::Value(n));
                    n1 + 1
                }
                LeftBracket => {
                    match self.number[pos] {
                        Marker::NumNum => {
                            self.number[pos] = Marker::NumList;
                        }
                        Marker::NumList => { // TODO: This case won't happpen, maybe remove it
                            self.number[pos] = Marker::ListNum;
                        }
                        Marker::ListNum => {
                            self.number[pos] = Marker::ListList;
                        }
                        _ => {}
                    }

                    self.parse(tokens, n1).unwrap()
                }
                RightBracket => unreachable!(),
            };

            assert!(tokens[n2] == RightBracket);
            self.number.push(Marker::End);

            return Some(n2 + 1);
        }

        None
    }

    fn add(&mut self, other: Snailfish) {
        self.number.insert(0, Marker::ListList);

        for v in other.number {
            self.number.push(v);
        }

        self.number.push(Marker::End);

        if self.print_steps {
            self.print_inline(0, false);
        }

        let mut ok = true;

        while ok {
            ok = self.explode();

            if self.print_steps {
                println!("---");
            }

            if !ok {
                ok = self.split();
            }
        }

        self.current_pos = -1;
    }

    fn explode_at(&mut self, i: usize) -> bool {
        match self.number[i] {
            Marker::NumNum => {
                let lhs = marker_to_value(&self.number[i + 1]) as u32;
                let rhs = marker_to_value(&self.number[i + 2]) as u32;

                for j in (0..i).rev() {
                    let n = marker_to_value(&self.number[j]);

                    if n >= 0 {
                        self.number[j] = Marker::Value(n as u32 + lhs);
                        break;
                    }
                }

                for j in i + 3..self.number.len() {
                    let n = marker_to_value(&self.number[j]);

                    if n >= 0 {
                        self.number[j] = Marker::Value(n as u32 + rhs);
                        break;
                    }
                }

                self.number[i] = Marker::Value(0);

                for j in (0..i).rev() {
                    match self.number[j] {
                        Marker::NumList | Marker::ListNum => {
                            self.number[j] = Marker::NumNum;
                            break;
                        }
                        Marker::ListList => {
                            self.number[j] = Marker::NumList;
                            break;
                        }
                        _ => {},
                    }
                }

                self.number.drain(i + 1..i + 4);

                return true;
            }
            _ => {}
        }

        return false;
    }

    fn explode(&mut self) -> bool {
        let mut i = 0;

        let mut bracket_count = 0;

        let mut exploded = false;

        'outer: while i < self.number.len() {
            let marker_value = marker_to_value(&self.number[i]);

            if self.print_steps && marker_value < 0 && marker_value > -5 {
                self.current_pos = i as i32;
                self.print_inline(0, false);
            }

            match self.number[i] {
                Marker::NumNum => {
                    bracket_count += 1;

                    if bracket_count > 4 {
                        if self.print_steps {
                            println!("{}: ... {} ... (explode)\n", i, bracket_count);
                        }

                        self.explode_at(i);
                        exploded = true;

                        break 'outer;
                    } else {
                        i += 3;
                    }
                },
                Marker::NumList | Marker::ListNum | Marker::ListList => {
                    bracket_count += 1;

                    i += 1;
                },
                Marker::End => {
                    bracket_count -= 1;
                    i += 1;

                    continue;
                },
                Marker::Value(_) => {
                    i += 1;

                    continue;
                },
            }

            if self.print_steps {
                println!("{}: ... {} ...\n", i, bracket_count);
            }
        }

        self.current_pos = -1;

        if self.print_steps {
            self.print_inline(0, false);
        }

        return exploded;
    }

    fn split(&mut self) -> bool {
        let mut i = 0;

        let mut splitted = false;

        while i < self.number.len() {
            match self.number[i] {
                Marker::Value(n) if n > 9 => {
                    let lhs = n / 2;
                    let rhs = n - lhs;

                    self.number[i] = Marker::NumNum;
                    self.number.insert(i + 1, Marker::Value(lhs));
                    self.number.insert(i + 2, Marker::Value(rhs));
                    self.number.insert(i + 3, Marker::End);

                    let mut bracket_end_count = 0;

                    for j in (0..i).rev() {
                        match self.number[j] {
                            Marker::NumNum => {
                                if bracket_end_count == 0 {
                                    if i - j == 1 {
                                        self.number[j] = Marker::ListNum;
                                    } else if i - j == 2 {
                                        self.number[j] = Marker::NumList;
                                    } else {
                                        unreachable!();
                                    }
                                    break;
                                } else {
                                    bracket_end_count -= 1;
                                }
                            }
                            Marker::NumList | Marker::ListNum => {
                                if bracket_end_count == 0 {
                                    self.number[j] = Marker::ListList;
                                    break;
                                } else {
                                    bracket_end_count -= 1;
                                }
                            }
                            Marker::End => {
                                bracket_end_count += 1;
                            },
                            Marker::ListList=> {
                                if bracket_end_count > 0 {
                                    bracket_end_count -= 1;
                                }
                            },
                            _ => {
                            }
                        }
                    }

                    splitted = true;
                    break;
                }
                _ => {
                    i += 1;
                }
            }
        }

        return splitted;
    }

    fn bracket_count_until(&self, pos: usize) -> usize {
        let mut i = 0;

        let mut bracket_count = 0;

        while i < self.number.len() {
            match self.number[i] {
                Marker::End => {
                    bracket_count -= 1;
                },
                Marker::Value(_) => {
                },
                _ => {
                    bracket_count += 1;
                },
            }

            if pos < i {
                break;
            }

            i += 1;
        }

        return bracket_count;
    }

    fn _find_sublist_end(&self, start: usize) -> usize {
        let mut bracket_count = 0;
        let mut i = start;

        while i < self.number.len() {
            let value = marker_to_value(&self.number[i]);

            if value == -5 {
                bracket_count -= 1;
            } else if value < 0 {
                bracket_count += 1;
            }

            if bracket_count == 0 {
                break;
            }

            i += 1;
        }

        return i;
    }

    fn magnetude(&self, start: usize) -> u32 {
        let pos = start;

        match self.number[pos] {
            Marker::NumNum => {
                let lhs = marker_to_value(&self.number[pos + 1]) as u32;
                let rhs = marker_to_value(&self.number[pos + 2]) as u32;

                3 * lhs + 2 * rhs
            }
            Marker::NumList => {
                let lhs = marker_to_value(&self.number[pos + 1]) as u32;

                let rhs_start = pos + 2;

                let rhs = self.magnetude(rhs_start);

                3 * lhs + 2 * rhs
            }
            Marker::ListNum => {
                let lhs_start = pos + 1;
                let lhs_end = self._find_sublist_end(lhs_start);

                let lhs = self.magnetude(lhs_start);
                let rhs = marker_to_value(&self.number[lhs_end + 1]) as u32;

                3 * lhs + 2 * rhs
            }
            Marker::ListList => {
                let lhs_start = pos + 1;
                let lhs_end = self._find_sublist_end(lhs_start);

                let lhs = self.magnetude(lhs_start);

                let rhs_start = lhs_end + 1;

                let rhs = self.magnetude(rhs_start);

                3 * lhs + 2 * rhs
            }
            _ => {
                unreachable!();
            },
        }
    }

    fn print_inline(&self, pos: usize, color_next: bool) -> usize {
        let colored = self.current_pos == pos as i32;

        if colored {
            print!("\x1B[31m[\x1B[0m");
        } else if color_next {
            print!("\x1B[32m[\x1B[0m");
        } else {
            print!("[");
        }

        let next_pos = match self.number[pos] {
            Marker::NumNum => {
                let lhs = marker_to_value(&self.number[pos + 1]);
                let rhs = marker_to_value(&self.number[pos + 2]);
                print!("{}, {}", lhs, rhs);

                pos + 4
            }
            Marker::NumList => {
                print!("{}, ", marker_to_value(&self.number[pos + 1]));

                let next_pos = self.print_inline(pos + 2, colored);

                next_pos + 1
            }
            Marker::ListNum => {
                let next_pos = self.print_inline(pos + 1, colored);

                print!(", {}", marker_to_value(&self.number[next_pos]));

                next_pos + 2
            }
            Marker::ListList => {
                let next_pos = self.print_inline(pos + 1, colored);

                print!(", ");

                let c =
                    self.current_pos == pos as i32 + 1 && self.number[pos + 1] == Marker::NumNum;

                let next_pos = self.print_inline(next_pos, c);

                next_pos + 1
            }
            _ => {
                println!("\n{:?}", self.number);
                unreachable!();
            },
        };

        if colored {
            print!("\x1B[31m]\x1B[0m");
        } else if color_next {
            print!("\x1B[32m]\x1B[0m");
        } else {
            print!("]");
        }

        if pos == 0 {
            print!("\n");
        }

        next_pos
    }
}

fn part1(content: String) {
    let mut snails = content
        .lines()
        .map(|line| {
            let tokens = read_tokens(line);

            let mut snail = Snailfish::new();

            snail.parse(&tokens, 0);

            snail
        })
        .collect::<Vec<Snailfish>>();

    let mut result = snails.remove(0);

    while snails.len() > 0 {
        let mut n = snails.remove(0);

        result.add(n);

        // result.print_inline(0, false);
        // println!("######");
    }

    result.print_inline(0, false);

    println!("mag: {}", result.magnetude(0));
}

fn part2(content: String) {
    let mut snails = content
        .lines()
        .map(|line| {
            let tokens = read_tokens(line);

            let mut snail = Snailfish::new();

            snail.parse(&tokens, 0);

            snail
        })
        .collect::<Vec<Snailfish>>();

    let mut greatest_magnetude = 0;

    for i in 0..snails.len() {
        for j in 0..snails.len() {
            if i != j {
                let mut first  = snails[i].copy();
                let second = snails[j].copy();

                first.add(second);

                let mag = first.magnetude(0);

                if mag > greatest_magnetude {
                    greatest_magnetude = mag;
                }

                let mut first = snails[j].copy();
                let second = snails[i].copy();

                first.add(second);

                let mag = first.magnetude(0);

                if mag > greatest_magnetude {
                    greatest_magnetude = mag;
                }
            }
        }
    }

    println!("mag: {}", greatest_magnetude);
}

fn main() {
    let filename = "inputs/day18.txt";

    let content = fs::read_to_string(filename).expect("Could not read file.");

    part2(content);
}
