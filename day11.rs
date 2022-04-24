use std::fs;

struct Cell {
    value: i32,
    flashed: bool,
}

impl Cell {
    fn new(value: i32) -> Self {
        Cell {
            value,
            flashed: false,
        }
    }
}

fn try_inc(v: usize, max: usize) -> usize {
    if v < max {
        v + 1
    } else {
        v
    }
}

fn try_dec(v: usize, min: usize) -> usize {
    if v > min {
        v - 1
    } else {
        v
    }
}

fn print_grid_flashes(grid: &Vec<Vec<Cell>>) {
    for row in grid.iter() {
        for c in row.iter() {
            if c.flashed {
                print!("*");
            } else {
                print!("{}", c.value);
            }
        }
        println!("");
    }
    println!(" --- ");
}

fn count_and_reset_grid_flashes(grid: &mut Vec<Vec<Cell>>) -> usize {
    let mut total = 0;

    for row in grid.iter_mut() {
        for c in row.iter_mut() {
            if c.flashed {
                total += 1;
                c.flashed = false;
            }
        }
    }

    total
}

fn walk_cell(grid: &mut Vec<Vec<Cell>>, i: usize, j: usize) {
    let nrows = grid.len();
    let ncols = grid[0].len();

    let start_i = try_dec(i, 0);
    let end_i = try_inc(i, nrows - 1);

    let start_j = try_dec(j, 0);
    let end_j = try_inc(j, ncols - 1);

    for u in start_i..=end_i {
        for v in start_j..=end_j {
            if i == u && j == v {
                continue;
            }

            if !grid[u][v].flashed {
                if grid[u][v].value == 9 {
                    grid[u][v].value = 0;
                    grid[u][v].flashed = true;

                    walk_cell(grid, u, v);
                } else {
                    grid[u][v].value += 1;
                }
            }
        }
    }
}

#[allow(dead_code)]
fn part1(content: String, nsteps: usize) {
    let mut grid: Vec<Vec<Cell>> = content
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| {
                    Cell::new(
                        c.to_digit(10).unwrap() as i32
                    )
                })
                .collect::<Vec<Cell>>()
        })
        .collect();

    print_grid_flashes(&grid);

    let mut flash_count = 0;

    let nrows = grid.len();
    let ncols = grid[0].len();

    for _ in 0..nsteps {
        for i in 0..nrows {
            for j in 0..ncols {
                if !grid[i][j].flashed {
                    if grid[i][j].value == 9 {
                        grid[i][j].value = 0;
                        grid[i][j].flashed = true;

                        walk_cell(&mut grid, i, j);
                    } else {
                        grid[i][j].value += 1;
                    }
                }
            }
        }

        // print_grid_flashes(&grid);
        flash_count += count_and_reset_grid_flashes(&mut grid);
    }

    println!("n# flashes = {}", flash_count);
}

fn is_all_flashed(grid: &Vec<Vec<Cell>>) -> bool {
    for row in grid.iter() {
        for c in row.iter() {
            if !c.flashed {
                return false;
            }
        }
    }

    true
}

fn part2(content: String) {
    let mut grid: Vec<Vec<Cell>> = content
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| {
                    Cell::new(
                        c.to_digit(10).unwrap() as i32
                    )
                })
                .collect::<Vec<Cell>>()
        })
        .collect();

    let mut nsteps = 0;

    let nrows = grid.len();
    let ncols = grid[0].len();

    loop {
        for i in 0..nrows {
            for j in 0..ncols {
                if !grid[i][j].flashed {
                    if grid[i][j].value == 9 {
                        grid[i][j].value = 0;
                        grid[i][j].flashed = true;

                        walk_cell(&mut grid, i, j);
                    } else {
                        grid[i][j].value += 1;
                    }
                }
            }
        }

        nsteps += 1;
        if is_all_flashed(&grid) {
            break;
        }

        count_and_reset_grid_flashes(&mut grid);
    }

    println!("n# steps = {}", nsteps);
}

fn main() {
    let filename = "./inputs/day11.txt";

    let content = fs::read_to_string(filename)
        .expect("Could not read file");


    part2(content);
}
