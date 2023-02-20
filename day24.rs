use std::fs;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Copy, Clone)]
enum Operand {
    Empty,
    Var(char),
    Value(i64),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operator {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Debug, Copy, Clone)]
struct Instruction(Operator, char, Operand);

struct ALU {
    x: i64,
    y: i64,
    z: i64,
    w: i64,

    instructions: Vec<Instruction>,
}

impl ALU {
    fn new(instructions: Vec<Instruction>) -> Self {
        ALU {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
            instructions,
        }
    }

    fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.z = 0;
        self.w = 0;
    }

    fn load(&mut self, name: char, value: i64) {
        match name {
            'x' => self.x = value,
            'y' => self.y = value,
            'z' => self.z = value,
            'w' => self.w = value,
            _ => unreachable!(),
        };
    }

    fn get(&mut self, name: char) -> i64 {
        match name {
            'x' => self.x,
            'y' => self.y,
            'z' => self.z,
            'w' => self.w,
            _ => unreachable!(),
        }
    }

    // https://www.keiruaprod.fr/blog/2021/12/29/a-comprehensive-guide-to-aoc-2021-day-24.html
    fn solve(&self, least_value: bool) -> i64 {
        // let z_divs = [ 1,  1,  1,  26,  1, 26,  1,  1,  1, 26, 26,  26,  26,  26];
        // let x_incs = [10, 13, 13, -11, 11, -4, 12, 12, 15, -2, -5, -11, -13, -10];
        // let y_incs = [13, 10,  3,   1,  9,  3,  5,  1,  0, 13,  7,  15,  12,   8];

        let mut z_divs = [0i64; 14];
        let mut x_incs = [0i64; 14];
        let mut y_incs = [0i64; 14];

        for i in 0..14 {
            let start = i * 18;

            let Instruction(_, _, z_div) = self.instructions[start + 4];

            match z_div {
                Operand::Value(v) => z_divs[i] = v,
                _ => unreachable!(),
            }

            let Instruction(_, _, x_inc) = self.instructions[start + 5];

            match x_inc {
                Operand::Value(v) => x_incs[i] = v,
                _ => unreachable!(),
            }

            let Instruction(_, _, y_inc) = self.instructions[start + 15];

            match y_inc {
                Operand::Value(v) => y_incs[i] = v,
                _ => unreachable!(),
            }
        }

        let mut zs: HashSet<i64> = HashSet::new();
        let mut result: HashMap<i64, Vec<i64>> = HashMap::new();

        zs.insert(0);

        for ops_idx in (0..14).rev() {
            let mut updated_zs = HashSet::new();

            for _w in 1..=9 {
                let w = 
                    if !least_value {
                        _w
                    } else {
                        10 - _w
                    };

                for z in zs.iter() {
                    let generated_zs =
                        go_backward(*z, w, x_incs[ops_idx], y_incs[ops_idx], z_divs[ops_idx]);

                    // println!("w: {}, len {}", w, result.len());
                    for z0 in generated_zs {
                        updated_zs.insert(z0);

                        if let Some(existing_solutions) = result.get(z) {
                            let mut new_existing_solutions = existing_solutions.clone();
                            new_existing_solutions.insert(0, w);

                            result.insert(z0, new_existing_solutions);
                        } else {
                            result.insert(z0, vec![w]);
                        }
                    }
                }
            }

            zs = updated_zs;
        }

        // for (k, v) in result.iter() {
        //     println!("{}: {:?}", k, v);
        // }

        if let Some(r) = result.get(&0) {
            let solution = r.iter().fold(0, |acc, value| acc * 10 + value);

            return solution;
        }

        return -1;
    }

    fn exec(&mut self, input: i64) {
        let mut current = input;
        let size = self.instructions.len();

        let mut power = 10000000000000;

        for i in 0..size {
            let Instruction(op, var1_name, var2) = self.instructions[i];

            if op == Operator::Inp {
                let value = current / power;
                current = current % power;
                power = power / 10;

                if value == 0 {
                    self.z = -1;
                    return;
                }

                self.load(var1_name, value);

                continue;
            };

            let mut var1_value = self.get(var1_name);
            let var2_value =
                match var2 {
                    Operand::Value(value) => {
                        value
                    }
                    Operand::Var(var2_name) => {
                        self.get(var2_name)
                    }
                    Operand::Empty => unreachable!()
                };

            match op {
                Operator::Add => var1_value += var2_value,
                Operator::Mul => var1_value *= var2_value,
                Operator::Div => var1_value /= var2_value,
                Operator::Mod => var1_value %= var2_value,
                Operator::Eql => {
                    if var1_value == var2_value {
                        var1_value = 1;
                    } else {
                        var1_value = 0;
                    }
                },
                _ => unreachable!(),
            }

            self.load(var1_name, var1_value);
        }
    }
}

/*
 * x(n) = (z(n-1) % 26) + x_inc
 * z(n) = 26 * (z(n-1) // z_div) + w + y_inc, if x(n) != w
 * z(n) = z(n-1) // divisor                 , if x(n) == w
 */
