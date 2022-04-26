use std::fs;
use std::collections::HashSet;

#[derive(Debug)]
struct Area {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Area {
    fn x_comp(&self, x: i32) -> i32 {
        if x < self.min_x {
            return -1;
        }

        if x > self.max_x {
            return 1;
        }

        return 0;
    }

    fn y_comp(&self, y: i32) -> i32 {
        if y < self.min_y {
            return -1;
        }

        if y > self.max_y {
            return 1;
        }

        return 0;
    }
}

fn abs(v: i32) -> i32 {
    if v > 0 {
        return v;
    }

    return -v;
}

fn extract_range(data: &str) -> (i32, i32) {
    let splited: Vec<i32> = data
        .to_string()
        .drain(2..)
        .collect::<String>()
        .split("..")
        .map(|n| {
            i32::from_str_radix(n, 10)
                .unwrap()
        })
        .collect();

    (splited[0], splited[1])
}

fn part1(mut content: String) {
    let area = {
        let area_str = content
            .drain(13..content.len() - 1)
            .collect::<String>();

        // println!("area: {}", area_str);

        let splited: Vec<&str> = area_str
            .split(", ")
            .collect();

        let xrange = extract_range(splited[0]);
        let yrange = extract_range(splited[1]);

        Area {
            min_x: xrange.0,
            max_x: xrange.1,
            min_y: yrange.0,
            max_y: yrange.1,
        }
    };

    println!("area: {:?}", area);

    assert!(area.max_x >= 0);

    let mut velocities: HashSet<(i32, i32)> = HashSet::new();

    // x(s) = s * v0x - s * (s - 1) / 2
    // vx(s) = v0x - s
    // y(s) = s * v0y - s * (s - 1) / 2
    // vy(s) = v0y - s
    //
    // y'(s) = v0y - (2s - 1) / 2
    //
    // v0y - (2s - 1) / 2 = 0
    // smax = (2 * v0y + 1) / 2 ~ smax = v0y
    // ymax = v0y * v0y - v0y * (v0y - 1) / 2
    // ymax = V0y * (v0y + 1) / 2

    let mut result: (i32, i32) = (0, 0);
    let mut max_y = i32::MIN;

    let v0x_max = area.max_x;

    for v0x in 1..=v0x_max {
        let mut step = 1;

        while step <= v0x {
            let vx = v0x - step;
            let x = step * v0x - step * (step - 1) / 2; 

            let xcmp = area.x_comp(x);

            if xcmp == 0 {
                // println!("x: {}, step: {}, v0x: {}, vx: {}", x, step, v0x, vx);

                if vx > 0 {
                    let mut y0 = area.min_y;

                    while y0 <= area.max_y {
                        let v0y = (2 * y0 + step * (step - 1)) / (2 * step);

                        let step_ymax = (2 * v0y + 1) / 2;

                        let exact_div = (2 * y0 + step * (step - 1)) % (2 * step) == 0;

                        if exact_div {
                            velocities.insert((v0x, v0y));
                        }

                        if exact_div && step_ymax > 0 && step_ymax <= step {
                            let new_max_y = v0y * (v0y + 1) / 2;
                            // println!("  y0: {}, v0y: {}, new max: {}", y0, v0y, new_max_y);

                            if new_max_y > max_y {
                                max_y = new_max_y;
                                result = (v0x, v0y);
                            }
                        }

                        y0 += 1;
                    }
                } else {
                    let mut sstep = step;

                    'inner: loop {
                        let mut y0 = area.min_y;

                        while y0 <= area.max_y {
                            let v0y = (2 * y0 + sstep * (sstep - 1)) / (2 * sstep);

                            let step_ymax = (2 * v0y + 1) / 2;

                            let exact_div = (2 * y0 + sstep * (sstep - 1)) % (2 * sstep) == 0;

                            if exact_div {
                                velocities.insert((v0x, v0y));
                            }

                            if exact_div && step_ymax > 0 && step_ymax <= sstep {
                                let new_max_y: i32 = v0y * (v0y + 1) / 2;
                                // println!("  *y0: {}, sstep: {}, v0y: {}, max: {}", y0, sstep, v0y, new_max_y);

                                if new_max_y > max_y {
                                    max_y = new_max_y;
                                    result = (v0x, v0y);
                                }

                            }

                            // a good point to stop stepping when sstep is 5 times y0
                            if abs(sstep / y0) == 5 {
                                break 'inner;
                            }

                            y0 += 1;
                        }

                        sstep += 1;
                    }
                }

            } else if xcmp > 0 {
                break;
            }

            step += 1;
        }
    }

    println!("result: {:?}, {}", result, max_y);
    println!("count: {}", velocities.len());
}

fn main() {
    // let filename = "./inputs/day17-example.txt";
    let filename = "./inputs/day17.txt";

    let content = fs::read_to_string(filename)
        .expect("Could not read file.");

    part1(content);
}
