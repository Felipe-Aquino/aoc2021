use std::fs;

struct Bounds {
    row_start: i32,
    row_end: i32,
    col_start: i32,
    col_end: i32,
}

fn expand_input(inputs: &mut Vec<String>) {
    for i in 0..inputs.len() {
        let mut input = String::new();

        input.push_str("..");
        input.push_str(inputs[i].as_str());
        input.push_str("..");

        inputs[i] = input;
    }

    let col_count = inputs[0].len();

    let ln = vec![b'.'; col_count];
    inputs.insert(0, String::from_utf8(ln).unwrap());

    let ln = vec![b'.'; col_count];
    inputs.insert(0, String::from_utf8(ln).unwrap());

    let ln = vec![b'.'; col_count];
    inputs.push(String::from_utf8(ln).unwrap());

    let ln = vec![b'.'; col_count];
    inputs.push(String::from_utf8(ln).unwrap());
}

fn expand_input2(inputs: &mut Vec<String>) {
    for i in 0..inputs.len() {
        let mut input = String::new();

        input.push_str("##");
        input.push_str(inputs[i].as_str());
        input.push_str("##");

        inputs[i] = input;
    }

    let col_count = inputs[0].len();

    let ln = vec![b'#'; col_count];
    inputs.insert(0, String::from_utf8(ln).unwrap());

    let ln = vec![b'#'; col_count];
    inputs.insert(0, String::from_utf8(ln).unwrap());

    let ln = vec![b'#'; col_count];
    inputs.push(String::from_utf8(ln).unwrap());

    let ln = vec![b'#'; col_count];
    inputs.push(String::from_utf8(ln).unwrap());
}

fn generate_output(inputs: &Vec<String>, enhancement: &[u8]) -> Vec<String> {
    let mut outputs: Vec<String> = vec![];

    for i in 1..inputs.len() - 1 {
        let input = inputs[i].as_str();
        let mut output = String::new();

        for j in 1..input.len() - 1 {
            // let bounds = frame_bounds(i, j, inputs.len(), input.len());
            let bounds = Bounds {
                row_start: (i as i32) - 1,
                row_end:   (i as i32) + 1,
                col_start: (j as i32) - 1,
                col_end:   (j as i32) + 1,
            };

            let mut value: usize = 0x0;
            let mut mask: usize = 0x100;

            for fi in bounds.row_start..=bounds.row_end {
                let frame_line =
                    if fi < 0 || fi >= inputs.len() as i32 {
                        inputs[0].as_bytes()
                    } else {
                        inputs[fi as usize].as_bytes()
                    };

                for fj in bounds.col_start..=bounds.col_end {
                    if fi < 0 || fi >= inputs.len() as i32 {
                        mask = mask >> 1;
                        continue;
                    }

                    if fj < 0 || fj >= frame_line.len() as i32 {
                        mask = mask >> 1;
                        continue;
                    }

                    if frame_line[fj as usize] == b'#' {
                        value = value | mask;
                    }

                    mask = mask >> 1;
                }
            }

            // print!("{}", enhancement[value] as char);
            output.push(enhancement[value] as char);
        }

        // print!("\n");
        outputs.push(output);
    }

    outputs
}

fn count_lit_pixels(inputs: &Vec<String>) -> usize {
    let mut total = 0;

    for input in inputs {
        for c in input.chars() {
            if c == '#' {
                total += 1;
            }
        }
    }

    total
}

fn part1(content: String) {
    let mut enhancement = "".as_bytes();
    let mut inputs: Vec<String> = vec![];

    for (i, line) in content.lines().enumerate() {
        if i == 0 {
            enhancement = line.clone().as_bytes();
            continue;
        }

        // skiping empty line
        if i > 1 {
            inputs.push(line.into());
        }
    }

    expand_input(&mut inputs);

    for i in 0..inputs.len() {
        println!("{}", inputs[i]);
    }

    println!("_____________________");

    let mut inputs = generate_output(&inputs, &enhancement);

    // expand_input(&mut inputs); // use this for example

    // because this especific case the enhancement starts with '#' and ends with '.'
    // litting all pixels and reverting back
    if enhancement[0] == b'.' {
        expand_input(&mut inputs);
    } else if enhancement[enhancement.len() - 1] == b'.' {
        expand_input2(&mut inputs);
    } else {
        unreachable!();
    }

    println!("_____________________");

    let inputs = generate_output(&inputs, &enhancement);


    println!("\nlit total: {}", count_lit_pixels(&inputs));
}

fn part2(content: String) {
    let mut enhancement = "".as_bytes();
    let mut inputs: Vec<String> = vec![];

    for (i, line) in content.lines().enumerate() {
        if i == 0 {
            enhancement = line.clone().as_bytes();
            continue;
        }

        // skiping empty line
        if i > 1 {
            inputs.push(line.into());
        }
    }

    for _ in 0..25 {
        expand_input(&mut inputs);

        inputs = generate_output(&inputs, &enhancement);

        // expand_input(&mut inputs); // use this for example

        // because this especific case the enhancement starts with '#' and ends with '.'
        // litting all pixels and reverting back
        if enhancement[0] == b'.' {
            expand_input(&mut inputs);
        } else if enhancement[enhancement.len() - 1] == b'.' {
            expand_input2(&mut inputs);
        } else {
            unreachable!();
        }

        inputs = generate_output(&inputs, &enhancement);
    }

    println!("\nlit total: {}", count_lit_pixels(&inputs));
}

fn main() {
    // let filename = "./inputs/day20-example.txt";
    let filename = "./inputs/day20.txt";

    let data = fs::read_to_string(filename)
        .expect("could not read file");

    part1(data);
}
