use std::fs;

#[derive(Copy, Clone, Debug)]
struct Range {
    start: i64,
    end: i64,
}

#[derive(Eq, PartialEq, Debug)]
enum Token {
    Ident(String),
    Number(i64),
    Equal,
    DotDot,
    Comma,
}

fn is_alpha(c: u8) -> bool {
    b'a' <= c && c <= b'z' || b'A' <= c && c <= b'Z'
}

fn is_digit(c: u8) -> bool {
    b'0' <= c && c <= b'9'
}

fn tokenize(line: &str) -> Vec<Token> {
    let mut tokens = vec![];

    let chars = line.as_bytes();

    let mut i = 0;

    while i < chars.len() {
        let mut c = chars[i];

        while c == b' ' {
            i += 1;
            c = chars[i];
        }

        let token = match c {
            b'.' => {
                if chars[i + 1] != b'.' {
                    panic!("next char is not a '.'");
                }

                i += 2;

                Token::DotDot
            },
            b',' => {
                i += 1;

                Token::Comma
            },
            b'=' => {
                i += 1;

                Token::Equal
            },
            _ => {
                if is_alpha(c) {
                    let start = i;

                    while i < chars.len() && is_alpha(chars[i]) {
                        i += 1;
                    }

                    Token::Ident(String::from_utf8(chars[start..i].to_vec()).unwrap())
                } else if c == b'-' || is_digit(c) {
                    let start = i;

                    if c == b'-' {
                        if i + 1 >= chars.len() || !is_digit(chars[i + 1]) {
                            panic!("next char is not a digit");
                        }

                        i += 1;
                    }

                    while i < chars.len() && is_digit(chars[i]) {
                        i += 1;
                    }

                    Token::Number(i64::from_str_radix(&line[start..i], 10).unwrap())
                } else {
                    unreachable!();
                }
            }
        };

        tokens.push(token);
    }

    tokens
}

#[derive(Copy, Clone, Debug)]
struct Cuboid {
    on: bool,
    x_range: Range,
    y_range: Range,
    z_range: Range,
}

fn read_line_cuboid(line: &str) -> Cuboid {
    let mut cuboid = Cuboid {
        on: false,
        x_range: Range{ start: 0, end: 0 },
        y_range: Range{ start: 0, end: 0 },
        z_range: Range{ start: 0, end: 0 },
    };

    let tokens = tokenize(&line);

    // println!("{:?}", tokens);

    if let Token::Ident(state) = &tokens[0] {
        cuboid.on = state == "on";
    } else {
        unreachable!();
    }

    let mut i = 1;

    while i < tokens.len() {
        let axis: &str = 
            if let Token::Ident(state) = &tokens[i] {
                state.as_str()
            } else {
                unreachable!("expecting an ident");
            };

        i += 1;

        if tokens[i] != Token::Equal {
            panic!("expecting an '='");
        }

        i += 1;

        let start: i64 = 
            if let Token::Number(n) = &tokens[i] {
                *n
            } else {
                unreachable!("expecting a number");
            };

        i += 1;

        if tokens[i] != Token::DotDot {
            panic!("expecting a '..'");
        }

        i += 1;

        let end: i64 = 
            if let Token::Number(n) = &tokens[i] {
                *n
            } else {
                unreachable!("expecting a number");
            };

        i += 1;

        if i < tokens.len() {
            if tokens[i] != Token::Comma {
                panic!("expecting a ','");
            }

            i += 1;
        }

        match axis {
            "x" => {
                cuboid.x_range.start = start;
                cuboid.x_range.end = end;
            },
            "y" => {
                cuboid.y_range.start = start;
                cuboid.y_range.end = end;
            },
            "z" => {
                cuboid.z_range.start = start;
                cuboid.z_range.end = end;
            },
            _ => {
                unreachable!("expecting axis to be x, y or z");
            },
        }
    }

    cuboid
}

fn clamp(v: i64, min: i64, max: i64) -> i64 {
    if v < min {
        min
    } else if v > max {
        min
    } else {
        v
    }
}