fn go_backward(zf: i64, w: i64, x_inc: i64, y_inc: i64, z_div: i64) -> Vec<i64> {
    let mut possible_zs = vec![];

    // k = zf - w - y_inc = 26 * (zf // z_div)
    let k = zf - w - y_inc;

    // testing if k is a multiple of 26
    if k % 26 == 0 {
        let t = k / 26;
        let zi = t * z_div;

        possible_zs.push(zi);
    }

    // After an integer division, the remainder is lost
    // Testing if w - x_inc can be the remainder of zi % 26,
    // where can be derived from x(n) = (z(n-1) % 26) + x_inc
    // zi % 26 = w - x_inc, when x(n) = w
    if 0 <= (w - x_inc) && (w - x_inc) < 26 {
        let zi = zf * 26 + w - x_inc;
        possible_zs.push(zi);
    }

    return possible_zs;
}


fn parse_instruction(line: &str) -> Instruction {
    let terms: Vec<&str> = line
        .split(' ')
        .collect();

    if terms.len() < 2 {
        unreachable!();
    }

    let var1: char = terms[1].chars().nth(0).unwrap();

    if terms[0] == "inp" {
        return Instruction(Operator::Inp, var1, Operand::Empty);
    }

    let var2 = 
        if let Ok(value) = i64::from_str_radix(terms[2], 10) {
            Operand::Value(value)
        } else {
            let v: char = terms[2].chars().nth(0).unwrap();

            Operand::Var(v)
        };

    match terms[0] {
        "add" => Instruction(Operator::Add, var1, var2),
        "mul" => Instruction(Operator::Mul, var1, var2),
        "div" => Instruction(Operator::Div, var1, var2),
        "mod" => Instruction(Operator::Mod, var1, var2),
        "eql" => Instruction(Operator::Eql, var1, var2),
        _ => unreachable!()
    }
}

fn part1(content: String) {
    let instructions: Vec<Instruction> = content
        .lines()
        .map(|line| parse_instruction(line))
        .collect();

    let mut alu = ALU::new(instructions);
    let input = alu.solve(false);

    if input == -1 {
        println!("no solutions found");
        return;
    }

    alu.reset();
    alu.exec(input);

    println!("input: {}", input);
    println!("x: {}, y: {}, z: {}, w: {}", alu.x, alu.y, alu.z, alu.w);
}

fn part2(content: String) {
    let instructions: Vec<Instruction> = content
        .lines()
        .map(|line| parse_instruction(line))
        .collect();

    let mut alu = ALU::new(instructions);
    let input = alu.solve(true);

    if input == -1 {
        println!("no solutions found");
        return;
    }

    alu.reset();
    alu.exec(input);

    println!("input: {}", input);
    println!("x: {}, y: {}, z: {}, w: {}", alu.x, alu.y, alu.z, alu.w);
}

fn main() {
    let filename = "./inputs/day24.txt";
    let data = fs::read_to_string(filename)
        .expect("could not read file");

    // part1(data);
    part2(data);
}

/*
z = 10 + w
z = -(w + 8) * 26 + 10 + w


a = [ 1,  1,  1,  26,  1, 26,  1,  1,  1, 26, 26,  26,  26,  26];
b = [10, 13, 13, -11, 11, -4, 12, 12, 15, -2, -5, -11, -13, -10];
c = [13, 10,  3,   1,  9,  3,  5,  1,  0, 13,  7,  15,  12,   8];

w = read
x = z % 26 + b;

if x != w {
    x = 1;
} else {
    x = 0;
}

z = z / a;
z = x * (25z + w + c) + z


w = read            w = v1
x = x * 0;          x = 0   // mul x 0
x = x + z;          z = 0   // add x z
x = x % 26;         x = 0  // mod x 26
z = z / 1;          z = 0   // div z 1
x = x + 10;         x = 10  // add x 10
x = x == w ? 1 : 0; x = 0   // eql x w
x = x == 0 ? 1 : 0; x = 1   // eql x 0
y = y * 0;          y = 0   // mul y 0
y = y + 25;         y = 25  // add y 25
y = y * x;          y = 25 // mul y x
y = y + 1;          y = 26 // add y 1
z = z * y;          z = 0  // mul z y
y = y * 0;          y = 0   // mul y 0
y = y + w           y = v1  // add y w
y = y + 13;         y = v1 + 13 // add y 13
y = y * x;          y = v1 + 13 // mul y x
z = z + y;          z = v1 + 13 // add z y

w = read            w = v2
x = x * 0;          x = 0          // mul x 0
x = x + z;          x = v1 + 13;   // add x z
x = x % 26;         x = 0          // mod x 26
z = z / 1;          z = v1 + 13;   // div z 1
x = x + 13;         x = 13         // add x 13
x = x == w ? 1 : 0; x = 0          // eql x w
x = x == 0 ? 1 : 0; x = 1          // eql x 0
y = y * 0;          y = 0;         // mul y 0
y = y + 25;         y = 25;        // add y 25
y = y * x;          y = 25;        // mul y x
y = y + 1;          y = 26;        // add y 1
z = z * y;          z = (v1+13)*26 // mul z y
y = y * 0;          y = 0;         // mul y 0
y = y + w;          y = v2;        // add y w
y = y + 10;         y = v2 + 10    // add y 10
y = y * x;          y = v2 + 10    // mul y x
z = z + y;          z = (v1+13)*26+v2+10 //  add z y
*/
