use std::fs;

fn state_to_string(state: &Vec<Vec<char>>) -> String {
    state
        .iter()
        .map(|line| line.iter().collect())
        .collect::<Vec<String>>()
        .join("\n")
}

fn state_clone(state: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    state
        .iter()
        .map(|line| line.clone())
        .collect()
}

fn state_overwrite(src: &Vec<Vec<char>>, dest: &mut Vec<Vec<char>>) {
    let rows = src.len();
    let cols = src[0].len();

    for i in 0..rows {
        for j in 0..cols {
            dest[i][j] = src[i][j];
        }
    }
}

fn part1(content: String) {
    let mut state: Vec<Vec<char>> =
        content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut state_copy = state_clone(&state);
    // state_overwrite(&state, &mut state_copy);

    let rows = state.len();
    let cols = state[0].len();

    let mut step_count = 0;

    println!("{}\n\n", state_to_string(&state_copy));

    loop {
        let mut no_moves_found = true;

        // Testing movement to the right
        for i in 0..rows {
            for j in 0..cols {
                if state[i][j] == '>' {
                    let next_j = (j + 1) % cols;

                    if state[i][next_j] == '.' {
                        state_copy[i][j] = state[i][next_j];
                        state_copy[i][next_j] = state[i][j];

                        no_moves_found = false;
                    }
                }
            }
        }

        state_overwrite(&state_copy, &mut state);

        // println!("{}\n************", state_to_string(&state_copy));
        // Testing movement to south
        for i in 0..rows {
            for j in 0..cols {
                if state[i][j] == 'v' {
                    let next_i = (i + 1) % rows;

                    if state[next_i][j] == '.' {
                        state_copy[i][j] = state[next_i][j];
                        state_copy[next_i][j] = state[i][j];

                        no_moves_found = false;
                    }
                }
            }
        }

        step_count += 1;

        // println!("{}\n\n", state_to_string(&state_copy));
        if no_moves_found {
            break;
        }

        state_overwrite(&state_copy, &mut state);
        // break;
    }

    println!("steps: {}", step_count);
    println!("{}", state_to_string(&state));
}

fn main() {
    let filename = "./inputs/day25.txt";

    let data = fs::read_to_string(filename)
        .expect("could read file");

    part1(data);
}