fn clamp_interval(v1: i64, v2: i64, min: i64, max: i64) -> (i64, i64) {
    if (v1 < min && v2 < min) || (v1 > max && v2 > max) {
        return (0, -1);
    }

    return (clamp(v1, min, max), clamp(v2, min, max));
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum CubeState {
    NotSet,
    On,
    Off,
}

#[allow(dead_code)]
fn part1(content: String) {
    let cuboids: Vec<Cuboid> = content
        .lines()
        .map(|line| {
            read_line_cuboid(&line)
        })
        .collect();

    // println!("{:?}", cuboids);

    let mut cubes = [[[CubeState::NotSet; 101]; 101]; 101];

    let min_value = -50;
    let max_value = 50;

    for cuboid in cuboids {
        let (xstart, xend) = clamp_interval(
            cuboid.x_range.start,
            cuboid.x_range.end,
            min_value,
            max_value
        );

        for x in xstart..=xend {
            let (ystart, yend) = clamp_interval(
                cuboid.y_range.start,
                cuboid.y_range.end,
                min_value,
                max_value
            );

            let i = (x - min_value) as usize;

            for y in ystart..=yend {
                let (zstart, zend) = clamp_interval(
                    cuboid.z_range.start,
                    cuboid.z_range.end,
                    min_value,
                    max_value
                );

                let j = (y - min_value) as usize;

                for z in zstart..=zend {
                    let k = (z - min_value) as usize;

                    if cuboid.on {
                        cubes[i][j][k] = CubeState::On;
                    } else {
                        cubes[i][j][k] = CubeState::Off;
                    }
                }
            }
        }
    }

    let mut not_set_count = 0;
    let mut on_count = 0;
    let mut off_count = 0;

    for i in 0..101 {
        for j in 0..101 {
            for k in 0..101 {
                if cubes[i][j][k] == CubeState::On {
                    on_count += 1;
                } else if cubes[i][j][k] == CubeState::Off {
                    off_count += 1;
                } else {
                    not_set_count += 1;
                }
            }
        }
    }

    println!("on:      {}", on_count);
    println!("off:     {}", off_count);
    println!("not set: {}", not_set_count);
}

fn min(a: i64, b: i64) -> i64 {
    if a > b {
        b
    } else {
        a
    }
}

fn max(a: i64, b: i64) -> i64 {
    if a > b {
        a
    } else {
        b
    }
}

fn cuboid_cube_count(c: &Cuboid) -> i64 {
    return (c.x_range.end - c.x_range.start + 1) *
           (c.y_range.end - c.y_range.start + 1) *
           (c.z_range.end - c.z_range.start + 1);
}

// c2 must be contained in c1
fn cuboids_sub(c1: &Cuboid, c2: &Cuboid) -> Vec<Cuboid> {
    let mut subcuboids = vec![];

    /*
    +--------------------+
    |                    |
    |                    |
    +---------+***+------+
    |         |   |      |
    +---------+***+------+
    |                    |
    +--------------------+
    */

    let top = Cuboid {
        on: false,
        x_range: c1.x_range,
        y_range: c1.y_range,
        z_range: Range {
            start: c1.z_range.start,
            end: c2.z_range.start - 1,
        },
    };

    if cuboid_cube_count(&top) > 0 {
        subcuboids.push(top);
    }

    let bottom = Cuboid {
        on: false,
        x_range: c1.x_range,
        y_range: c1.y_range,
        z_range: Range {
            start: c2.z_range.end + 1,
            end: c1.z_range.end,
        },
    };

    if cuboid_cube_count(&bottom) > 0 {
        subcuboids.push(bottom);
    }

    let left = Cuboid {
        on: false,
        x_range: Range {
            start: c1.x_range.start,
            end: c2.x_range.start - 1,
        },
        y_range: c1.y_range,
        z_range: c2.z_range,
    };

    if cuboid_cube_count(&left) > 0 {
        subcuboids.push(left);
    }

    let right = Cuboid {
        on: false,
        x_range: Range {
            start: c2.x_range.end + 1,
            end: c1.x_range.end,
        },
        y_range: c1.y_range,
        z_range: c2.z_range,
    };

    if cuboid_cube_count(&right) > 0 {
        subcuboids.push(right);
    }

    let back = Cuboid {
        on: false,
        x_range: c2.x_range,
        y_range: Range {
            start: c1.y_range.start,
            end: c2.y_range.start - 1,
        },
        z_range: c2.z_range,
    };

    if cuboid_cube_count(&back) > 0 {
        subcuboids.push(back);
    }

    let front = Cuboid {
        on: false,
        x_range: c2.x_range,
        y_range: Range {
            start: c2.y_range.end + 1,
            end: c1.y_range.end,
        },
        z_range: c2.z_range,
    };

    if cuboid_cube_count(&front) > 0 {
        subcuboids.push(front);
    }

    return subcuboids;
}

fn cuboids_intersect(c1: &Cuboid, c2: &Cuboid) -> Option<Cuboid> {
    let xmin = max(c1.x_range.start, c2.x_range.start);
    let xmax = min(c1.x_range.end, c2.x_range.end);

    if xmin > xmax {
        return None;
    }

    let ymin = max(c1.y_range.start, c2.y_range.start);
    let ymax = min(c1.y_range.end, c2.y_range.end);

    if ymin > ymax {
        return None;
    }

    let zmin = max(c1.z_range.start, c2.z_range.start);
    let zmax = min(c1.z_range.end, c2.z_range.end);

    if zmin > zmax {
        return None;
    }

    let r = Cuboid {
        on: false,
        x_range: Range{ start: xmin, end: xmax },
        y_range: Range{ start: ymin, end: ymax },
        z_range: Range{ start: zmin, end: zmax },
    };

    Some(r)
}

fn part2(content: String) {
    let cuboids: Vec<Cuboid> = content
        .lines()
        .map(|line| {
            read_line_cuboid(&line)
        })
        .collect();

    let mut subcuboids = vec![];

    let first = Cuboid {
        on: cuboids[0].on,
        x_range: cuboids[0].x_range,
        y_range: cuboids[0].y_range,
        z_range: cuboids[0].z_range
    };

    subcuboids.push(first);

    for i in 1..cuboids.len() {
        for j in (0..subcuboids.len()).rev() {
            if let Some(c) = cuboids_intersect(&cuboids[i], &subcuboids[j]) {
                let new_cuboids = cuboids_sub(&subcuboids[j], &c);

                subcuboids.remove(j);
                subcuboids.extend(new_cuboids);
            }
        }

        if cuboids[i].on {
            subcuboids.push(cuboids[i]);
        }
    }

    let mut count = 0;

    for c in subcuboids.iter() {
        // println!("{:?}: {}", c, cuboid_cube_count(&c));
        count += cuboid_cube_count(&c);
    }

    println!("count: {}", count);
}

fn main() {
    // let filename = "inputs/day22-example2.txt";
    let filename = "inputs/day22.txt";

    let data = fs::read_to_string(filename)
        .expect("Could read file");

    // part1(data);
    part2(data);
}

