use std::fs;
use std::collections::HashSet;

fn _part1(content: String) {
    let values: Vec<Vec<i32>> = content
        .lines()
        .map(|line| {
            line
                .trim()
                .chars()
                .map(|c| {
                    c.to_digit(10).unwrap() as i32
                })
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut low_points: Vec<i32> = vec![];

    let nrows = values.len();
    let ncols = values[0].len();

    for i in 0..nrows {
        for j in 0..ncols {
            let mut is_lowest = true;

            if i > 0 {
                is_lowest &= values[i][j] < values[i - 1][j];
            }

            if i < nrows - 1 {
                is_lowest &= values[i][j] < values[i + 1][j];
            }

            if j > 0 {
                is_lowest &= values[i][j] < values[i][j - 1];
            }

            if j < ncols - 1 {
                is_lowest &= values[i][j] < values[i][j + 1];
            }

            if is_lowest {
                low_points.push(values[i][j]);
            }
        }
    }

    let sum_of_risk_levels = low_points
        .iter()
        .fold(0, |acc, v| acc + v + 1);

    println!("sum of risk levels: {}", sum_of_risk_levels);
}

fn basin_walk(values: &Vec<Vec<i32>>, pts: &mut HashSet<(usize, usize)>, i: usize, j: usize) {
    let nrows = values.len();
    let ncols = values[0].len();

    if values[i][j] >= 9 {
        return;
    }

    pts.insert((i, j));

    if i > 0 && values[i][j] < values[i - 1][j] {
        basin_walk(values, pts, i - 1, j);
    }

    if i < nrows - 1 && values[i][j] < values[i + 1][j]{
        basin_walk(values, pts, i + 1, j);
    }

    if j > 0 && values[i][j] < values[i][j - 1] {
        basin_walk(values, pts, i, j - 1);
    }

    if j < ncols - 1 && values[i][j] < values[i][j + 1]{
        basin_walk(values, pts, i, j + 1);
    }
}

fn part2(content: String) {
    let values: Vec<Vec<i32>> = content
        .lines()
        .map(|line| {
            line
                .trim()
                .chars()
                .map(|c| {
                    c.to_digit(10).unwrap() as i32
                })
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut basin_sizes: Vec<usize> = vec![];

    let nrows = values.len();
    let ncols = values[0].len();

    for i in 0..nrows {
        for j in 0..ncols {
            let mut is_lowest = true;

            if i > 0 {
                is_lowest &= values[i][j] < values[i - 1][j];
            }

            if i < nrows - 1 {
                is_lowest &= values[i][j] < values[i + 1][j];
            }

            if j > 0 {
                is_lowest &= values[i][j] < values[i][j - 1];
            }

            if j < ncols - 1 {
                is_lowest &= values[i][j] < values[i][j + 1];
            }

            if is_lowest {
                let mut points: HashSet<(usize, usize)> = HashSet::new();
                basin_walk(&values, &mut points, i, j);
                basin_sizes.push(points.len());
            }
        }
    }

    basin_sizes.sort();

    let mut result = 1;

    let v = basin_sizes.pop().unwrap();
    result *= v;

    println!(":: {}", v);

    let v = basin_sizes.pop().unwrap();
    result *= v;

    println!(":: {}", v);

    let v = basin_sizes.pop().unwrap();
    result *= v;

    println!(":: {}", v);

    println!("\nresult {}", result);
}


fn main() {
    let filename = "./inputs/day09.txt";

    let content = fs::read_to_string(filename)
        .expect("Could not open file");

    part2(content);
}
