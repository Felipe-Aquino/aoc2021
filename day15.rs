use std::fs;

#[derive(Copy, Clone, Debug)]
enum Dir {
    Down,
    Right,
}

#[derive(Copy, Clone, Debug)]
struct Cell {
    value: u32,

    lower_path_cost: u32,
    lower_path_dir: Dir,
}

struct Grid {
    data: Vec<Cell>,

    nrows: usize,
    ncols: usize,
}

impl Grid {
    fn new(nrows: usize, ncols: usize) -> Self {
        let cell = Cell {
            value: 0,
            lower_path_cost: 1e6 as u32,
            lower_path_dir: Dir::Right,
        };

        Grid {
            data: vec![cell; ncols * nrows],
            nrows,
            ncols,
        }
    }

    fn get_value_at(&self, i: usize, j: usize) -> u32 {
        let cell = self.data.get(i * self.ncols + j).unwrap();

        cell.value
    }

    fn set_value_at(&mut self, i: usize, j: usize, value: u32) {
        if let Some(cell) = self.data.get_mut(i * self.ncols + j) {
            cell.value = value;
        }
    }

    fn update_costs(&mut self) {
        let mut row_idx: i32 = self.nrows as i32 - 1;
        let mut col_idx: i32 = self.ncols as i32 - 1;

        {
            let cell = self.data.last_mut().unwrap();
            cell.lower_path_cost = cell.value;
        }

        while row_idx > -1 || col_idx > -1 {
            if row_idx > -1 {
                for j in (0..self.ncols).rev() {
                    let pos = (row_idx as usize) * self.nrows + j;

                    let (mut lower_cost, mut lower_dir) = {
                        let cell = self.data.get(pos).unwrap();

                        (cell.lower_path_cost, cell.lower_path_dir)
                    };

                    let mut changed = false;

                    if j < self.ncols - 1 {
                        let right_cell = self.data.get(pos + 1).unwrap();

                        if right_cell.lower_path_cost < lower_cost {
                            lower_cost = right_cell.lower_path_cost;
                            lower_dir = Dir::Right;

                            changed = true;
                        }
                    }

                    if row_idx < (self.nrows - 1) as i32 {
                        let down_cell = self.data.get(pos + self.ncols).unwrap();

                        if down_cell.lower_path_cost < lower_cost {
                            lower_cost = down_cell.lower_path_cost;
                            lower_dir = Dir::Down;

                            changed = true;
                        }
                    }

                    if changed {
                        let cell = self.data.get_mut(pos).unwrap();

                        cell.lower_path_cost = cell.value + lower_cost;
                        cell.lower_path_dir = lower_dir;
                    }
                }

                row_idx -= 1;
            }

            if col_idx > -1 {
                for i in (0..self.nrows).rev() {
                    let pos = i * self.nrows + col_idx as usize;

                    let (mut lower_cost, mut lower_dir) = {
                        let cell = self.data.get(pos).unwrap();

                        (cell.lower_path_cost, cell.lower_path_dir)
                    };

                    let mut changed = false;

                    if col_idx < (self.ncols - 1) as i32 {
                        let right_cell = self.data.get(pos + 1).unwrap();

                        if right_cell.lower_path_cost < lower_cost {
                            lower_cost = right_cell.lower_path_cost;
                            lower_dir = Dir::Right;

                            changed = true;
                        }
                    }

                    if i < self.nrows - 1 {
                        let down_cell = self.data.get(pos + self.ncols).unwrap();

                        if down_cell.lower_path_cost < lower_cost {
                            lower_cost = down_cell.lower_path_cost;
                            lower_dir = Dir::Down;

                            changed = true;
                        }
                    }

                    if changed {
                        let cell = self.data.get_mut(pos).unwrap();

                        cell.lower_path_cost = cell.value + lower_cost;
                        cell.lower_path_dir = lower_dir;
                    }
                }

                col_idx -= 1;
            }
        }
    }

    fn print_at(&self, i: usize, j: usize) {
        println!("{:?}", self.data.get(i * self.ncols + j));
    }

    fn print_path(&self) {
        let mut path: Vec<(usize, usize)> = vec![];

        let mut i = 0;
        let mut j = 0;

        while i < self.nrows && j < self.ncols {
            let cell = self.data.get(i * self.ncols + j).unwrap();

            path.push((i, j));
            match cell.lower_path_dir {
                Dir::Right => j += 1,
                Dir::Down => i += 1,
            }
        }

        println!("{:?}", path);

        for i in 0..self.nrows {
            for j in 0..self.ncols {
                let v = self.get_value_at(i, j);

                if let Some(_) = path.iter().find(|(pi, pj)| *pi == i && *pj == j) {
                    //print!("\\033[31m{}\\033[39m", v);
                    // print!("\\e[1;96;127m{}\\e[0m", v);
                    print!("\x1B[31m{}\x1B[0m", v);

                } else {
                    print!("{}", v);
                }
            }
            print!("\n");
        }
    }
}

fn part1(content: String) {
    let lines: Vec<&str> = content
        .lines()
        .collect();

    let nrows = lines.len();
    let ncols = lines[0].len();

    println!("nrows: {}, ncols: {}", nrows, ncols);

    let mut grid = Grid::new(nrows, ncols);

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let value = ((c as u8) - 48) as u32;

            grid.set_value_at(i, j, value);
        }
    }

    grid.update_costs();

    grid.print_at(0, 0);
    // grid.print_at(0, 1);
    // grid.print_at(0, 2);

    // grid.print_at(1, 0);
    // grid.print_at(1, 1);
    // grid.print_at(1, 2);

    // grid.print_at(2, 0);
    // grid.print_at(2, 1);
    // grid.print_at(2, 2);
    //
    grid.print_path();
}

fn main() {
    let filename = "./inputs/day15.txt";
    // let filename = "./inputs/day15-example.txt";

    let content = fs::read_to_string(filename)
        .expect("Could not read file.");

    part1(content);
}
