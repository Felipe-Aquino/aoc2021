use std::fs;

#[derive(Copy, Clone, Debug)]
struct Cell {
    value: u32,
    cost: u32,

    prev: Option<usize>,
    visited: bool,
}

struct Grid {
    data: Vec<Cell>,

    nrows: usize,
    ncols: usize,

    min_path: Vec<(usize, usize)>,
}

impl Grid {
    fn new(nrows: usize, ncols: usize) -> Self {
        let cell = Cell {
            value: 0,
            cost: 1000000u32,
            prev: None,
            visited: false,
        };

        Grid {
            data: vec![cell; ncols * nrows],
            nrows,
            ncols,

            min_path: vec![],
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

    fn set_cost_at(&mut self, i: usize, j: usize, value: u32) {
        if let Some(cell) = self.data.get_mut(i * self.ncols + j) {
            cell.cost = value;
        }
    }

    fn index_to_pos(&self, idx: usize) -> (usize, usize) {
        let i = idx / self.ncols;
        let j = idx - i * self.ncols;

        (i, j)
    }

    fn avaliate_step(&mut self, i: usize, j: usize) -> (usize, usize) {
        let curr = i * self.ncols + j;

        let cell_cost = {
            let cell = self.data.get(curr).unwrap();
            cell.cost
        };

        if i < self.nrows - 1 {
            let neighbor = self.data.get_mut(curr + self.ncols).unwrap();

            if !neighbor.visited {
                if neighbor.cost > cell_cost + neighbor.value {
                    neighbor.cost = cell_cost + neighbor.value;
                    neighbor.prev = Some(curr);
                }
            }
        }

        if j < self.ncols - 1 {
            let neighbor = self.data.get_mut(curr + 1).unwrap();

            if !neighbor.visited {
                if neighbor.cost > cell_cost + neighbor.value {
                    neighbor.cost = cell_cost + neighbor.value;
                    neighbor.prev = Some(curr);
                }
            }
        }

        if i > 0 {
            let neighbor = self.data.get_mut(curr - self.ncols).unwrap();

            if !neighbor.visited {
                if neighbor.cost > cell_cost + neighbor.value {
                    neighbor.cost = cell_cost + neighbor.value;
                    neighbor.prev = Some(curr);
                }
            }
        }

        if j > 0 {
            let neighbor = self.data.get_mut(curr - 1).unwrap();

            if !neighbor.visited {
                if neighbor.cost > cell_cost + neighbor.value {
                    neighbor.cost = cell_cost + neighbor.value;
                    neighbor.prev = Some(curr);
                }
            }
        }

        {
            let cell = self.data.get_mut(curr).unwrap();
            cell.visited = true;
        }

        let mut min_cost = 100000;
        let mut next_cell = 0;

        for (i, c) in self.data.iter().enumerate() {
            if !c.visited && c.cost < min_cost {
                min_cost = c.cost;
                next_cell = i;
            }
        }

        self.index_to_pos(next_cell)
    }

    fn avaliate_paths(&mut self) {
        self.set_cost_at(0, 0, 0);

        let mut i = 0;
        let mut j = 0;

        loop {
            if i == self.nrows - 1 && j == self.ncols - 1 {

                break;
            }

            let n = self.avaliate_step(i, j);

            i = n.0;
            j = n.1;
        }

        i = self.nrows - 1;
        j = self.ncols - 1;

        loop {
            self.min_path.push((i, j));

            let cell = self.data.get(i * self.ncols + j).unwrap();

            if let Some(idx) = cell.prev {
                let (ni, nj) = self.index_to_pos(idx);
                i = ni;
                j = nj;

            } else {
                println!("Ended at ({}, {})", i, j);
                break;
            }
        }

        // path end
        println!("final path: {:?}", self.min_path);
    }

    fn print_at(&self, i: usize, j: usize) {
        println!("{:?}", self.data.get(i * self.ncols + j));
    }

    fn print_path(&self) {
        // println!("{:?}", self.min_path);

        for i in 0..self.nrows {
            for j in 0..self.ncols {
                let v = self.get_value_at(i, j);

                if let Some(_) = self.min_path.iter().find(|(pi, pj)| *pi == i && *pj == j) {
                    //print!("\\033[31m{}\\033[39m", v);
                    // print!("\\e[1;96;127m{}\\e[0m", v);
                    print!("\x1B[31m{}\x1B[0m", v);

                } else {
                    print!("{}", v);
                }
            }
            print!("\n");
        }

        print!("\n");


        print!("min cost");
        self.print_at(self.nrows - 1, self.ncols - 1);
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

    grid.avaliate_paths();

    // grid.print_at(0, 0);
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

fn part2(content: String) {
    let lines: Vec<&str> = content
        .lines()
        .collect();

    let nrows = lines.len();
    let ncols = lines[0].len();

    println!("nrows: {}, ncols: {}", nrows, ncols);

    let mut grid = Grid::new(5 * nrows, 5 * ncols);

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let value = ((c as u8) - 48) as u32;

            for si in 0..5 {
                let mut v1 = value + si as u32;

                if v1 > 9 {
                    v1 = v1 - 9
                }

                for sj in 0..5 {
                    let mut v2 = v1 + sj as u32;

                    if v2 > 9 {
                        v2 = v2 - 9
                    }

                    grid.set_value_at(i + si * nrows, j + sj * ncols, v2);
                }
            }
        }
    }

    grid.avaliate_paths();

    print!("min cost");
    grid.print_at(grid.nrows - 1, grid.ncols - 1);
}

fn main() {
    let filename = "./inputs/day15.txt";
    // let filename = "./inputs/day15-example.txt";

    let content = fs::read_to_string(filename)
        .expect("Could not read file.");

    part2(content);
}
