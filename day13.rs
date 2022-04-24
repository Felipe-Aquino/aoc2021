use std::fs;

struct Point {
    x: usize,
    y: usize,
}

enum FoldType {
    Vertical(usize),
    Horizontal(usize),
}

#[allow(dead_code)]
fn print_points(points: &Vec<Point>, max_point: &Point) {
    let mut mat: Vec<Vec<char>> = vec![];

    for _i in 0..=max_point.y {
        let mut line: Vec<char> = vec![];

        for _j in 0..=max_point.x {
            line.push('.');
        }

        mat.push(line);
    }

    // println!("max:  {}, {}", max_point.x, max_point.y);
    for p in points.iter() {
        // println!("{}, {}", p.x, p.y);
        mat[p.y][p.x] = '#';
    }

    for i in 0..max_point.y {
        for j in 0..max_point.x {
            print!("{}", mat[i][j]);
        }
        print!("\n");
    }

}

fn remove_duplicated(points: &mut Vec<Point>) {
    let mut dups: Vec<usize> = vec![];

    for (i, p1) in points.iter().enumerate() {
        let mut found = false;
        let mut last_idx = 0;

        for j in (i + 1)..points.len() {
            if p1.x == points[j].x && p1.y == points[j].y {
                found = true;
                last_idx = j;
                break;
            }
        }
        
        if found {
            dups.push(last_idx);
        }
    }

    // println!("dups {:?}, {}", dups, points.len());

    dups.sort();

    for &i in dups.iter().rev() {
        points.remove(i);
    }
}

fn y_fold(points: &mut Vec<Point>, y: usize, max_point: &mut Point) {
    let mid_y = max_point.y / 2;

    if y >= mid_y {
        for p in points.iter_mut() {
            if p.y > y {
                p.y = 2 * y - p.y;
            }
        }

        max_point.y = y;
    } else {
        let inv_y = max_point.y - y;

        for p in points.iter_mut() {
            p.y = max_point.y - p.y;

            if p.y > inv_y {
                p.y = 2 * inv_y - p.y;
            }
        }

        max_point.y = inv_y;
    }

    remove_duplicated(points);
}

fn x_fold(points: &mut Vec<Point>, x: usize, max_point: &mut Point) {
    let mid_x = max_point.x / 2;

    // println!("@@ {}, {}, {}", x, mid_x, max_point.x);
    if x >= mid_x {
        for p in points.iter_mut() {
            if p.x > x {
                p.x = 2 * x - p.x;
            }
        }

        max_point.x = x;
    } else {
        let inv_x = max_point.x - x;

        for p in points.iter_mut() {
            p.x = max_point.x - p.x;

            if p.x > inv_x {
                p.x = 2 * inv_x - p.x;
            }
        }

        max_point.x = inv_x;
    }

    remove_duplicated(points);
}

fn part1(content: String) {
    let lines = content
        .lines()
        .collect::<Vec<&str>>();

    let empty_line_pos = lines
        .iter()
        .position(|v| v.is_empty())
        .expect("Empty line not found.");

    // let mut remaining_lines = lines.drain(..empty_line_pos);

    let mut points: Vec<Point> = lines
        .iter()
        .enumerate()
        .filter(|&(i, _)| i < empty_line_pos)
        .map(|(_, line)| {
            let splited = line
                .split(',')
                .collect::<Vec<&str>>();

            let x = usize::from_str_radix(splited[0], 10)
                .expect("Not a valid number");

            let y = usize::from_str_radix(splited[1], 10)
                .expect("Not a valid number");

            Point {
                x, y
            }
        })
        .collect();

    let folds: Vec<FoldType> = lines
        .iter()
        .enumerate()
        .filter(|&(i, _)| i > empty_line_pos)
        .map(|(_, line)| {
            if line.starts_with("fold along x=") {
                let value = line.strip_prefix("fold along x=")
                    .expect("Wrong prefix");

                let x = usize::from_str_radix(value, 10)
                    .expect("Not a valid number");

                FoldType::Vertical(x)
            } else {
                let value = line.strip_prefix("fold along y=")
                    .expect("Wrong prefix");

                let y = usize::from_str_radix(value, 10)
                    .expect("Not a valid number");

                FoldType::Horizontal(y)
            }
        })
        .collect();

    let mut max_point = Point { x: 0, y: 0 };

    for p in points.iter() {
        if p.x > max_point.x {
            max_point.x = p.x
        }

        if p.y > max_point.y {
            max_point.y = p.y
        }
    }

    // print_points(&points, &max_point);

    for fold in folds {
        match fold {
            FoldType::Horizontal(y) => {
                println!("y fold in {}", y);
                y_fold(&mut points, y, &mut max_point);
            },
            FoldType::Vertical(x) => {
                println!("x fold in {}", x);
                x_fold(&mut points, x, &mut max_point);
            },
        }

        // println!("\n*************\n");

        // print_points(&points, &max_point);
    }


    println!("\n*************\n");
    print_points(&points, &max_point);
    println!("\ncount: {}", points.len());
}

fn main() {
    // let filename = "./inputs/day13.txt";
    let filename = "./inputs/day13.txt";

    let content = fs::read_to_string(filename)
        .expect("Could not read file");

    part1(content);
}
