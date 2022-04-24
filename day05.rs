use std::fs;
use std::mem;

struct Segment<T> {
    x1: T, 
    y1: T, 
    x2: T, 
    y2: T, 
}

impl<T> Segment<T> {
    fn new(x1: T, y1: T, x2: T, y2: T) -> Self {
        Segment {
            x1, y1, x2, y2,
        }
    }
}

fn sig(v: f32) -> f32{
    if v > 0.0 {
        1.0
    } else if v < 0.0 {
        -1.0
    } else {
        0.0
    }
}

fn max3(a: i32, b: i32, c: i32) -> i32 {
    if a > b {
        if a > c {
            a
        } else {
            c
        }
    } else {
        if b > c {
            b
        } else {
            c
        }
    }
}

fn part1(content: String) {
    let mut x_max: i32 = -1;
    let mut y_max: i32 = -1;

    let segments: Vec<Segment<i32>> = content
        .lines()
        .map(|line| {
            let points: Vec<&str> = line
                .split(" -> ")
                .collect();

            let result: Vec<i32> = points[0]
                .split(",")
                .map(|v| {
                    i32::from_str_radix(v, 10).expect("not a number")
                })
                .collect();

            let x1 = result[0];
            let y1 = result[1];

            let result: Vec<i32> = points[1]
                .split(",")
                .map(|v| {
                    i32::from_str_radix(v, 10).expect("not a number")
                })
                .collect();

            let x2 = result[0];
            let y2 = result[1];

            x_max = max3(x_max, x1, x2);
            y_max = max3(y_max, y1, y2);

            Segment::<i32>::new(x1, y1, x2, y2)
        })
        .collect();

    let mut grid = vec![vec![0i32; (x_max + 1) as usize]; (y_max + 1) as usize];

    for seg in segments.iter() {
        if seg.x1 == seg.x2 {
            let mut start = seg.y1 as usize;
            let mut end = seg.y2 as usize;

            if start > end {
                mem::swap(&mut start, &mut end);
            }

            let x = seg.x1 as usize;
            for i in start..=end {
                grid[i][x] += 1;
            }
        }
        else if seg.y1 == seg.y2 {
            let mut start = seg.x1 as usize;
            let mut end = seg.x2 as usize;

            if start > end {
                mem::swap(&mut start, &mut end);
            }

            let y = seg.y1 as usize;
            for i in start..=end {
                grid[y][i] += 1;
            }
        }
    }

    // for row in grid.iter() {
    //     for col in row.iter() {
    //         if *col == 0 {
    //             print!(".");
    //         } else {
    //             print!("{}", *col);
    //         }
    //     }

    //     print!("\n");
    // }

    let mut count = 0;

    for row in grid.iter() {
        for col in row.iter() {
            if *col > 1 {
                count += 1;
            }
        }
    }

    println!("count: {}", count);
}

fn part2(content: String) {
    let mut x_max: i32 = -1;
    let mut y_max: i32 = -1;

    let segments: Vec<Segment<f32>> = content
        .lines()
        .map(|line| {
            let points: Vec<&str> = line
                .split(" -> ")
                .collect();

            let result: Vec<i32> = points[0]
                .split(",")
                .map(|v| {
                    i32::from_str_radix(v, 10).expect("not a number")
                })
                .collect();

            let x1 = result[0];
            let y1 = result[1];

            let result: Vec<i32> = points[1]
                .split(",")
                .map(|v| {
                    i32::from_str_radix(v, 10).expect("not a number")
                })
                .collect();

            let x2 = result[0];
            let y2 = result[1];

            x_max = max3(x_max, x1, x2);
            y_max = max3(y_max, y1, y2);

            Segment::<f32>::new(x1 as f32, y1 as f32, x2 as f32, y2 as f32)
        })
        .collect();

    let mut grid = vec![vec![0i32; (x_max + 1) as usize]; (y_max + 1) as usize];

    for seg in segments.iter() {
        let mut dx = seg.x2 - seg.x1;
        let mut dy = seg.y2 - seg.y1;

        let valid_ratio =
            if dx == 0.0 && dy == 0.0 {
                false
            } else if dx == 0.0 {
                dy = sig(dy);
                true
            } else if dy == 0.0 {
                dx = sig(dx);
                true
            } else {
                let ratio = dy / dx;

                dy = sig(dy);
                dx = sig(dx);

                ratio == 1.0 || ratio == -1.0
            };

        if valid_ratio {
            let mut x = seg.x1;
            let mut y = seg.y1;

            let mut finished = false;

            while !finished {
                grid[y as usize][x as usize] += 1;

                finished = (x == seg.x2) && (y == seg.y2);

                x += dx;
                y += dy;
            }
        }
    }

    /*
    for row in grid.iter() {
        for col in row.iter() {
            if *col == 0 {
                print!(".");
            } else {
                print!("{}", *col);
            }
        }

        print!("\n");
    }
    */

    let mut count = 0;

    for row in grid.iter() {
        for col in row.iter() {
            if *col > 1 {
                count += 1;
            }
        }
    }

    println!("count: {}", count);
}

fn main() {
    let filename = "./inputs/day05.txt";

    let content = fs::read_to_string(filename)
        .expect("Could not read file");

    part2(content);
}

