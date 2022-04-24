use std::fs;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Completed {
    Row(usize, i32),
    Column(usize, i32),
    Nothing,
}

struct Board {
    values: [[i32; 5]; 5],
    marked: [[bool; 5]; 5],
    completed: Completed,
}

impl Board {
    fn new(values: [[i32; 5]; 5]) -> Board {
        let marked = [[false; 5]; 5];

        Board {
            values,
            marked,
            completed: Completed::Nothing,
        }
    }

    fn mark(&mut self, n: i32) -> Completed {
        if self.completed != Completed::Nothing {
            return self.completed;
        }

        let mut found = false;

        'outer: for i in 0..5 {
            for j in 0..5 {
                if self.values[i][j] == n {
                    self.marked[i][j] = true;
                    found = true;
                    break 'outer;
                }
            }
        }

        if found {
            for i in 0..5 {
                let mut col_finished = true;
                let mut row_finished = true;

                for j in 0..5 {
                    row_finished = row_finished && self.marked[i][j];
                    col_finished = col_finished && self.marked[j][i];
                }

                if row_finished {
                    self.completed = Completed::Row(i, n);
                    return self.completed;
                }

                if col_finished {
                    self.completed = Completed::Column(i, n);
                    return self.completed;
                }
            }
        }

        Completed::Nothing
    }

    pub fn sum_completed(&self, c: Completed) -> i32 {
        let mut sum = 0i32;

        match c {
            Completed::Nothing => sum = 0,
            Completed::Column(col, _) => {
                for i in 0..5 {
                    sum += self.values[i][col];
                }
            },
            Completed::Row(row, _) => {
                for i in 0..5 {
                    println!("{}: {}", i, self.values[row][i]);
                    sum += self.values[row][i];
                }
            },
        }

        sum
    }

    fn sum_umarked(&self) -> i32 {
        let mut sum = 0;

        for i in 0..5 {
            for j in 0..5 {
                if !self.marked[i][j] {
                    sum += self.values[i][j];
                }
            }
        }

        sum
    }
}

fn _part1(content: String) {
    let mut data: Vec<&str> = content.split_terminator("\n\n").collect();

    let numbers: Vec<i32> = data[0]
        .split(",")
        .map(|v| {
            i32::from_str_radix(v, 10)
                .expect("Not a number")
        })
        .collect();

    let mut boards: Vec<Board> = data
        .drain(1..)
        .map(|board_str| {
            let mut values = [[0i32; 5]; 5];

            for (i, row_str) in board_str.split("\n").enumerate() {
                for (j, num_str) in row_str.split_whitespace().enumerate() {
                    let num = i32::from_str_radix(num_str, 10)
                        .expect("Not a number");

                    values[i][j] = num;
                }
            }

            Board::new(values)
        })
        .collect();

    'marking: for &n in numbers.iter() {
        for (i, b) in boards.iter_mut().enumerate() {
            let completed = b.mark(n);

            if completed != Completed::Nothing {
                let sum = boards[i].sum_umarked();

                println!("result: {}", sum * n);
                break 'marking;
            }
        }
    }
}

fn part2(content: String) {
    let mut data: Vec<&str> = content.split_terminator("\n\n").collect();

    let numbers: Vec<i32> = data[0]
        .split(",")
        .map(|v| {
            i32::from_str_radix(v, 10)
                .expect("Not a number")
        })
        .collect();

    let mut boards: Vec<Board> = data
        .drain(1..)
        .map(|board_str| {
            let mut values = [[0i32; 5]; 5];

            for (i, row_str) in board_str.split("\n").enumerate() {
                for (j, num_str) in row_str.split_whitespace().enumerate() {
                    let num = i32::from_str_radix(num_str, 10)
                        .expect("Not a number");

                    values[i][j] = num;
                }
            }

            Board::new(values)
        })
        .collect();

    let mut win_count = 0;

    for &n in numbers.iter() {
        for (i, b) in boards.iter_mut().enumerate() {
            if b.completed == Completed::Nothing {
                if b.mark(n) != Completed::Nothing {
                    let sum = b.sum_umarked();

                    println!("result for {}: {}", i, sum * n);
                    win_count += 1;
                }
            }
        }

        if win_count == boards.len() {
            break;
        }
    }
}

fn main() {
    let filename = "./inputs/day04.txt";

    let content = fs::read_to_string(filename)
        .expect("Could not read file");

    part2(content);
}
