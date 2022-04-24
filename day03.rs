use std::fs;

fn _part1(content: String) {
    let lines = content.lines().collect::<Vec<&str>>();

    let num_bits = lines[0].len();
    let mut bits: Vec<i32> = vec![0; num_bits];

    for line in lines.iter() {
        for (i, c) in line.char_indices() {
            if c == '1' {
                bits[i] += 1; 
            }
        }
    }

    let mut gamma_rate: u16 = 0;

    for i in 0..num_bits {
        gamma_rate = gamma_rate << 1;

        if 2 * bits[i] > lines.len() as i32 {
            gamma_rate |= 1;
        } else {
            gamma_rate &= 0xFFFE;
        }
    }

    let mask = 0xFFFF as u16 >> (16 - num_bits);

    let epsilon_rate = !gamma_rate & mask;
    let power = (gamma_rate as u32) * (epsilon_rate as u32);

    println!("gamma: {}, epsilon: {}, power: {}", gamma_rate, epsilon_rate, power);
}

fn bit_criteria(content: Vec<&str>, num_bits: usize, lead_char: char) -> i32 {
    let mut current_bit = 0;

    let mut filtered = content;

    let non_lead_char =
        if lead_char == '1' {
            '0'
        } else {
            '1'
        };

    loop {
        let count = filtered
            .iter()
            .fold(0, |acc, line| {
                let c = line
                    .chars()
                    .nth(current_bit)
                    .unwrap();

                if c == lead_char {
                    acc + 1
                } else {
                    acc
                }
            });

        let keep_char =
            if lead_char == '1' {
                if 2 * count >= filtered.len() {
                    lead_char
                } else {
                    non_lead_char
                }
            } else {
                if 2 * count > filtered.len() {
                    non_lead_char
                } else {
                    lead_char
                }
            };

        let mut aux: Vec<&str> = Vec::new();

        for line in filtered {
            let c = line
                .chars()
                .nth(current_bit)
                .unwrap();

            if c == keep_char {
                aux.push(line);
            }
        }

        filtered = aux;

        current_bit += 1;

        if filtered.len() <= 1 || current_bit == num_bits {
            break;
        }
    }

    i32::from_str_radix(filtered[0], 2)
        .expect("Not a number")
}

fn part2(content: String) {
    let lines = content.lines().collect::<Vec<&str>>();

    let num_bits = lines[0].len();

    let o2_generator_rating = bit_criteria(
        lines.clone(),
        num_bits,
        '1'
    );

    let co2_scrubber_rating = bit_criteria(
        lines.clone(),
        num_bits,
        '0'
    );

    let life_support_rating = o2_generator_rating * co2_scrubber_rating;

    println!(
        "O2 gen: {}, CO2 scrubber: {}, life support: {}",
        o2_generator_rating,
        co2_scrubber_rating,
        life_support_rating
    );
}

fn main() {
    let filename = "./inputs/day03-part1.txt";

    let content = fs::read_to_string(filename)
        .expect("Could not read file");

    part2(content);
}
